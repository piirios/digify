use std::fmt;

use color_eyre::eyre::{bail, Result};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

mod span;

pub use span::*;

#[derive(Parser)]
#[grammar = "../grammar/grammar.pest"]
pub struct DigifyParser;

#[derive(Debug)]
pub struct Item<'a, T: fmt::Debug> {
    pub item: T,
    pub span: Span<'a>,
}

pub type Istr<'a> = Item<'a, &'a str>;
pub type IExpr<'a> = Item<'a, Expr<'a>>;
pub type IStmt<'a> = Item<'a, Stmt<'a>>;

#[derive(Debug)]
pub enum Stmt<'a> {
    Definition(Istr<'a>, Istr<'a>),
    Let(Istr<'a>, IExpr<'a>),
    Assert(IExpr<'a>, IExpr<'a>),
    Print(Element<'a>),
    Block(Vec<IStmt<'a>>),
}

#[derive(Debug)]
pub enum Expr<'a> {
    Ident(Istr<'a>),
    Mul(Box<IExpr<'a>>, Box<IExpr<'a>>),
    Div(Box<IExpr<'a>>, Box<IExpr<'a>>),
    Power(Box<IExpr<'a>>, i32),
    Simplify(Box<IExpr<'a>>),
    None,
}

#[derive(Debug)]
pub enum Element<'a> {
    String(Istr<'a>),
    Expr(IExpr<'a>),
}

impl DigifyParser {
    pub fn parse_to_ast(file: &str) -> Result<IStmt> {
        let file = Self::parse(Rule::program, file)?.next().unwrap();

        let span = Span::from(file.as_span());
        let block = file
            .into_inner()
            .map(Self::parse_stmt)
            .collect::<Result<Vec<_>>>()?;
        let block = Stmt::Block(block);

        Ok(Item::new(block, span))
    }

    fn parse_stmt(pair: Pair<Rule>) -> Result<IStmt> {
        if Rule::stmt == pair.as_rule() {
            let span = Span::from(pair.as_span());
            let mut inner = pair.into_inner();
            let keyword = inner.next().unwrap();

            let stmt = match keyword.as_rule() {
                Rule::keyword_define => {
                    let ident = inner.next().unwrap();
                    let string = inner.next().unwrap().into_inner().next().unwrap();

                    let ident = Item::new_str(ident);
                    let string = Item::new_str(string);

                    // let ident = Item::new(ident.as_str(), Span::from(ident.as_span()));
                    // let string = Item::new(string.as_str(), Span::from(string.as_span()));

                    Stmt::Definition(ident, string)
                }
                Rule::keyword_let => {
                    let ident = inner.next().unwrap();
                    let expr = Self::parse_expr(inner.next().unwrap())?;

                    let ident = Item::new_str(ident);

                    Stmt::Let(ident, expr)
                }
                Rule::keyword_assert => {
                    let expr1 = Self::parse_expr(inner.next().unwrap())?;
                    let expr2 = Self::parse_expr(inner.next().unwrap())?;
                    Stmt::Assert(expr1, expr2)
                }
                Rule::keyword_print => {
                    let element = Self::parse_element(inner.next().unwrap())?;
                    Stmt::Print(element)
                }
                _ => bail!("unkown rule in stmt match: {:?}", keyword.as_rule()),
            };

            Ok(Item::new(stmt, span))
        } else {
            bail!("Try parsing {:?} as a Stmt", pair.as_rule())
        }
    }

    fn parse_expr(pair: Pair<Rule>) -> Result<IExpr> {
        if pair.as_rule() == Rule::expr {
            let outer_span = Span::from(pair.as_span());
            pair.into_inner()
                .try_fold(Item::default(), move |acc, pair| {
                    let expr = match pair.as_rule() {
                        Rule::ident => {
                            let span = Span::from(pair.as_span());
                            let expr = Expr::Ident(Item::new_str(pair));
                            let expr = Item::new(expr, span.clone());

                            match acc.item {
                                Expr::Mul(inner, _) => {
                                    // let span = acc.span.extend(span);
                                    let expr = Expr::Mul(inner, Box::new(expr));

                                    Item::new(expr, acc.span)
                                }
                                Expr::Div(inner, _) => {
                                    let expr = Expr::Div(inner, Box::new(expr));
                                    // let span = acc.span.extend(span);

                                    Item::new(expr, acc.span)
                                }
                                Expr::Simplify(_) => {
                                    let expr = Expr::Simplify(Box::new(expr));
                                    // let span = acc.span.extend(span);

                                    Item::new(expr, acc.span)
                                }
                                _ => expr,
                            }
                        }
                        Rule::expr => {
                            let expr = Self::parse_expr(pair)?;

                            match acc.item {
                                Expr::Mul(inner, _) => {
                                    let expr = Expr::Mul(inner, Box::new(expr));
                                    // let span = acc.span.extend(span);

                                    Item::new(expr, acc.span)
                                }
                                Expr::Div(inner, _) => {
                                    let expr = Expr::Div(inner, Box::new(expr));
                                    // let span = acc.span.extend(span);

                                    Item::new(expr, acc.span)
                                }
                                Expr::Simplify(_) => {
                                    let expr = Expr::Simplify(Box::new(expr));
                                    // let span = acc.span.extend(span);

                                    Item::new(expr, acc.span)
                                }
                                _ => expr,
                            }
                        }
                        Rule::mul => {
                            let expr = Expr::Mul(Box::new(acc), Box::default());
                            // let span = Span::from(pair.as_span());

                            Item::new(expr, outer_span.clone())
                        }
                        Rule::div => {
                            let expr = Expr::Div(Box::new(acc), Box::default());
                            // let span = Span::from(pair.as_span());

                            Item::new(expr, outer_span.clone())
                        }
                        Rule::number => {
                            let expr = Expr::Power(
                                Box::new(acc),
                                pair.as_str().trim().parse::<i32>().unwrap(),
                            );
                            let span = Span::from(pair.as_span());

                            Item::new(expr, span)
                        }
                        Rule::percent => {
                            let expr = Expr::Simplify(Box::default());
                            let span = Span::from(pair.as_span());

                            Item::new(expr, span)
                        }
                        _ => unreachable!(),
                    };

                    Ok(expr)
                })
        } else {
            bail!("Try parsing {:?} as a Expr", pair.as_rule())
        }
    }

    fn parse_element(pair: Pair<Rule>) -> Result<Element> {
        if pair.as_rule() == Rule::element {
            let inner = pair.into_inner().next().unwrap();

            let element = match inner.as_rule() {
                Rule::expr => Element::Expr(Self::parse_expr(inner)?),
                Rule::string => {
                    let string = inner.into_inner().next().unwrap();
                    Element::String(Item::new_str(string))
                }
                _ => unreachable!(),
            };

            Ok(element)
        } else {
            bail!("Try parsing {:?} as a Element", pair.as_rule())
        }
    }
}

impl<'a, T: fmt::Debug> Item<'a, T> {
    pub fn item(&self) -> &T {
        &self.item
    }
}

impl<'a> Item<'a, &'a str> {
    fn new_str(value: Pair<'a, Rule>) -> Self {
        Self {
            item: value.as_str(),
            span: Span::from(value.as_span()),
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.item
    }
}

impl<'a> Default for Item<'a, Expr<'a>> {
    fn default() -> Self {
        Self {
            item: Expr::None,
            span: Span::default(),
        }
    }
}

impl<'a, T: fmt::Debug> Item<'a, T> {
    pub fn new(item: T, span: Span<'a>) -> Self {
        Self { item, span }
    }
}
