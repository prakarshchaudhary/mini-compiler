#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(i32),
    Operator(String),
    Punctuation(char),
}
pub struct Lexer {
    source: String,
    pos: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer { source, pos: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.pos < self.source.len() {
            let current_char = self.source.chars().nth(self.pos).unwrap();

            if current_char.is_whitespace() {
                self.pos += 1;
                continue;
            }

            // Numbers
            if current_char.is_digit(10) {
                let mut num_str = String::new();
                while self.pos < self.source.len() && self.source.chars().nth(self.pos).unwrap().is_digit(10) {
                    num_str.push(self.source.chars().nth(self.pos).unwrap());
                    self.pos += 1;
                }
                tokens.push(Token::Number(num_str.parse().unwrap()));
                continue;
            }

            // Identifiers / Keywords
            if current_char.is_alphabetic() {
                let mut ident = String::new();
                while self.pos < self.source.len() && self.source.chars().nth(self.pos).unwrap().is_alphanumeric() {
                    ident.push(self.source.chars().nth(self.pos).unwrap());
                    self.pos += 1;
                }

                match ident.as_str() {
                    "let" | "if" | "else" | "while" | "fn" | "return" => tokens.push(Token::Keyword(ident)),
                    _ => tokens.push(Token::Identifier(ident)),
                }
                continue;
            }

            // Operators and punctuation
            match current_char {
                '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' => {
                    let mut op = current_char.to_string();
                    self.pos += 1;
                    // Handle double char operators like '==', '>=', '!='
                    if self.pos < self.source.len() {
                        let next_char = self.source.chars().nth(self.pos).unwrap();
                        if (current_char == '=' && next_char == '=') ||
                            (current_char == '!' && next_char == '=') ||
                            (current_char == '<' && next_char == '=') ||
                            (current_char == '>' && next_char == '=') {
                            op.push(next_char);
                            self.pos += 1;
                        }
                    }
                    tokens.push(Token::Operator(op));
                }
                '(' | ')' | '{' | '}' | ';' => {
                    tokens.push(Token::Punctuation(current_char));
                    self.pos += 1;
                }
                _ => {
                    panic!("Unknown character: {}", current_char);
                }
            }
        }

        tokens
    }
}
