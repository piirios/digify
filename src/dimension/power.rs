use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::mem;
use std::rc::Rc;

use super::Sign;
use super::Unite;

#[derive(Debug, Hash)]
pub struct Power<D> {
    pub dimention: Rc<D>,
    pub sign: Sign,
    pub power: u32,
}

impl<D> Power<D> {
    pub fn new(dimention: Rc<D>, sign: Sign, power: u32) -> Self {
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
    Spreaded(T),
}

impl<T> Concat<T> {
    fn unwrap(self) -> T {
        match self {
            Self::Concated(t) => t,
            Self::Spreaded(t) => t,
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
        let mut data: T = unsafe { mem::MaybeUninit::zeroed().assume_init() };
        let mut swapped = false;

        match self {
            Self::Concated(t) => {
                mem::swap(&mut data, t);
                swapped = true;
            }
            _ => (),
        }

        if swapped {
            mem::swap(self, &mut Self::Spreaded(data))
        }
    }
}

#[derive(Debug)]
pub struct Flatten {
    vec: Concat<Vec<Power<Unite>>>,
}

impl Flatten {
    #[inline]
    pub fn one(power: Power<Unite>) -> Self {
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

    pub fn concate(&mut self) {
        let mut hmap = HashMap::new();

        for power_container in self.vec.get_ref() {
            let power = hmap.entry(power_container.dimention.clone()).or_insert(0);
            *power += power_container.as_i32()
        }

        let mut vec = hmap
            .drain()
            .map(|string_power| string_power.into())
            .collect::<Vec<Power<Unite>>>();

        vec.sort_unstable_by_key(|power| power.dimention.clone());

        self.vec = Concat::Concated(vec);
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.vec.get_ref().hash(&mut hasher);
        hasher.finish()
    }
}

impl<'s> From<(Rc<String>, i32)> for Power<Unite> {
    fn from((string, power): (Rc<String>, i32)) -> Self {
        let sign = if power.is_negative() {
            Sign::Minus
        } else {
            Sign::Plus
        };

        Self::new(string, sign, power.abs() as u32)
    }
}

mod tests {
    #[allow(unused_imports)]
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

    #[test]
    fn hash_same_flatten() {
        let flatten = Flatten {
            vec: Concat::Concated(vec![
                Power::new(Rc::new("M".to_owned()), Sign::Plus, 1),
                Power::new(Rc::new("L".to_owned()), Sign::Plus, 2),
                Power::new(Rc::new("T".to_owned()), Sign::Minus, 2),
            ]),
        };

        assert_eq!(flatten.hash(), flatten.hash())
    }

    #[test]
    fn hash_equals_flatten() {
        let flatten1 = Flatten {
            vec: Concat::Concated(vec![
                Power::new(Rc::new("M".to_owned()), Sign::Plus, 1),
                Power::new(Rc::new("L".to_owned()), Sign::Plus, 2),
                Power::new(Rc::new("T".to_owned()), Sign::Minus, 2),
            ]),
        };

        let flatten2 = Flatten {
            vec: Concat::Concated(vec![
                Power::new(Rc::new("M".to_owned()), Sign::Plus, 1),
                Power::new(Rc::new("L".to_owned()), Sign::Plus, 2),
                Power::new(Rc::new("T".to_owned()), Sign::Minus, 2),
            ]),
        };

        assert_eq!(flatten1.hash(), flatten2.hash())
    }

    #[test]
    fn hash_equals_flatten_after_concat() {
        let masse = Rc::new("M".to_owned());
        let longueur = Rc::new("L".to_owned());
        let temps = Rc::new("T".to_owned());

        let mut flatten1 = Flatten {
            vec: Concat::Concated(vec![
                Power::new(masse.clone(), Sign::Plus, 2),
                Power::new(temps.clone(), Sign::Minus, 2),
                Power::new(longueur.clone(), Sign::Plus, 2),
                Power::new(masse.clone(), Sign::Minus, 3),
                Power::new(masse.clone(), Sign::Plus, 2),
            ]),
        };
        flatten1.concate();

        let mut flatten2 = Flatten {
            vec: Concat::Concated(vec![
                Power::new(Rc::new("M".to_owned()), Sign::Plus, 1),
                Power::new(Rc::new("L".to_owned()), Sign::Plus, 2),
                Power::new(Rc::new("T".to_owned()), Sign::Minus, 2),
            ]),
        };
        flatten2.concate();

        assert_eq!(flatten1.hash(), flatten2.hash())
    }
}
