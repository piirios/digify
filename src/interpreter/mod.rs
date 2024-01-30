mod element;
mod scope;
mod variable;

use element::Element;
use scope::TowerScope;
use variable::Unit;

use crate::error::{DigifyError, ErrorKind, Result};
use crate::parser::Element as AstElement;
use crate::parser::{IExpr, IStmt, Stmt};

#[derive(Debug, Default)]
pub struct Interpreter<'a> {
    scopes: TowerScope<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn eval(&mut self, stmt: IStmt<'a>) -> Result<'a, ()> {
        match stmt.item {
            Stmt::Definition(ident, symbole) => self.scopes.define(ident, symbole.item)?,
            Stmt::Let(ident, expr) => self.scopes.insert(ident, self.eval_expr(expr)?)?,
            Stmt::Assert(unit1, unit2) => {
                let unit1 = self.eval_expr(unit1)?;
                let unit2 = self.eval_expr(unit2)?;

                let cmp = unit1.eq(&unit2, &self.scopes);
                if !cmp {
                    let kind = ErrorKind::AssertFail(
                        unit1.to_string(&self.scopes),
                        unit2.to_string(&self.scopes),
                    );
                    let span = stmt.span.clone();

                    return Err(DigifyError::new(kind, span));
                }
            }
            Stmt::Print(element) => self.eval_element(element)?.println(&self.scopes),
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

    fn eval_expr(&self, expr: IExpr<'a>) -> Result<'a, Unit<'a>> {
        let unit = Unit::from(expr.item, &self.scopes)?;

        let idents = unit.top().keys().chain(unit.bottom().keys());

        for ident in idents {
            if !self.scopes.contains(ident) {
                let king = ErrorKind::VariableNotDeclared((*ident).to_owned());
                let span = expr.span;
                return Err(DigifyError::new(king, span));
            }
        }

        Ok(unit)
    }

    fn eval_element(&self, element: AstElement<'a>) -> Result<'a, Element<'a>> {
        let element = match element {
            AstElement::Expr(expr) => Element::Expr(self.eval_expr(expr)?),
            AstElement::String(string) => Element::String(string.item),
        };

        Ok(element)
    }
}
