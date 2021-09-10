
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

    pub fn as_i32(&self) -> i32 {
        match self.sign {
            Sign::Plus => self.power as i32,
            Sign::Minus => -(self.power as i32),
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
            Self::Spreaded(t) => t,
        }
    }

    fn get_ref(&self) -> &T {
        match self {
            Self::Concated(t) => t,
            Self::Spreaded(t) => t,
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
        let mut hmap = HashMap::new();

        for power_container in self.vec.get_ref() {
            let power = hmap.entry(power_container.dimention).or_insert(0);
            *power += power_container.as_i32()
        }

        let mut vec = hmap
            .drain()
            .map(|string_power| string_power.into())
            .collect::<Vec<Power<'di, Unite>>>();

        vec.sort_unstable_by_key(|power| power.dimention);

        self.vec = Concat::Concated(vec);
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.vec.get_ref().hash(&mut hasher);
        hasher.finish()
    }
}

impl<'s> From<(&'s String, i32)> for Power<'s, Unite> {
    fn from((string, power): (&'s String, i32)) -> Self {
        let sign = if power.is_negative() {
            Sign::Minus
        } else {
            Sign::Plus
        };

        Self::new(string, sign, power.abs() as u32)
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