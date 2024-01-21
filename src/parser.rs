use color_eyre::eyre::{bail, Result};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar/grammar.pest"]
pub struct DigifyParser;

#[derive(Debug)]
pub enum Stmt<'a> {
    Definition(&'a str, &'a str),
    Let(&'a str, Expr<'a>),
    Assert(Expr<'a>, Expr<'a>),
    Print(Element<'a>),
    Block(Vec<Stmt<'a>>),
}

#[derive(Debug)]
pub enum Expr<'a> {
    Ident(&'a str),
    Mul(Box<Expr<'a>>, Box<Expr<'a>>),
    Div(Box<Expr<'a>>, Box<Expr<'a>>),
    Power(Box<Expr<'a>>, i32),
    None,
}

#[derive(Debug)]
pub enum Element<'a> {
    String(&'a str),
    Expr(Expr<'a>),
}

impl DigifyParser {
    pub fn parse_to_ast(file: &str) -> Result<Stmt> {
        let file = Self::parse(Rule::program, file)?.next().unwrap();

        let block = file
            .into_inner()
            .map(Self::parse_stmt)
            .collect::<Result<Vec<_>>>()?;

        Ok(Stmt::Block(block))
    }

    fn parse_stmt(pair: Pair<Rule>) -> Result<Stmt> {
        if Rule::stmt == pair.as_rule() {
            let mut inner = pair.into_inner();
            let keyword = inner.next().unwrap();

            let stmt = match keyword.as_rule() {
                Rule::keyword_define => {
                    let ident = inner.next().unwrap().as_str();
                    let string = inner.next().unwrap().into_inner().next().unwrap().as_str();
                    Stmt::Definition(ident, string)
                }
                Rule::keyword_let => {
                    let ident = inner.next().unwrap().as_str();
                    let expr = Self::parse_expr(inner.next().unwrap());
                    Stmt::Let(ident, expr)
                }
                Rule::keyword_assert => {
                    let expr1 = Self::parse_expr(inner.next().unwrap());
                    let expr2 = Self::parse_expr(inner.next().unwrap());
                    Stmt::Assert(expr1, expr2)
                }
                Rule::keyword_print => {
                    let element = Self::parse_element(inner.next().unwrap());
                    Stmt::Print(element)
                }
                _ => bail!("unkown rule in stmt match: {:?}", keyword.as_rule()),
            };

            Ok(stmt)
        } else {
            bail!("Try parsing {:?} as a Stmt", pair.as_rule())
        }
    }

    fn parse_expr(pair: Pair<Rule>) -> Expr {
        if pair.as_rule() == Rule::expr {

            pair.into_inner().fold(Expr::None, |acc, pair| {
                let expr = match pair.as_rule() {
                    Rule::ident => {
                        let expr = Expr::Ident(pair.as_str());

                        match acc {
                            Expr::Mul(inner, _) => Expr::Mul(inner, Box::new(expr)),
                            Expr::Div(inner, _) => Expr::Div(inner, Box::new(expr)),
                            _ => expr,
                        }
                    }
                    Rule::expr => {
                        let expr = Self::parse_expr(pair);

                        match acc {
                            Expr::Mul(inner, _) => Expr::Mul(inner, Box::new(expr)),
                            Expr::Div(inner, _) => Expr::Div(inner, Box::new(expr)),
                            _ => expr,
                        }
                    }
                    Rule::mul => Expr::Mul(Box::new(acc), Box::new(Expr::None)),
                    Rule::div => Expr::Div(Box::new(acc), Box::new(Expr::None)),
                    Rule::number => {
                        Expr::Power(Box::new(acc), pair.as_str().parse::<i32>().unwrap())
                    }
                    _ => unreachable!(),
                };

                expr
            })
        } else {
            unreachable!()
        }
    }

    fn parse_element(pair: Pair<Rule>) -> Element {
        if pair.as_rule() == Rule::element {
            let inner = pair.into_inner().next().unwrap();

            match inner.as_rule() {
                Rule::expr => Element::Expr(Self::parse_expr(inner)),
                Rule::string => Element::String(inner.into_inner().next().unwrap().as_str()),
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }
}
