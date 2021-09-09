mod power;

use power::*;

pub type Unite = String;

pub enum RawDimension<'di> {
    Unite(Unite),
    Power(Power<'di>),
    Composit(&'di RawDimension<'di>, Operator, &'di RawDimension<'di>),
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy)]
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

pub struct Dimension<'di> {
    flattened: Vec<RawDimension<'di>>,
    tree: RawDimension<'di>,
}

// impl<'di> From<RawDimension<'di>> for Dimension<'di> {
//     fn from(dim: RawDimension<'di>) -> Dimension<'di> {
//         Dimension {
//             tree: dim,
//             flattened: dim.flatten_recursive(Sign::Plus, 1),
//         }
//     }
// }

impl<'di> RawDimension<'di> {
    // TODO: add a method to concate all SI dimentions into one
    // see crate::dimension::power::Flatten::concate
    fn flatten_recursive(&'di self, sign: Sign, power: u32) -> Flatten<'di> {
        match self {
            Self::Unite(_) => Flatten::one(Power::new(self, Sign::Plus, power)),
            Self::Power(power_) => power_
                .raw_dimention
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
