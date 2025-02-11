use super::{expression::Expression, variable::Variable};

#[derive(Debug)]
pub enum Statement {
    Variable(Variable),
    Expression(Expression),
    Return(Option<Expression>),
}
