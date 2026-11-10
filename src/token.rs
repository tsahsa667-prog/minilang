#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Литералы и идентификаторы
    Number(f64),
    Bool(bool),
    Ident(String),

    // Ключевые слова
    Let,
    Print,
    If,
    Else,
    While,

    // Математические операторы
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %

    // Присваивание и операторы сравнения
    Assign, // =
    Eq,     // ==
    NotEq,  // !=
    Gt,     // >
    Lt,     // <
    GtEq,   // >=
    LtEq,   // <=

    // Скобки и разделители
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }

    // Конец файла
    Eof,
}
