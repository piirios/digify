
use std::mem;

use super::RawDimension;
use super::Sign;


pub struct Power<'di> {
    pub raw_dimention: &'di RawDimension<'di>,
    pub sign: Sign,
    pub power: u32,
}

impl<'di> Power<'di> {
    pub fn new(raw_dimention: &'di RawDimension<'di>, sign: Sign, power: u32) -> Self {
        Self {
            raw_dimention,
            sign,
            power,
        }
    }
}

#[derive(Debug)]
pub enum Concated<T> {
    Concated(T),
    Spreaded(T)
}

impl<T> Concated<T> {
    fn unwrap(self) -> T {
        match self {
            Self::Concated(t) => t,
            Self::Spreaded(t) => t
        }
    }

    fn get_mut(&mut self) -> &mut T {
        match self {
            Self::Concated(t) => t,
            Self::Spreaded(t) => t
        }
    }

    fn make_spreaded(&mut self) {
        let mut data: T = unsafe {
            mem::MaybeUninit::zeroed().assume_init()
        };
        let mut swapped = false;

        match self {
            Self::Concated(t) => {
                mem::swap(&mut data, t);
                swapped = true;
            }
            _ => ()
        }

        if swapped {
            mem::swap(self, &mut Self::Spreaded(data))
        }
    }
}

pub struct Flatten<'di> {
    vec: Concated<Vec<Power<'di>>>,
}

impl<'di> Flatten<'di> {
    #[inline]
    pub fn one(power: Power<'di>) -> Self {
        Self {
            vec: Concated::Spreaded(vec![power]),
        }
    }

    #[inline]
    pub fn append(&mut self, other: Self) {
        self.vec.make_spreaded();

        let vec = self.vec.get_mut();
        vec.append(&mut other.vec.unwrap());
    }

    fn concate(&mut self) {
        unimplemented!()
    }
}

mod tests {
    use super::*;

    #[test]
    fn make_spreaded_already_spread() {
        let v = vec![7, 6, 7, 8];
        let mut con = Concated::Spreaded(v);

        con.make_spreaded();

        println!("{:?}", con);

        assert!(matches!(con, Concated::Spreaded(_)));
        assert_eq!(con.unwrap(), vec![7, 6, 7, 8])
    }
    
    #[test]
    fn make_spreaded_already_concat() {
        let v = vec![7, 6, 8];
        let mut con = Concated::Concated(v);

        con.make_spreaded();

        println!("{:?}", con);

        assert!(matches!(con, Concated::Spreaded(_)));
        assert_eq!(con.unwrap(), vec![7, 6, 8])
    }
}