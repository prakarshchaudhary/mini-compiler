use std::collections::{HashMap, HashSet};
use crate::ast::*;

pub struct SemanticAnalyzer {
    variables: HashSet<String>,
    functions: HashSet<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            variables: HashSet::new(),
            functions: HashSet::new(),
        }
    }

    pub fn analyze(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.visit_stmt(stmt);
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, expr } => {
                self.visit_expr(expr);
                self.variables.insert(name.clone());
            }
            Stmt::If { cond, then_block, else_block } => {
                self.visit_expr(cond);
                for s in then_block {
                    self.visit_stmt(s);
                }
                if let Some(block) = else_block {
                    for s in block {
                        self.visit_stmt(s);
                    }
                }
            }
            Stmt::While { cond, body } => {
                self.visit_expr(cond);
                for s in body {
                    self.visit_stmt(s);
                }
            }
            Stmt::Function { name, params, body } => {
                self.functions.insert(name.clone());
                let old_vars = self.variables.clone();
                for p in params {
                    self.variables.insert(p.clone());
                }
                for s in body {
                    self.visit_stmt(s);
                }
                self.variables = old_vars;
            }
            Stmt::Return(expr) => {
                self.visit_expr(expr);
            }
            Stmt::Expr(expr) => {
                self.visit_expr(expr);
            }
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(_) => {}
            Expr::Var(name) => {
                if !self.variables.contains(name) {
                    eprintln!("Warning: variable `{}` used before declaration", name);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.visit_expr(left);
                self.visit_expr(right);
            }
            Expr::Call { name, args } => {
                if !self.functions.contains(name) {
                    eprintln!("Warning: function `{}` called before declaration", name);
                }
                for arg in args {
                    self.visit_expr(arg);
                }
            }
        }
    }
}
