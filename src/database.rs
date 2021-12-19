use crate::parser;
use crate::terms::{Clause, Functor, Name};
use std::fs;

pub struct Database {
    pub clauses: Vec<Clause>,
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

        let clauses = parser
            .parse(&content)
            .map_err(|_| Error::ParsingError)?
            .into_iter()
            .map(|t| Clause::from(t).map_err(|_| Error::ParsingError))
            .collect::<Result<Vec<Clause>, Error>>()?;

        Ok(Database { clauses })
    }

    pub fn matching_clauses(&self, fname: &Name, farity: usize) -> Vec<Clause> {
        let mut result: Vec<Clause> = self
            .clauses
            .iter()
            .filter(|c| {
                let Functor { name, args } = &c.head;
                name == fname && farity == args.len()
            })
            .cloned()
            .collect();
        result
    }
}
