//! We use a custom compiler output style instead of using an existing crate for screen reader accessibility.
//!
//! Unfortunately. While there are a lot of nice ones around, they do not suit the format in mind, and have some edge cases with screen readers
//!

use crate::lexer::token::Span;
use colored::*;

pub struct OutputBuilder {
    text: Vec<ColoredString>,
}

impl OutputBuilder {
    pub fn new() -> Self {
        Self { text: vec![] }
    }
    pub fn err(mut self, span: Span, text: &str) -> Self {
        let text = format!("Error: {}\n{}", text, span.to_string());
        self.text.push(text.red());
        self
    }
    pub fn warn(mut self, span: Span, text: &str) -> Self {
        let text = format!("Warning: {}\n{}", text, span.to_string());
        self.text.push(text.yellow());
        self
    }
    /// For stuff such as success build messages, succeeded module builds, etc.
    pub fn success(mut self, text: &str) -> Self {
        self.text.push(text.green());
        self
    }
    pub fn build(mut self) {
        for message in self.text {
            println!("{}", message);
            println!();
        }
    }
}
