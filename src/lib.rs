pub mod combinators;
pub mod types;

use types::Program;

#[derive(Debug, PartialEq)]
pub struct Parser<'a> {
    pub source: String,
    pub program: Option<Program<'a>>,
}

impl<'a> From<String> for Parser<'a> {
    fn from(source: String) -> Self {
        Parser {
            source,
            program: None,
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser {
            source: String::new(),
            program: None,
        }
    }
}
