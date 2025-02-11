#[derive(Debug)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    StringLiteral(String),
    Boolean(bool),
    Identifier(String),
}
