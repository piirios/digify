use crate::dimension::RawDimension;
use crate::error::{Digirror, Error};
use std::collections::HashMap;

pub struct VTable<'di> {
    table: HashMap<Name, RawDimension<'di>>,
}

impl<'di> VTable<'di> {
    #[inline]
    pub fn push<E>(&mut self, name: Name, dimension: RawDimension<'di>) -> Result<(), Error<E>> {
        if !self.table.contains_key(&name) {
            self.table.insert(name, dimension);
            Ok(())
        } else {
            Err(Digirror::NameAlreadyExist(name).into())
        }
    }
}

impl<'di> Default for VTable<'di> {
    fn default() -> Self {
        Self {
            table: HashMap::new(),
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
