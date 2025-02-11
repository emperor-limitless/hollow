use chumsky::prelude::*;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Variable(String),
}

fn main() -> impl Parser<char, Expr, Error = Simple<char>> {
    let ident = text::ident().padded(); // Handle whitespace around identifier

    let type_annotation = just(':').padded().ignore_then(text::ident()).padded(); // The actual type name

    let value = text::ident() // This is simplified - you'd want actual expression parsing here
        .padded();

    let let_binding = just("let")
        .padded()
        .ignore_then(ident)
        .then(type_annotation.or_not()) // Makes the type annotation optional
        .then_ignore(just('=').padded())
        .then(value)
        .then_ignore(just(';'));

    // Test cases
    let test1 = let_binding.parse("let x = value;");
    let test2 = let_binding.parse("let foo: String = bar;");
    println!("Parsed successfully!");
}
