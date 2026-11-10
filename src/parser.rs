use crate::ast::{BinaryOp, Expr, Stmt};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos + 1).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.current().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: Token, err_msg: &str) -> Result<(), String> {
        if self.current() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("Синтаксическая ошибка: {}, получено {:?}", err_msg, self.current()))
        }
    }

    // Точка входа: разбирает всю программу в список инструкций
    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while self.current() != &Token::Eof {
            statements.push(self.parse_stmt()?);
        }
        Ok(statements)
    }

    // Разбор одной инструкции (команды)
    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.current().clone() {
            Token::Let => self.parse_let(),
            Token::Print => self.parse_print(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Ident(name) => {
                if self.peek() == &Token::Assign {
                    self.advance(); // пропускаем имя переменной
                    self.advance(); // пропускаем '='
                    let value = self.parse_expression()?;
                    Ok(Stmt::Assign { name, value })
                } else {
                    Err(format!(
                        "Синтаксическая ошибка: ожидалось '=' после имени переменной '{}'",
                        name
                    ))
                }
            }
            other => Err(format!(
                "Синтаксическая ошибка: неожиданное начало инструкции {:?}",
                other
            )),
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, String> {
        self.advance(); // пропускаем 'let'
        let name = match self.advance() {
            Token::Ident(n) => n,
            other => {
                return Err(format!(
                    "Синтаксическая ошибка: после 'let' ожидалось имя переменной, получено {:?}",
                    other
                ))
            }
        };
        self.expect(Token::Assign, "ожидался символ '=' при объявлении переменной")?;
        let value = self.parse_expression()?;
        Ok(Stmt::Let { name, value })
    }

    fn parse_print(&mut self) -> Result<Stmt, String> {
        self.advance(); // пропускаем 'print'
        let expr = self.parse_expression()?;
        Ok(Stmt::Print(expr))
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.advance(); // пропускаем 'if'
        let condition = self.parse_expression()?;
        let then_branch = self.parse_block()?;

        let else_branch = if self.current() == &Token::Else {
            self.advance(); // пропускаем 'else'
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.advance(); // пропускаем 'while'
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Stmt::While { condition, body })
    }

    // Разбор блока инструкций внутри { ... }
    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(Token::LBrace, "ожидалась открывающая фигурная скобка '{'")?;
        let mut stmts = Vec::new();

        while self.current() != &Token::RBrace && self.current() != &Token::Eof {
            stmts.push(self.parse_stmt()?);
        }

        self.expect(Token::RBrace, "ожидалась закрывающая фигурная скобка '}'")?;
        Ok(stmts)
    }

    // --- Разбор выражений с учётом приоритетов ---

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_comparison()
    }

    // Самый низкий приоритет среди бинарных операций: ==, !=, >, <, >=, <=
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_term()?;

        while let Some(op) = match self.current() {
            Token::Eq => Some(BinaryOp::Eq),
            Token::NotEq => Some(BinaryOp::NotEq),
            Token::Gt => Some(BinaryOp::Gt),
            Token::Lt => Some(BinaryOp::Lt),
            Token::GtEq => Some(BinaryOp::GtEq),
            Token::LtEq => Some(BinaryOp::LtEq),
            _ => None,
        } {
            self.advance();
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // Сложение и вычитание: +, -
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_factor()?;

        while let Some(op) = match self.current() {
            Token::Plus => Some(BinaryOp::Add),
            Token::Minus => Some(BinaryOp::Sub),
            _ => None,
        } {
            self.advance();
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // Умножение, деление, остаток: *, /, %
    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;

        while let Some(op) = match self.current() {
            Token::Star => Some(BinaryOp::Mul),
            Token::Slash => Some(BinaryOp::Div),
            Token::Percent => Some(BinaryOp::Mod),
            _ => None,
        } {
            self.advance();
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // Унарный минус: -x
    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.current() == &Token::Minus {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::UnaryMinus(Box::new(expr)));
        }
        self.parse_primary()
    }

    // Базовые элементы: числа, true/false, переменные и скобки ( ... )
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::Bool(b) => Ok(Expr::Bool(b)),
            Token::Ident(name) => Ok(Expr::Variable(name)),
            Token::LParen => {
                let expr = self.parse_expression()?;
                self.expect(Token::RParen, "ожидалась закрывающая скобка ')'")?;
                Ok(expr)
            }
            other => Err(format!(
                "Синтаксическая ошибка в выражении: неожиданный токен {:?}",
                other
            )),
        }
    }
}
