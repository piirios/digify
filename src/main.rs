#[macro_use]
extern crate pest_derive;

mod dimension;
mod error;
mod parse;
mod utils;
mod vtable;

use std::path::Path;

use error::Error;
use parse::{into_intructions_sorted, parse_from_file};
use vtable::VTable;

fn main() {
    let arg = std::env::args()
        .nth(1)
        .unwrap_or("./exemple.dgf".to_owned());
    let input_path = Path::new(&arg);

    let mut buffer = String::new();
    let ast = match parse_from_file(input_path, &mut buffer) {
        Err(err) => match err {
            Error::Io(err) => panic!("{:?}", err),
            Error::Pest(err) => panic!("{:?}", err),
            _ => panic!(),
        },
        Ok(pair) => pair,
    };

    let mut vtable = VTable::default();

    let i = into_intructions_sorted(ast);

    println!("{:#?}", i);
}
