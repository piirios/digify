use core::fmt;
use std::error::Error;

use crate::parser::Span;

pub type Result<'a, T> = color_eyre::Result<T, DigifyError<'a>>;

#[derive(Debug)]
pub struct DigifyError<'a> {
    kind: ErrorKind,
    span: Span<'a>,
}

impl<'a> DigifyError<'a> {
    pub fn new(kind: ErrorKind, span: Span<'a>) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    AssertFail(String, String),
    VariableAlreadyDeclared(String),
    VariableNotDeclared(String),
}


impl<'a> fmt::Display for DigifyError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let padding = if self.span.start().col() - 1 == 0 {
            ""
        } else {
            " ... "
        };

        write!(f, 
            "{} | {}{}\n\t",
            self.span.start().line(),
            padding,
            self.span.input()
        )?;

        match &self.kind {
            ErrorKind::AssertFail(expected, actual) => {
                write!(f, "Assertion failed: expected {}, got {}", expected, actual)
            }
            ErrorKind::VariableAlreadyDeclared(ident) => {
                write!(f, "Variable {} already declared", ident)
            }
            ErrorKind::VariableNotDeclared(ident) => {
                write!(f, "Variable {} not declared", ident)
            }
        }
    }
}

impl<'a> Error for DigifyError<'a> {}
