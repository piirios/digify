use std::{cell::OnceCell, collections::HashMap};

use super::Unit;

#[derive(Debug, Clone)]
pub struct Axiom<'a> {
    ident: &'a str,
    symbole: &'a str,
    unit: OnceCell<Unit<'a>>,
}

impl<'a> Axiom<'a> {
    pub fn new(ident: &'a str, symbole: &'a str) -> Self {
        Self {
            ident,
            symbole,
            unit: OnceCell::new(),
        }
    }

    pub fn symbole(&self) -> &str {
        self.symbole
    }

    pub fn as_unit(&self) -> &Unit<'a> {
        self.unit.get_or_init(|| {
            let mut top = HashMap::new();
            top.insert(self.ident, 1);

            Unit::new(top, HashMap::new())
        })
    }
}

impl PartialEq for Axiom<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.symbole == other.symbole && self.ident == other.ident
    }
}
