use crate::database::Database;
use crate::printer::print;
use crate::terms::{Clause, Query, Variable};
use crate::unify::unify_functors;

pub struct Session {
    db: Database,
}

impl Session {
    pub fn create(file: &str) -> Result<Session, ()> {
        let db = Database::from_file(file).map_err(|_| ())?;
        Ok(Session { db })
    }

    pub fn query(&self, mut query: Query) {
        let mut first = match query.select() {
            Some(c) => c,
            _ => {
                return;
            }
        };

        for Clause { head, body: _ } in self.db.matching_clauses(&first.name, first.args.len()) {
            if let Some(env) = unify_functors(&first, &head) {
                println!("-------");
                for (Variable(name), value) in env.map.iter() {
                    println!("{} = {}", name, print(value));
                }
            }
        }
        println!("-------");
        println!("false");
    }
}
