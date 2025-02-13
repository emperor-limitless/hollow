use super::{keyword::Keyword, operator::Operator};

#[derive(Debug, Clone)]
pub enum TokenKind {
    Keyword(Keyword),
    Ident(String),
    Number(i64),
    Operator(Operator),
    LeftBrace,
    RightBrace,
    LefttParen,
    RightParen,
    Colon,
    Semicolon,
    Comma,
    DoubleQuote,
    SingleQuote, // For characters.
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token: Option<TokenKind>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub file: String,
    pub content: String,
}

impl Span {
    pub fn new(file: String, line: usize, column: usize, content: String) -> Self {
        Self {
            file,
            line,
            column,
            content,
        }
    }
    pub fn to_string(&self) -> String {
        format!(
            "\t--> {}:{}:{}\n\t{}",
            self.file, self.line, self.column, self.content
        )
    }
}
