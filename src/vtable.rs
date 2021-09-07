
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

pub impl<'a> Dimention<'a>{
    fn flatten(&'a self, register: &VTable, sign: &Sign) -> Self{
        match self{
            Self::Composit(&dim1, &op, &dim2) => {
                match op {
                    Operator::Mul => {
                        Self::Flatten(
                            vec![dim1.flatten(&register, &sign), dim1.flatten(&register, &sign)]
                        )
                    },
                    Operator::Div => {
                        Self::Flatten(
                            vec![dim1.flatten(&register, &sign), dim2.flatten(&register, &sign.flip())]
                        )
                    }
                }
            },
            Self::Power(&dim, pow) => { Self::Power(&dim, sign.resolve(pow))},
            Self::Flatten(&content) => {content}
            Self::Unite(name) => Self::Unite(name)
       }
    }
    fn check(&'a self, other: &'a Dimention) -> bool {
        let me = match self{
            Self::Flatten(content) => content,
            
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