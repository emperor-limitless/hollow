mod lexer;
use lexer::Lexer;
mod output;
use output::OutputBuilder;

fn main() -> anyhow::Result<()> {
    let mut lexer = Lexer::new("test.hl")?;
    let result = lexer.tokenize();
    println!("Parsed successfully!");
    Ok(())
}
