use std::boxed::Box;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use either::Either;
use itertools::Itertools;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

use crate::dimension::Operator;
use crate::error::Error;
use crate::utils::RemoveQuotes;

pub type Instructions<'s> = Vec<Instruction<'s>>;

#[derive(Debug)]
pub enum Instruction<'s> {
    Define(&'s str, &'s str),
    Import(&'s str),
    Let(&'s str, DimensionStr<'s>),
    Assert(bool, Vec<DimensionStr<'s>>),
    Print(Either<(Option<char>, Vec<DimensionStr<'s>>), &'s str>), // flag then ident or expr | text
}

#[derive(Debug, Clone)]
pub enum DimensionStr<'s> {
    Unite(&'s str),
    Power(Box<DimensionStr<'s>>, i32),
    Composit(Box<DimensionStr<'s>>, Operator, Box<DimensionStr<'s>>),
    Simplify(Box<DimensionStr<'s>>),
}

impl<'s> PartialEq for Instruction<'s> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Define(_, _) => match other {
                Self::Define(_, _) => true,
                _ => false,
            },
            Self::Import(_) => match other {
                Self::Import(_) => true,
                _ => false,
            },
            Self::Let(_, _) => match other {
                Self::Let(_, _) => true,
                _ => false,
            },
            Self::Assert(_, _) => match other {
                Self::Assert(_, _) => true,
                _ => false,
            },
            Self::Print(_) => match other {
                Self::Print(_) => true,
                _ => false,
            },
        }
    }
}

impl<'s> PartialOrd for Instruction<'s> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Self::Define(_, _) => match other {
                Self::Define(_, _) => Ordering::Equal,
                _ => Ordering::Less,
            },
            _ => match other {
                Self::Define(_, _) => Ordering::Greater,
                _ => Ordering::Equal,
            },
        })
    }
}

impl<'s> Eq for Instruction<'s> {}

impl<'s> Ord for Instruction<'s> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'s> DimensionStr<'s> {
    #[inline]
    fn push(self, op: Operator, other: Self) -> Self {
        Self::Composit(Box::new(self), op, Box::new(other))
    }

    #[inline]
    fn add_power(self, power: i32) -> Self {
        Self::Power(Box::new(self), power)
    }
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DigifyParse;

pub fn parse_from_file<'a>(
    input_path: &Path,
    buffer: &'a mut String,
) -> Result<Pairs<'a, Rule>, Error<Rule>> {
    let mut input_file = File::open(input_path)?;

    input_file.read_to_string(buffer)?;

    Ok(DigifyParse::parse(Rule::program, buffer)?)
}

fn parse_dimension_str<'s>(pair: Pair<'s, Rule>) -> Option<DimensionStr<'s>> {
    match pair.as_rule() {
        Rule::dimension => {
            let mut pair_iter = pair.into_inner();

            let first_pair = pair_iter.next().unwrap();
            let mut dimension = match first_pair.as_rule() {
                Rule::keyword_simplify => DimensionStr::Simplify(Box::new(
                    parse_dimension_str(pair_iter.next().unwrap()).unwrap(),
                )),
                _ => {
                    // println!("dim: {:?}", a);
                    parse_dimension_str(first_pair).unwrap()
                } // panic
            };

            for mut op_dim in &pair_iter.chunks(2) {
                let op = pair_into_operator(op_dim.next().unwrap()).unwrap();
                let dim = parse_dimension_str(op_dim.next().unwrap()).unwrap();
                dimension = dimension.push(op, dim);
            }
            Some(dimension)
        }
        Rule::dimension_ident => {
            let mut pair_iter = pair.into_inner();
            let mut dimension = DimensionStr::Unite(pair_iter.next().unwrap().as_str());

            if let Some(power) = pair_iter.next() {
                let power = power.as_str().parse::<i32>().unwrap();
                dimension = dimension.add_power(power);
            }

            Some(dimension)
        }
        Rule::dimension_groupe => {
            let mut pair_iter = pair.into_inner();
            let dimension = parse_dimension_str(pair_iter.next().unwrap()).unwrap();

            Some(dimension)
        }
        _ => None,
    }
}

fn pair_into_operator<'s>(pair: Pair<'s, Rule>) -> Option<Operator> {
    if matches!(pair.as_rule(), Rule::operator) {
        match pair.into_inner().next().unwrap().as_rule() {
            Rule::mul => Some(Operator::Mul),
            Rule::div => Some(Operator::Div),
            _ => unreachable!(),
        }
    } else {
        None
    }
}

pub fn into_intructions<'s>(ast: Pairs<'s, Rule>) -> Instructions<'s> {
    let instructions = ast
        .flat_map(|pair| {
            if matches!(pair.as_rule(), Rule::expr) {
                if let Some(stmt) = pair.into_inner().next() {
                    match stmt.as_rule() {
                        Rule::importExpr => {
                            let mut values = stmt.into_inner().skip(1);
                            let path = values.next().unwrap().as_str().remove_quotes();
                            Some(Instruction::Import(path))
                        }
                        Rule::letExpr => {
                            let mut values = stmt.into_inner().skip(1);
                            let ident = values.next().unwrap().as_span().as_str();
                            let dim = parse_dimension_str(values.next().unwrap()).unwrap();
                            Some(Instruction::Let(ident, dim))
                        }
                        Rule::defineExpr => {
                            let mut values = stmt.into_inner().skip(1);
                            let ident = values.next().unwrap().as_span().as_str();
                            let unit = values.next().unwrap().as_span().as_str().remove_quotes();
                            Some(Instruction::Define(ident, unit))
                        }
                        Rule::assertExpr => {
                            let mut values = stmt.into_inner().skip(1);

                            let first_pair = values.next().unwrap();
                            let (neg, dimensions) = match first_pair.as_rule() {
                                Rule::neg => (
                                    true,
                                    values
                                        .map(|pair| parse_dimension_str(pair).unwrap())
                                        .collect::<Vec<_>>(),
                                ),
                                _ => (
                                    false,
                                    std::iter::once(first_pair)
                                        .chain(values)
                                        .map(|pair| parse_dimension_str(pair).unwrap())
                                        .collect::<Vec<_>>(),
                                ),
                            };

                            Some(Instruction::Assert(neg, dimensions))
                        }
                        Rule::printExpr => {
                            let mut values = stmt.into_inner().skip(1);

                            let first_pair = values.next().unwrap();

                            let instruction = Instruction::Print(match first_pair.as_rule() {
                                r @ (Rule::flag | Rule::dimension) => {
                                    let flag = if matches!(r, Rule::flag) {
                                        Some(first_pair.as_str().escape_debug().nth(1).unwrap())
                                    } else {
                                        None
                                    };

                                    let dimensions = values
                                        .map(|pair| parse_dimension_str(pair).unwrap())
                                        .collect::<Vec<_>>();

                                    Either::Left((flag, dimensions))
                                }
                                Rule::string => Either::Right(first_pair.as_str()),
                                _ => unreachable!(),
                            });

                            Some(instruction)
                        }
                        x => {
                            println!("{:?}", x);
                            None
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Instructions>();
    instructions
}

pub fn into_intructions_sorted<'s>(ast: Pairs<'s, Rule>) -> Instructions<'s> {
    let mut instructions = into_intructions(ast);

    instructions.sort();

    instructions
}
