use crate::dimension::RawDimension;
use crate::error::{Digirror, Error};
use std::collections::HashMap;

pub struct VTable {
    table: HashMap<Name, RawDimension>,
}

impl VTable {
    #[inline]
    pub fn push<E>(&mut self, name: Name, dimension: RawDimension) -> Result<(), Error<E>> {
        if !self.table.contains_key(&name) {
            self.table.insert(name, dimension);
            Ok(())
        } else {
            Err(Digirror::NameAlreadyExist(name).into())
        }
    }
}

impl Default for VTable {
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
