#[derive(Debug)]
pub struct Import {
    pub name: String,
    pub imports: Vec<String>, // the sub-imports, I.E., `import std.(io, random);, becomes ["io", "random"]`
}
