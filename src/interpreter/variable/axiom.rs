use std::collections::HashMap;

use super::Unit;

#[derive(Debug, Clone, PartialEq)]
pub struct Axiom<'a> {
    ident: &'a str,
    symbole: &'a str,
}

impl<'a> Axiom<'a> {
    pub fn new(ident: &'a str, symbole: &'a str) -> Self {
        Self { ident, symbole }
    }

    pub fn into_unit(self) -> Unit<'a> {
        let mut top = HashMap::new();
        top.insert(self.ident, 1);

        Unit::new(top, HashMap::new())
    }

    pub fn symbole(&self) -> &str {
        self.symbole
    }
}
