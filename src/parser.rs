use crate::ast::*;
use crate::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, kind: TokenKind) -> Token {
        let tok = self.next().expect("Unexpected end of input");
        if tok.kind != kind {
            panic!("Expected {:?}, got {:?}", kind, tok.kind);
        }
        tok
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.peek().is_some() {
            stmts.push(self.parse_stmt());
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.peek().map(|t| &t.kind) {
            Some(TokenKind::Let) => self.parse_let(),
            Some(TokenKind::If) => self.parse_if(),
            Some(TokenKind::While) => self.parse_while(),
            Some(TokenKind::Fn) => self.parse_function(),
            Some(TokenKind::Return) => self.parse_return(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_let(&mut self) -> Stmt {
        self.expect(TokenKind::Let);
        let name = self.expect(TokenKind::Ident).value;
        self.expect(TokenKind::Eq);
        let expr = self.parse_expr();
        self.expect(TokenKind::Semicolon);
        Stmt::Let { name, expr }
    }

    fn parse_if(&mut self) -> Stmt {
        self.expect(TokenKind::If);
        let cond = self.parse_expr();
        let then_block = self.parse_block();
        let else_block = if self.peek().map(|t| t.kind.clone()) == Some(TokenKind::Else) {
            self.next();
            Some(self.parse_block())
        } else {
            None
        };
        Stmt::If { cond, then_block, else_block }
    }

    fn parse_while(&mut self) -> Stmt {
        self.expect(TokenKind::While);
        let cond = self.parse_expr();
        let body = self.parse_block();
        Stmt::While { cond, body }
    }

    fn parse_function(&mut self) -> Stmt {
        self.expect(TokenKind::Fn);
        let name = self.expect(TokenKind::Ident).value;
        self.expect(TokenKind::LParen);
        let mut params = Vec::new();
        while let Some(tok) = self.peek() {
            if tok.kind == TokenKind::RParen {
                break;
            }
            let param_name = self.expect(TokenKind::Ident).value;
            params.push(param_name);
            if let Some(tok) = self.peek() {
                if tok.kind == TokenKind::Comma {
                    self.next();
                }
            }
        }
        self.expect(TokenKind::RParen);
        let body = self.parse_block();
        Stmt::Function { name, params, body }
    }

    fn parse_return(&mut self) -> Stmt {
        self.expect(TokenKind::Return);
        let expr = self.parse_expr();
        self.expect(TokenKind::Semicolon);
        Stmt::Return(expr)
    }

    fn parse_expr_stmt(&mut self) -> Stmt {
        let expr = self.parse_expr();
        self.expect(TokenKind::Semicolon);
        Stmt::Expr(expr)
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.expect(TokenKind::LBrace);
        let mut stmts = Vec::new();
        while let Some(tok) = self.peek() {
            if tok.kind == TokenKind::RBrace {
                break;
            }
            stmts.push(self.parse_stmt());
        }
        self.expect(TokenKind::RBrace);
        stmts
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_binary()
    }

    fn parse_binary(&mut self) -> Expr {
        let mut left = self.parse_primary();
        while let Some(tok) = self.peek() {
            match tok.kind {
                TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                    let op = tok.kind.clone();
                    self.next();
                    let right = self.parse_primary();
                    left = Expr::Binary {
                        op: op.to_string(),
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        left
    }

    fn parse_primary(&mut self) -> Expr {
        let tok = self.next().expect("Unexpected end of input");
        match tok.kind {
            TokenKind::Number => Expr::Number(tok.value.parse().unwrap()),
            TokenKind::Ident => {
                if let Some(next) = self.peek() {
                    if next.kind == TokenKind::LParen {
                        self.next();
                        let mut args = Vec::new();
                        while let Some(arg) = self.peek() {
                            if arg.kind == TokenKind::RParen {
                                break;
                            }
                            args.push(self.parse_expr());
                            if let Some(tok) = self.peek() {
                                if tok.kind == TokenKind::Comma {
                                    self.next();
                                }
                            }
                        }
                        self.expect(TokenKind::RParen);
                        Expr::Call { name: tok.value, args }
                    } else {
                        Expr::Var(tok.value)
                    }
                } else {
                    Expr::Var(tok.value)
                }
            }
            TokenKind::LParen => {
                let expr = self.parse_expr();
                self.expect(TokenKind::RParen);
                expr
            }
            _ => panic!("Unexpected token {:?}", tok.kind),
        }
    }
}
