use super::{access_modifier::AccessModifier, expression::Expression, statement::Statement};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: Option<String>,
    pub parameters: Vec<Parameter>,
    pub ody: Vec<Statement>,
    pub access: Option<AccessModifier>,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub parameter_type: String,
    pub default_value: Option<Expression>,
}
