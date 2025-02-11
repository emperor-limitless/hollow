use super::{function::Function, variable::Variable};

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub parent: Option<String>,
    pub members: Vec<Member>,
}

#[derive(Debug)]
pub enum Member {
    Field(Variable),
    Method(Function),
}
