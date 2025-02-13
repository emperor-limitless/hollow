// You might think the names are somewhat longer than usual programming languages.
// But I think that variants/types should somewhat describe themselves, within reason of course.
// So 'GreaterEquals' is fine, but 'IsLessThanButGreaterThanWhileBeingAsciiNumeric' is obviously not.

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Operator {
    EqualsEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
    Dot,
}
