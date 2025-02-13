#[derive(Debug, Clone)]
pub enum Keyword {
    Let,
    Var,
    Fn,
    Return,
    If,
    Else,
    // Reserved keywords. Even though we  don't have them for a while yet.
    Use,
    Class,
    Struct,
    Async,
    Await,
    Public,
    Private,
    Static,
}
