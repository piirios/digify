use std::collections::HashMap;

use color_eyre::eyre::{bail, eyre, OptionExt, Result};

use crate::interpreter::variable::{Unit, Variable};

use super::variable::Axiom;

#[derive(Debug, Default)]
pub struct TowerScope<'a> {
    scopes: Vec<Scope<'a>>,
}

#[derive(Debug, Default)]
pub struct Scope<'a> {
    variables: HashMap<&'a str, Variable<'a>>,
}

impl<'a> TowerScope<'a> {
    pub fn define(&mut self, ident: &'a str, symbole: &'a str) -> Result<()> {
        self.scopes.last_mut().unwrap().define(ident, symbole)
    }

    pub fn insert(&mut self, ident: &'a str, unit: Unit<'a>) -> Result<()> {
        self.scopes.last_mut().unwrap().insert(ident, unit)
    }

    pub fn contains(&self, ident: &'a str) -> bool {
        self.scopes.iter().rev().any(|scope| scope.contains(ident))
    }

    pub fn get(&self, ident: &str) -> Result<&Variable<'a>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(ident))
            .ok_or_eyre(eyre!("Unknow variable {}", ident))
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::default())
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }
}

// impl<'a> Default for TowerScope<'a> {
//     fn default() -> Self {
//         Self {
//             scopes: vec![Scope::default()],
//         }
//     }
// }

impl<'a> Scope<'a> {
    fn define(&mut self, ident: &'a str, symbole: &'a str) -> Result<()> {
        if self.variables.contains_key(ident) {
            bail!("Variable `{}` is already declared", ident)
        }

        let axiom = Axiom::new(ident, symbole);
        let axiom = Variable::Axiom(axiom);

        self.variables.insert(ident, axiom);
        Ok(())
    }

    fn insert(&mut self, ident: &'a str, unit: Unit<'a>) -> Result<()> {
        if self.variables.contains_key(ident) {
            // TODO: Use digify error
            bail!("Variable `{}` is already declared", ident)
        }

        let variable = Variable::Unit(unit);

        self.variables.insert(ident, variable);
        Ok(())
    }

    fn contains(&self, ident: &str) -> bool {
        self.variables.contains_key(ident)
    }

    fn get(&self, ident: &str) -> Option<&Variable<'a>> {
        self.variables.get(ident)
    }
}
