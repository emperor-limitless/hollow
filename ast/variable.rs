use super::expression::Expression;

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub Type: Option<String>,
    pub Mutable: bool,
    pub value: Option<Expression>,
}
