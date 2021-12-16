use crate::parser;
use crate::terms::Term;
use std::fs;

pub struct Database {
    pub clauses: Vec<Term>,
}

#[derive(Debug)]
pub enum Error {
    NotExistingFile,
    ParsingError,
}

impl Database {
    pub fn from_file(file: &str) -> Result<Database, Error> {
        let content = fs::read_to_string(file).map_err(|_| Error::NotExistingFile)?;
        let parser = parser::grammar::ProgramParser::new();
        let clauses = parser.parse(&content).map_err(|_| Error::ParsingError)?;
        Ok(Database { clauses })
    }
}
