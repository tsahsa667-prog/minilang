use crate::token::Token;

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current() {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }

            if ch == '/' && self.peek() == Some('/') {
                while let Some(c) = self.current() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
                continue;
            }

            if ch.is_ascii_digit() {
                tokens.push(self.read_number()?);
                continue;
            }

            if ch.is_ascii_alphabetic() || ch == '_' {
                tokens.push(self.read_identifier());
                continue;
            }

            let token = match ch {
                '+' => {
                    self.advance();
                    Token::Plus
                }
                '-' => {
                    self.advance();
                    Token::Minus
                }
                '*' => {
                    self.advance();
                    Token::Star
                }
                '/' => {
                    self.advance();
                    Token::Slash
                }
                '%' => {
                    self.advance();
                    Token::Percent
                }
                '(' => {
                    self.advance();
                    Token::LParen
                }
                ')' => {
                    self.advance();
                    Token::RParen
                }
                '{' => {
                    self.advance();
                    Token::LBrace
                }
                '}' => {
                    self.advance();
                    Token::RBrace
                }
                '=' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        Token::Eq
                    } else {
                        Token::Assign
                    }
                }
                '!' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        Token::NotEq
                    } else {
                        return Err(
                            "Лексическая ошибка: неожиданный символ '!' (возможно, имелось в виду '!=')".to_string()
                        );
                    }
                }
                '>' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        Token::GtEq
                    } else {
                        Token::Gt
                    }
                }
                '<' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        Token::LtEq
                    } else {
                        Token::Lt
                    }
                }
                _ => {
                    return Err(format!("Лексическая ошибка: неизвестный символ '{}'", ch));
                }
            };

            tokens.push(token);
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut num_str = String::new();
        let mut has_dot = false;

        while let Some(ch) = self.current() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        num_str
            .parse::<f64>()
            .map(Token::Number)
            .map_err(|_| format!("Лексическая ошибка: некорректное число '{}'", num_str))
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(ch) = self.current() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "let" => Token::Let,
            "print" => Token::Print,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            _ => Token::Ident(ident),
        }
    }
}
