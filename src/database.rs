use crate::parser;
use crate::terms::{Name, Term};
use std::fs;

pub struct Database {
    pub clauses: Vec<Clause>,
}

#[derive(Debug)]
pub enum Error {
    NotExistingFile,
    ParsingError,
}

pub struct Clause {
    pub head: Term,
    pub body: Term,
}

impl Clause {
    pub fn from(term: Term) -> Result<Clause, ()> {
        match term {
            Term::Functor {
                name: Name(name),
                mut args,
            } if name == ":-" => match args.len() {
                0 => Err(()),
                1 => Err(()),
                2 => {
                    let body = args.pop().unwrap();
                    let head = args.pop().unwrap();
                    Ok(Clause { head, body })
                }
                _ => Err(()),
            },
            head => Ok(Clause {
                head,
                body: Term::name("true"),
            }),
        }
    }
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
}
