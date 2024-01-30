use std::collections::HashMap;

// use color_eyre::eyre::{eyre, OptionExt};

use crate::error::{DigifyError, ErrorKind, Result};
use crate::interpreter::variable::{Unit, Variable};
use crate::parser::{Istr, Item};

use super::variable::{Axiom, IVariable};

#[derive(Debug, Default)]
pub struct TowerScope<'a> {
    scopes: Vec<Scope<'a>>,
}

#[derive(Debug, Default)]
pub struct Scope<'a> {
    variables: HashMap<&'a str, IVariable<'a>>,
}

impl<'a> TowerScope<'a> {
    pub fn define(&mut self, ident: Istr<'a>, symbole: &'a str) -> Result<'a, ()> {
        self.scopes.last_mut().unwrap().define(ident, symbole)
    }

    pub fn insert(&mut self, ident: Istr<'a>, unit: Unit<'a>) -> Result<'a, ()> {
        self.scopes.last_mut().unwrap().insert(ident, unit)
    }

    pub fn contains(&self, ident: &'a str) -> bool {
        self.scopes.iter().rev().any(|scope| scope.contains(ident))
    }

    pub fn get(&self, ident: Istr<'a>) -> Result<'a, &IVariable<'a>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(&ident).ok())
            .ok_or_else(|| {
                let kind = ErrorKind::VariableNotDeclared(ident.as_str().to_owned());
                let span = ident.span;
                DigifyError::new(kind, span)
            })
    }

    pub fn get_existing(&self, ident: &str) -> Option<&IVariable<'a>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get_existing(ident))
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
    fn define(&mut self, ident: Istr<'a>, symbole: &'a str) -> Result<'a, ()> {
        if self.variables.contains_key(ident.as_str()) {
            let kind = ErrorKind::VariableAlreadyDeclared(ident.as_str().to_owned());
            let span = ident.span;

            return Err(DigifyError::new(kind, span));
        }

        let axiom = Axiom::new(ident.as_str(), symbole);
        let axiom = Variable::Axiom(axiom);
        let axiom = Item::new(axiom, ident.span);

        // ident.item is used instead of ident.as_str() because the borrow checker
        // can not know that the ident.as_str() only borrow ident.item
        self.variables.insert(ident.item, axiom);
        Ok(())
    }

    fn insert(&mut self, ident: Istr<'a>, unit: Unit<'a>) -> Result<'a, ()> {
        if self.variables.contains_key(ident.as_str()) {
            let kind = ErrorKind::VariableAlreadyDeclared(ident.as_str().to_owned());
            let span = ident.span;

            return Err(DigifyError::new(kind, span));
        }

        let variable = Variable::Unit(unit);
        let variable = Item::new(variable, ident.span);

        // ident.item is used instead of ident.as_str() because the borrow checker
        // can not know that the ident.as_str() only borrow ident.item
        self.variables.insert(ident.item, variable);
        Ok(())
    }

    fn contains(&self, ident: &str) -> bool {
        self.variables.contains_key(ident)
    }

    fn get(&self, ident: &Istr<'a>) -> Result<'a, &IVariable<'a>> {
        if let Some(variable) = self.variables.get(ident.as_str()) {
            Ok(variable)
        } else {
            let kind = ErrorKind::VariableNotDeclared(ident.as_str().to_owned());
            let span = ident.span.clone();

            Err(DigifyError::new(kind, span))
        }
    }

    fn get_existing(&self, ident: &str) -> Option<&IVariable<'a>> {
        self.variables.get(ident)
    }
}
