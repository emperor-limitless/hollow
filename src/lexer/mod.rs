use std::{fs, io::Read};

pub mod keyword;
use keyword::Keyword;
pub mod operator;
use operator::Operator;
pub mod token;
use crate::output::*;
use token::*;

pub struct Lexer<'a> {
    tokens: Vec<Token>,
    position: usize,
    line: usize,
    column: usize,
    input: String,
    file: &'a str,
    current_line: String,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a str) -> anyhow::Result<Self> {
        let mut source = fs::File::open(file)?;
        let mut input = String::new();
        source.read_to_string(&mut input)?;
        Ok(Self {
            tokens: vec![],
            position: 0,
            input,
            line: 1,
            column: 1,
            file,
            current_line: String::new(),
        })
    }

    pub fn create_span(&self) -> Span {
        Span::new(
            self.file.to_string(),
            self.line,
            self.column,
            self.current_line.clone(),
        )
    }

    pub fn peek(&mut self) -> bool {
        if self.position + 1 < self.input.len() {
            self.position += 1;
            self.column += 1;
            true
        } else {
            false
        }
    }

    fn get_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn handle_whitespace(&mut self) {
        if let Some(char) = self.get_char() {
            if char == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        // First, build the current_line up to the next newline
        let mut line_start = self.position;
        while let Some(c) = self.input.chars().nth(line_start) {
            if c == '\n' {
                break;
            }
            self.current_line.push(c);
            line_start += 1;
        }

        while self.position < self.input.len() {
            if let Some(char) = self.get_char() {
                match char {
                    c if c.is_whitespace() => {
                        if c == '\n' {
                            self.current_line = String::new();
                            // Build next line
                            let mut next_pos = self.position + 1;
                            while let Some(nc) = self.input.chars().nth(next_pos) {
                                if nc == '\n' {
                                    break;
                                }
                                self.current_line.push(nc);
                                next_pos += 1;
                            }
                        }
                        self.handle_whitespace();
                        continue;
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let text = self.handle_text();
                        // Handle token creation here
                        continue;
                    }
                    _ => {
                        let mut unknown = String::new();
                        unknown.push(char);
                        self.peek();

                        while let Some(next_char) = self.get_char() {
                            if next_char.is_whitespace() {
                                break;
                            }
                            unknown.push(next_char);
                            if !self.peek() {
                                break;
                            }
                        }
                        OutputBuilder::new()
                            .err(
                                self.create_span(),
                                &format!("Unexpected token: {}", unknown),
                            )
                            .build();
                    }
                }
            }
        }
        self.tokens.clone()
    }

    pub fn handle_text(&mut self) -> String {
        let mut text = String::new();
        while let Some(c) = self.get_char() {
            if c.is_alphanumeric() || c == '_' {
                text.push(c);
                if !self.peek() {
                    break;
                }
            } else {
                break;
            }
        }
        text
    }
}
