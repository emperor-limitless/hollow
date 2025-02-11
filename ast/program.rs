use super::{access_modifier::AccessModifier, declaration::Declaration, import::Import};

#[derive(Debug)]
pub struct Program {
    pub imports: Vec<Import>,
    pub declarations: Vec<Declaration>,
    pub access: Option<AccessModifier>,
}
