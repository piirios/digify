use std::cell::OnceCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops;

use color_eyre::eyre::Result;

use crate::interpreter::scope::TowerScope;
use crate::interpreter::variable::Variable;
use crate::parser::Expr;

#[derive(Debug, Clone, Default)]
pub struct Unit<'a> {
    top: HashMap<&'a str, u32>,
    bottom: HashMap<&'a str, u32>,
    simplify: Box<OnceCell<Unit<'a>>>,
}

impl<'a> Unit<'a> {
    pub fn new(top: HashMap<&'a str, u32>, bottom: HashMap<&'a str, u32>) -> Self {
        Self {
            top,
            bottom,
            simplify: Box::new(OnceCell::new()),
        }
    }

    pub fn top(&self) -> &HashMap<&'a str, u32> {
        &self.top
    }

    pub fn bottom(&self) -> &HashMap<&'a str, u32> {
        &self.bottom
    }

    pub fn simplify(&self, scopes: &TowerScope<'a>) -> &Unit<'a> {
        self.simplify.get_or_init(|| {
            let top = self
                .top
                .iter()
                .map(|(ident, power)| {
                    scopes
                        .get(ident)
                        .unwrap()
                        .simplify(scopes)
                        .clone()
                        .power(*power)
                })
                .reduce(|acc, variable| acc * variable)
                .unwrap_or_default();

            let bottom = self
                .bottom
                .iter()
                .map(|(ident, power)| {
                    scopes
                        .get(ident)
                        .unwrap()
                        .simplify(scopes)
                        .clone()
                        .power(*power)
                })
                .reduce(|acc, variable| acc * variable)
                .unwrap_or_default();

            top / bottom
        })
    }

    fn power(mut self, power: u32) -> Self {
        self.top.values_mut().for_each(|value| *value *= power);
        self.bottom.values_mut().for_each(|value| *value *= power);
        self
    }

    pub fn eq(&self, other: &Unit<'a>, scopes: &TowerScope<'a>) -> bool {
        let unit1 = self.simplify(scopes);
        let unit2 = other.simplify(scopes);

        unit1.top() == unit2.top() && unit1.bottom() == unit2.bottom()
    }

    pub fn to_string(&self, scopes: &TowerScope<'a>) -> String {
        let mut pos_ident = self.top.keys().copied().collect::<Vec<_>>();
        let mut neg_ident = self.bottom.keys().copied().collect::<Vec<_>>();
        pos_ident.sort_unstable();
        neg_ident.sort_unstable();

        if pos_ident.is_empty() && neg_ident.is_empty() {
            return "1".to_string();
        }

        if pos_ident.len() == 1 && neg_ident.is_empty() {
            let variable = scopes.get(pos_ident[0]).unwrap();

            match variable {
                Variable::Axiom(axiom) => return axiom.symbole().to_string(),
                Variable::Unit(unit) => return unit.to_string(scopes),
            }
        }

        let mut string = String::new();

        for ident in pos_ident {
            let power = self.top[ident];
            if power == 1 {
                string.push_str(ident);
                string.push(' ');
            } else {
                string.push_str(ident);
                string.push('^');
                string.push_str(&power.to_string());
                string.push(' ');
            }
        }

        for ident in neg_ident {
            let power = self.bottom[ident];
            if power == 1 {
                string.push_str(ident);
                string.push_str("-1 ");
            } else {
                string.push_str(ident);
                string.push_str("^-");
                string.push_str(&power.to_string());
                string.push(' ');
            }
        }

        string
    }

    pub fn print(&self, scopes: &TowerScope<'a>) {
        println!("{}", self.to_string(scopes));
    }
}

