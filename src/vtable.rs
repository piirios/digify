
use std::collections::HashMap;
use std::boxed::Box;
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
    Power(&'di Dimention<'di>,Sign, u32),
    Composit(&'di Dimention<'di>, Operator, &'di Dimention<'di>),

}

struct RawDimension<'di>{
    flattened: Vec<Dimention<'di>>,
    tree: Dimention<'di>
}
impl<'a> RawDimension<'a>{
    fn combine(dim1: RawDimension<'a>, op: Operator, dim2: RawDimension<'a>) -> RawDimension<'a>{
        let flattened = vec![dim1.flattened, dim2.flattened].into_iter().flatten().collect::<Vec<Dimention<'a>>>();
        let tree = Dimention::Composit(&dim1.tree, op, &dim2.tree);
        RawDimension{
            flattened: flattened,
            tree:tree
        }

    }
}

pub enum Operator {
    Mul,
    Div
}

enum Sign {
    Plus,
    Minus
}
impl Sign {
    const fn flip(&self) -> Self{
        match Self {
            Self::Plus => Self::Minus,
            Self::Minus => Self::Plus
        }
    }
    const fn resolve(&self, power:i32) -> i32 {
        match Self {
            Self::Plus => power,
            Self::Minus => -power
        }
    }
}