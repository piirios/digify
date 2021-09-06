

use std::fs::File;
use std::io::Read;
use std::path::Path;

use pest::{Parser, iterators::Pairs};

use crate::{error::Error, vtable::{Dimention, Name, VTable}};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DigifyParse;

pub fn parse_from_file<'a>(input_path: &Path, buffer: &'a mut String) -> Result<Pairs<'a, Rule>, Error<Rule>> {
    
    let mut input_file = File::open(input_path)?;

    input_file.read_to_string(buffer)?;

    Ok(DigifyParse::parse(Rule::program, buffer)?)
}

pub fn define_unite(ast: Pairs<Rule>, vtable: &mut VTable) {
    ast.for_each(|pair| {
        if let Some(expr) = pair.into_inner().next() {
            match expr.as_rule() {
                Rule::defineExpr => {
                    expr.into_inner().skip(1).for_each(|part| {
                        let mut name = Name::default();
                        let mut string = String::new();
                        match part.as_rule() {
                            Rule::ident => name.add_name(part.as_span().as_str().to_owned()),
                            Rule::string => string = part.as_span().as_str().to_owned(),
                            _ => ()
                        }
                        vtable.push(name, Dimention::Unite(string))
                    })
                }
                _ => ()
            }
        }
    });
}