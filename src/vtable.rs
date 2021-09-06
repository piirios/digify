
use std::collections::HashMap;
use crate::error::Error;

pub struct VTable<'di> {
    table: HashMap<Name, Dimention<'di>>
}

impl<'di> VTable<'di> {
    pub const fn push(&mut self, name: Name, dimention: Dimention) -> Error {
        // TODO: return error if name already existing
        self.table.insert(name, dimention)
    }
}


impl<'di> Default for VTable<'di> {
    fn default() -> Self {
        Self {
            table: HashMap::new()
        }
    }
}

#[derive(Debug, Default)]
pub struct Name {
    name: String,
}

impl Name {
    pub const fn add_name(&mut self, name: String) {
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