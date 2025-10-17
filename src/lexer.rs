#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Let,
    If,
    Else,
    While,
    Fn,
    Return,

    // Identifiers and literals
    Ident,
    Number,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Eq,

    // Symbols
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,

    // End of input
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.pos).cloned()
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.source.get(self.pos).cloned();
        self.pos += 1;
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            self.skip_whitespace();

            if ch.is_alphabetic() || ch == '_' {
                tokens.push(self.lex_ident_or_keyword());
            } else if ch.is_ascii_digit() {
                tokens.push(self.lex_number());
            } else {
                match self.next().unwrap() {
                    '+' => tokens.push(Token { kind: TokenKind::Plus, value: "+".to_string() }),
                    '-' => tokens.push(Token { kind: TokenKind::Minus, value: "-".to_string() }),
                    '*' => tokens.push(Token { kind: TokenKind::Star, value: "*".to_string() }),
                    '/' => tokens.push(Token { kind: TokenKind::Slash, value: "/".to_string() }),
                    '=' => tokens.push(Token { kind: TokenKind::Eq, value: "=".to_string() }),
                    '(' => tokens.push(Token { kind: TokenKind::LParen, value: "(".to_string() }),
                    ')' => tokens.push(Token { kind: TokenKind::RParen, value: ")".to_string() }),
                    '{' => tokens.push(Token { kind: TokenKind::LBrace, value: "{".to_string() }),
                    '}' => tokens.push(Token { kind: TokenKind::RBrace, value: "}".to_string() }),
                    ',' => tokens.push(Token { kind: TokenKind::Comma, value: ",".to_string() }),
                    ';' => tokens.push(Token { kind: TokenKind::Semicolon, value: ";".to_string() }),
                    _ => panic!("Unexpected character '{}'", ch),
                }
            }
        }

        tokens.push(Token { kind: TokenKind::EOF, value: "".to_string() });
        tokens
    }

    fn lex_ident_or_keyword(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.pos += 1;
            } else {
                break;
            }
        }

        let kind = match ident.as_str() {
            "let" => TokenKind::Let,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "fn" => TokenKind::Fn,
            "return" => TokenKind::Return,
            _ => TokenKind::Ident,
        };

        Token { kind, value: ident }
    }

    fn lex_number(&mut self) -> Token {
        let mut num = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                num.push(ch);
                self.pos += 1;
            } else {
                break;
            }
        }
        Token { kind: TokenKind::Number, value: num }
    }
}
