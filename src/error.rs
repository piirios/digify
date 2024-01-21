use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct DigifyError {
    kind: ErrorKind,
    span: Span,
}

impl DigifyError {
    pub fn new(kind: ErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    AssertFail(String, String),
}

#[derive(Debug, Default)]
pub struct Span {
    line: usize,
    column: usize,
    // length: usize,
}

impl fmt::Display for DigifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error at line {}, column {}:\n\t",
            self.span.line, self.span.column
        )?;

        match &self.kind {
            ErrorKind::AssertFail(expected, actual) => {
                write!(f, "Assertion failed: expected {}, got {}", expected, actual)
            }
        }
    }
}

impl Error for DigifyError {}
