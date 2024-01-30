use std::fs::File;
use std::io::Read;

use color_eyre::{
    config::HookBuilder,
    eyre::{ensure, Context, Result},
};

mod error;
mod interpreter;
mod parser;

use parser::DigifyParser;

use crate::interpreter::Interpreter;

fn main() -> Result<()> {
    HookBuilder::blank()
        .add_default_filters()
        .display_location_section(false)
        .install()?;

    let args = std::env::args().skip(1);
    let (flags, args): (Vec<_>, Vec<_>) = args.partition(|arg| arg.starts_with('-'));

    ensure!(args.len() == 1, "No input file");

    let mut input = String::new();
    let mut file = File::open(&args[0]).wrap_err_with(|| format!("No file named: {}", args[1]))?;
    file.read_to_string(&mut input)?;

    let input: &'static str = input.leak();

    let ast = DigifyParser::parse_to_ast(input)?;

    let mut interpreter = Interpreter::default();

    if flags.contains(&"-d".to_string()) {
        dbg!(&ast);
    }
    interpreter.eval(ast)?;

    Ok(())
}
