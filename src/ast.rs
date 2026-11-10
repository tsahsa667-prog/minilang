use std::fmt;

// Значения, с которыми работает интерпретатор во время выполнения
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
}

// Красивый вывод значений для команды print и сообщений об ошибках
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

// Бинарные операторы (арифметика и сравнение)
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,   // +
    Sub,   // -
    Mul,   // *
    Div,   // /
    Mod,   // %
    Eq,    // ==
    NotEq, // !=
    Gt,    // >
    Lt,    // <
    GtEq,  // >=
    LtEq,  // <=
}

// Выражения (возвращают значение Value)
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Bool(bool),
    Variable(String),
    UnaryMinus(Box<Expr>),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

// Инструкции / команды языка (выполняют действия)
#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    Print(Expr),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
}
