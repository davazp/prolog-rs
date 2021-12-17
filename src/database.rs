use crate::parser;
use crate::printer::print;
use crate::terms::{Functor, Name, Term};
use crate::unify::unify_functors;
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
    pub head: Functor,
    pub body: Term,
}

impl Clause {
    pub fn from(term: Term) -> Result<Clause, ()> {
        match term.as_functor() {
            Some(Functor {
                name: Name(name),
                mut args,
            }) if name == ":-" => match args.len() {
                0 => Err(()),
                1 => Err(()),
                2 => {
                    let body = args.pop().unwrap();
                    let head = args.pop().and_then(|h| h.as_functor()).unwrap();
                    Ok(Clause { head, body })
                }
                _ => Err(()),
            },
            Some(head) => Ok(Clause {
                head,
                body: Term::name("true"),
            }),
            None => Err(()),
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

    fn matching_clauses(&self, fname: &Name, farity: usize) -> Vec<&Clause> {
        self.clauses
            .iter()
            .filter(|c| {
                if let Functor { name, args } = &c.head {
                    name == fname && farity == args.len()
                } else {
                    false
                }
            })
            .collect()
    }

    pub fn query(&self, query: Functor) {
        for Clause { head, body: _ } in self.clauses.iter() {
            if let Some(env) = unify_functors(&query, head) {
                println!("-------");
                for (key, value) in env.map.iter() {
                    println!("{} = {}", key.0, print(value));
                }
            }
        }
        println!("-------");
        println!("false");
    }
}
