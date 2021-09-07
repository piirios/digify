
use std::{collections::HashMap, ops::IndexMut};
use std::boxed::Box;
use crate::error::{Digirror, Error};

pub struct VTable<'di> {
    table: HashMap<Name, RawDimension<'di>>
}

impl<'di> VTable<'di> {
    #[inline]
    pub fn push<E>(&mut self, name: Name, dimention: RawDimension<'di>) -> Result<(), Error<E>> {
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

pub enum RawDimension<'di> {
    Unite(String),
    Power(&'di RawDimension<'di>,Sign, u32),
    Composit(&'di RawDimension<'di>, Operator, &'di RawDimension<'di>),

}
impl<'di> From<RawDimension<'di>> for Dimension<'di>{
    fn from(dim: RawDimension<'di>) -> Dimension<'di>{
        match self{
            Self::Unite(name) => {
                Dimension{
                    flattened: vec![*self],
                    tree: *self
                }
            }
            Self::Power(dim,sign, pow) => {
                let resolved = Self::Power(*dim, sign.resolve(power));
                Dimension{
                    flattened: vec![*self],
                    tree: *self
                }
            }
            Self::Composit(dim1, op, dim2) => {
                match op{
                    Operator::Mul => {
                        Dimension::combine(dim1.parse(sign), *op, dim2.parse(sign))
                    },
                    Operator::Div => {
                        Dimension::combine(dim1.parse(sign), *op, dim2.parse(sign.flip()))
                    }
                }
            }
        }
    }
}

struct Dimension<'di>{
    flattened: Vec<RawDimension<'di>>,
    tree: RawDimension<'di>
}
impl<'a> Dimension<'a>{
    fn combine(dim1: Dimension<'a>, op: Operator, dim2: Dimension<'a>) -> Dimension<'a>{
        let flattened = vec![dim1.flattened, dim2.flattened].into_iter().flatten().collect::<Vec<RawDimension<'a>>>();
        let tree = RawDimension::Composit(&dim1.tree, op, &dim2.tree);
        Dimension{
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
        match self {
            Self::Plus => Self::Minus,
            Self::Minus => Self::Plus
        }
    }
    const fn resolve(&self, power:i32) -> i32 {
        match self {
            Self::Plus => power,
            Self::Minus => -power
        }
    }
}