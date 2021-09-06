
#[macro_use]
extern crate pest_derive;

mod parse;
mod error;
mod vtable;

use std::path::Path;

use pest::iterators::{Pair, Pairs};

use vtable::VTable;
use parse::{parse_from_file, Rule};
use error::Error;


fn main() {
    let arg = std::env::args().nth(1).unwrap_or("./exemple.dgf".to_owned());
    let input_path = Path::new(&arg);    

    let mut buffer = String::new();
    let ast = match parse_from_file(input_path, &mut buffer) {
        Err(err) => match err {
            Error::Io(err) => panic!("{:?}", err),
            Error::Pest(err) => panic!("{:?}", err),
            _ => panic!()
        }
        Ok(pair) => pair
    };

    let mut vtable = VTable::default();

    // define_unite(ast.clone(), &mut vtable);
}
