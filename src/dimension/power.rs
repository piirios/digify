
use std::mem;

use super::RawDimension;
use super::Sign;


pub struct Power<'di, D> {
    pub dimention: &'di D,
    pub sign: Sign,
    pub power: u32,
}

impl<'di, D> Power<'di, D> {
    pub fn new(dimention: &'di D, sign: Sign, power: u32) -> Self {
        Self {
            dimention,
            sign,
            power,
        }
    }
}

#[derive(Debug)]
pub enum Concat<T> {
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
    vec: Concat<Vec<Power<'di, Unite>>>,
}

impl<'di> Flatten<'di> {
    #[inline]
    pub fn one(power: Power<'di, Unite>) -> Self {
        Self {
            vec: Concat::Spreaded(vec![power]),
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
        let mut con = Concat::Spreaded(v);

        con.make_spreaded();

        println!("{:?}", con);

        assert!(matches!(con, Concat::Spreaded(_)));
        assert_eq!(con.unwrap(), vec![7, 6, 7, 8])
    }
    
    #[test]
    fn make_spreaded_already_concat() {
        let v = vec![7, 6, 8];
        let mut con = Concat::Concated(v);

        con.make_spreaded();

        println!("{:?}", con);

        assert!(matches!(con, Concat::Spreaded(_)));
        assert_eq!(con.unwrap(), vec![7, 6, 8])
    }
}