
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
    Power(&'di RawDimension<'di>,Sign, i32),
    Composit(&'di RawDimension<'di>, Operator, &'di RawDimension<'di>),

}
struct Dimension<'di>{
    flattened: Vec<RawDimension<'di>>,
    tree: RawDimension<'di>
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
    const fn add(&self, other: Self) -> Self {
        match self{
            Self::Plus => {
                match other {
                    Self::Plus => Self::Plus,
                    Self::Minus => Self::Minus
                }
            },
            Self::Minus => {
                match other{
                    Self::Plus => Self::Minus,
                    Self::Minus => Self::Plus
                }
            }
        }
    }
}

impl<'di> From<RawDimension<'di>> for Dimension<'di>{
    fn from(dim: RawDimension<'di>) -> Dimension<'di>{
        Dimension{
            flattened: dim.flatten_recursive(Sign::Plus, 1),
            tree: dim
        }
        }
    }

impl<'a> RawDimension<'a>{
    fn flatten_recursive(&'a self, sign: Sign, power: i32) -> Vec<RawDimension>{
        match self{
            Self::Unite(name) => {
                vec![Self::Power(&RawDimension::Unite(name.to_string()), Sign::Plus, sign.resolve(power))]
            }
            Self::Power(dim,sign_pow, pow) => {
                dim.flatten_recursive(sign_pow.add(sign), power*pow)
            }
            Self::Composit(dim1, op, dim2) => {
                match op{
                    Operator::Mul => {
                        let array = vec![dim1.flatten_recursive(sign, power), dim2.flatten_recursive(sign, power)];
                        array.into_iter().flatten().collect::<Vec<RawDimension>>()
                    },
                    Operator::Div => {
                        vec![dim1.flatten_recursive(sign, power), dim2.flatten_recursive(sign.flip(), power)].into_iter().flatten().collect::<Vec<RawDimension>>()
                    }
                }
            }
        }   
    }
}
impl<'a> Dimension<'a>{
    fn check(&'a self, other: Self) -> Option<bool>{
        self.flattened.into_iter().fold(Some(true), |before, element|{
            let r = if let RawDimension::Power(unit, _,pow) = element{
                if let RawDimension::Unite(own_name) = unit{
                    other.flattened.into_iter().flat_map(|el| {
                        if let RawDimension::Power(unit_other,_, pow_other ) = el{
                            if let RawDimension::Unite(uname) = unit_other{
                                if (own_name == uname) && (pow == pow_other){
                                    Some(true)
                                }else{
                                    None
                                }
                            }else{
                                None
                            }
                        }else{
                            None
                        }
                    }).collect::<Vec<bool>>().first()
                } else {
                    None
                }
            } else {
                None
            };
            Some(*r? && before?)
        }
    }
}