impl<'a> Unit<'a> {
    pub fn from(value: Expr<'a>, scopes: &TowerScope<'a>) -> Result<Self> {
        fn insert_in_frac<'b>(
            top: &mut HashMap<&'b str, u32>,
            bottom: &mut HashMap<&'b str, u32>,
            expr: Expr<'b>,
            power: u32,
            simplify: bool,
            scopes: &TowerScope<'b>,
        ) -> Result<()> {
            match expr {
                Expr::Ident(ident) => {
                    if simplify {
                        let variable = scopes.get(ident)?.simplify(scopes);

                        for (ident, power) in variable.top() {
                            *top.entry(ident).or_default() += power;
                        }
                        for (ident, power) in variable.bottom() {
                            *bottom.entry(ident).or_default() += power;
                        }
                    } else {
                        *top.entry(ident).or_default() += power
                    }
                }
                Expr::Mul(expr1, expr2) => {
                    insert_in_frac(top, bottom, *expr1, power, simplify, scopes)?;
                    insert_in_frac(top, bottom, *expr2, power, simplify, scopes)?;
                }
                Expr::Div(expr1, expr2) => {
                    insert_in_frac(top, bottom, *expr1, power, simplify, scopes)?;
                    insert_in_frac(bottom, top, *expr2, power, simplify, scopes)?;
                }
                Expr::Power(expr, number) => match number.cmp(&0) {
                    Ordering::Less => insert_in_frac(
                        bottom,
                        top,
                        *expr,
                        power * -number as u32,
                        simplify,
                        scopes,
                    )?,
                    Ordering::Greater => {
                        insert_in_frac(top, bottom, *expr, power * number as u32, simplify, scopes)?
                    }
                    Ordering::Equal => (),
                },
                Expr::Simplify(expr) => {
                    insert_in_frac(top, bottom, *expr, power, true, scopes)?
                }
                Expr::None => (),
            }

            Ok(())
        }

        let mut top = HashMap::new();
        let mut bottom = HashMap::new();

        insert_in_frac(&mut top, &mut bottom, value, 1, false, scopes)?;
        let bottom_idents = bottom.keys().copied().collect::<Vec<_>>();

        for ident in bottom_idents {
            let mut is_cancel = false;
            if let Some(pos_power) = top.get_mut(ident) {
                match bottom[ident].cmp(pos_power) {
                    Ordering::Equal => {
                        is_cancel = true;
                    }
                    Ordering::Greater => {
                        is_cancel = true;
                        *bottom.get_mut(ident).unwrap() -= *pos_power;
                    }
                    Ordering::Less => {
                        *pos_power -= bottom[ident];
                        bottom.remove(ident);
                    }
                }
            }

            if is_cancel {
                top.remove(ident);
                bottom.remove(ident);
            }
        }

        Ok(Self {
            top,
            bottom,
            simplify: Box::new(OnceCell::new()),
        })
    }
}

impl<'a> ops::Mul for Unit<'a> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.simplify = Box::new(OnceCell::new());

        // Insert rhs.top values inside self.top
        for (ident, pos_power) in rhs.top {
            let mut is_cancel = false;
            if let Some(neg_power) = self.bottom.get_mut(ident) {
                match pos_power.cmp(neg_power) {
                    Ordering::Equal => {
                        is_cancel = true;
                    }
                    Ordering::Greater => {
                        is_cancel = true;
                        *self.top.entry(ident).or_default() += pos_power - *neg_power;
                    }
                    Ordering::Less => *neg_power -= pos_power,
                }
            } else {
                self.top.insert(ident, pos_power);
            }
            if is_cancel {
                self.bottom.remove(ident);
            }
        }

        // Insert rhs.bottom values inside self.bottom
        for (ident, neg_power) in rhs.bottom {
            let mut is_cancel = false;
            if let Some(pos_power) = self.top.get_mut(ident) {
                match neg_power.cmp(pos_power) {
                    Ordering::Equal => {
                        is_cancel = true;
                    }
                    Ordering::Greater => {
                        is_cancel = true;
                        *self.bottom.entry(ident).or_default() += neg_power - *pos_power;
                    }
                    Ordering::Less => *pos_power -= neg_power,
                }
            } else {
                self.bottom.insert(ident, neg_power);
            }
            if is_cancel {
                self.top.remove(ident);
            }
        }

        self
    }
}

impl<'a> ops::Div for Unit<'a> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        let inveres_rhs = Self {
            top: rhs.bottom,
            bottom: rhs.top,
            simplify: rhs.simplify,
        };

        self * inveres_rhs
    }
}
