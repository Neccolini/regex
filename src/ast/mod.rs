mod lexer;
pub mod parser;

#[derive(Debug, PartialEq)]
pub enum Ast {
    Literal(char),          // a
    Concat(Vec<Ast>),       // ab
    Alternate(Vec<Ast>),    // a|b
    Repetition(Repetition), // +, *
}

#[derive(Debug, PartialEq)]
pub struct Repetition {
    pub ast: Box<Ast>,
    pub min: u32,
    pub max: Option<u32>,
}
