use color_eyre::eyre::{ensure, Result};

mod element;
mod scope;
mod variable;

use element::Element;
use scope::TowerScope;
use variable::Unit;

use crate::{error::{DigifyError, ErrorKind, Span}, parser::{Element as AstElement, Expr, Stmt}};

#[derive(Debug, Default)]
pub struct Interpreter<'b> {
    scopes: TowerScope<'b>,
}

impl<'a> Interpreter<'a> {
    pub fn eval(&mut self, stmt: Stmt<'a>) -> Result<()> {
        match stmt {
            Stmt::Definition(ident, symbole) => self.scopes.define(ident, symbole)?,
            Stmt::Let(ident, expr) => self.scopes.insert(ident, self.eval_expr(expr)?)?,
            Stmt::Assert(unit1, unit2) => {
                let unit1 = self.eval_expr(unit1)?;
                let unit2 = self.eval_expr(unit2)?;
                
                let cmp = unit1.eq(&unit2, &self.scopes);
                if !cmp {
                    let error = DigifyError::new(
                    ErrorKind::AssertFail(unit1.to_string(&self.scopes), unit2.to_string(&self.scopes)),
                        Span::default(),
                    );

                    return Err(error.into())
                }
                // ensure!(cmp, "assert fail bettewen {} and {}", expr1, expr2);
            }
            Stmt::Print(element) => self.eval_element(element)?.print(&self.scopes),
            Stmt::Block(stmts) => {
                self.scopes.enter_scope();
                for stmt in stmts {
                    self.eval(stmt)?;
                }
                self.scopes.exit_scope();
            }
        };

        Ok(())
    }

    fn eval_expr(&self, expr: Expr<'a>) -> Result<Unit<'a>> {
        let unit = Unit::from(expr, &self.scopes)?;

        for ident in unit.top().keys() {
            ensure!(self.scopes.contains(ident))
        }

        for ident in unit.bottom().keys() {
            ensure!(self.scopes.contains(ident))
        }

        Ok(unit)
    }

    fn eval_element(&self, element: AstElement<'a>) -> Result<Element<'a>> {
        let element = match element {
            AstElement::Expr(expr) => Element::Expr(self.eval_expr(expr)?),
            AstElement::String(string) => Element::String(string),
        };

        Ok(element)
    }
}
