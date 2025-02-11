use super::{
    class::Class, expression::Expression, function::Function, program::Program,
    statement::Statement,
};

#[derive(Debug)]
pub enum Node {
    Program(Program),
    Class(Class),
    Function(Function),
    Expression(Expression),
    Statement(Statement),
}
