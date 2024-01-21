mod power;

use std::rc::Rc;

use power::*;

pub type Unite = String;

pub enum RawDimension {
    Unite(Rc<Unite>),
    Power(Power<RawDimension>),
    Composit(Rc<RawDimension>, Operator, Rc<RawDimension>),
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum Sign {
    Plus,
    Minus,
}
impl Sign {
    const fn flip(&self) -> Self {
        match self {
            Self::Plus => Self::Minus,
            Self::Minus => Self::Plus,
        }
    }

    const fn multiply(&self, other: Self) -> Self {
        match self {
            Self::Plus => match other {
                Self::Plus => Self::Plus,
                Self::Minus => Self::Minus,
            },
            Self::Minus => match other {
                Self::Plus => Self::Minus,
                Self::Minus => Self::Plus,
            },
        }
    }
}

pub struct Dimension {
    raw_dimension: RawDimension,
    flattened: Flatten,
    hash: u64, // used to compare two dimentions
}

impl From<RawDimension> for Dimension {
    fn from(dim: RawDimension) -> Dimension {
        let flattened = dim.flatten();
        let hash = flattened.hash();

        Dimension {
            raw_dimension: dim,
            flattened,
            hash,
        }
    }
}

impl PartialEq for Dimension {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl RawDimension {
    #[inline]
    fn flatten(&self) -> Flatten {
        self.flatten_recursive(Sign::Plus, 1)
    }

    fn flatten_recursive(&self, sign: Sign, power: u32) -> Flatten {
        match self {
            Self::Unite(unite) => Flatten::one(Power::new(unite.clone(), Sign::Plus, power)),
            Self::Power(power_) => power_
                .dimention
                .flatten_recursive(power_.sign.multiply(sign), power * power_.power),
            Self::Composit(dim1, op, dim2) => match op {
                Operator::Mul => {
                    let mut flatten = dim1.flatten_recursive(sign, power);
                    flatten.append(dim2.flatten_recursive(sign, power));
                    flatten
                }
                Operator::Div => {
                    let mut flatten = dim1.flatten_recursive(sign, power);
                    flatten.append(dim2.flatten_recursive(sign.flip(), power));
                    flatten
                }
            },
        }
    }
}

// impl<'a> Dimension<'a> {
//     fn check(&'a self, other: Self) -> Option<bool> {
//         self.flattened
//             .into_iter()
//             .fold(Some(true), |before, element| {
//                 let r = if let RawDimension::Power(power1) = element {
//                     if let RawDimension::Unite(own_name) = power1.raw_dimention {
//                         other
//                             .flattened
//                             .into_iter()
//                             .flat_map(|el| {
//                                 if let RawDimension::Power(power2) = el {
//                                     if let RawDimension::Unite(uname) = power2.raw_dimention {
//                                         if (own_name == uname) && (power1.power == power2.power) {
//                                             Some(true)
//                                         } else {
//                                             None
//                                         }
//                                     } else {
//                                         None
//                                     }
//                                 } else {
//                                     None
//                                 }
//                             })
//                             .collect::<Vec<bool>>()
//                             .first()
//                     } else {
//                         None
//                     }
//                 } else {
//                     None
//                 };
//                 Some(*r? && before?)
//             })
//     }
// }
