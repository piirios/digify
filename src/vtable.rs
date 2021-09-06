
use std::collections::HashMap;
use crate::error::{Digirror, Error};

pub struct VTable<'di> {
    table: HashMap<Name, Dimention<'di>>
}

impl<'di> VTable<'di> {
    #[inline]
    pub fn push<E>(&mut self, name: Name, dimention: Dimention<'di>) -> Result<(), Error<E>> {
        // TODO: return error if name already existing
        if !self.table.contains_key(&name) {
            self.table.insert(name, dimention);
            Ok(())
        } else {
            Err(Digirror::NameAlreadyExist(name).into())
        }
    }
}


impl<'di> Default for VTable<'di> {
    fn default() -> Self {
        Self {
            table: HashMap::new()
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct Name {
    name: String,
}

impl Name {
    #[inline]
    pub fn add_name(&mut self, name: String) {
        self.name = name
    }
}

pub enum Dimention<'di> {
    Unite(String),
    Power(&'di Dimention<'di>, u32),
    Composit(&'di Dimention<'di>, Operator, &'di Dimention<'di>),
    Flatten( Vec<&'di Dimention<'di>>)

}

pub enum Operator {
    Mul,
    Div
}