use super::{class::Class, function::Function, variable::Variable};

#[derive(Debug)]
pub enum Declaration {
    Class(Class),
    Function(Function),
    Variable(Variable),
}
