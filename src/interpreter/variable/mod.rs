mod axiom;
mod unit;

pub use axiom::Axiom;
pub use unit::Unit;

use crate::{interpreter::scope::TowerScope, parser::Item};

pub type IVariable<'a> = Item<'a, Variable<'a>>;

#[derive(Debug, Clone)]
pub enum Variable<'a> {
    Unit(Unit<'a>),
    Axiom(Axiom<'a>),
}

impl<'a> Variable<'a> {
    // fn as_unit(&self) -> &Unit<'a> {
    //     match self {
    //         Self::Unit(unit) => unit,
    //         Self::Axiom(axiom) => axiom.as_unit(),
    //     }
    // }

    fn simplify(&self, scopes: &TowerScope<'a>) -> &Unit<'a> {
        match self {
            Self::Unit(unit) => unit.simplify(scopes),
            Self::Axiom(axiom) => axiom.as_unit(),
        }
    }

    // pub fn eq(&'a self, other: &'a Self, scopes: &'a TowerScope<'a>) -> bool {
    //     match (self, other) {
    //         (Self::Axiom(axiom1), Self::Axiom(axiom2)) => axiom1 == axiom2,
    //         (Variable::Unit(unit), Variable::Axiom(axiom))
    //         | (Variable::Axiom(axiom), Variable::Unit(unit)) => {
    //             let unit = unit.simplify(scopes);
    //             let axiom_in_unit = unit
    //                 .top()
    //                 .get(axiom.ident())
    //                 .filter(|power| **power == 1)
    //                 .is_some();
    //             let unit_len_of_1 = unit.top().len() == 1;
    //             let unit_no_neg = unit.bottom().is_empty();
    //             axiom_in_unit && unit_len_of_1 && unit_no_neg
    //         }
    //         (Variable::Unit(unit1), Variable::Unit(unit2)) => {
    //             unit1.eq(unit2, scopes)
    //         }
    //     }
    // }
}
