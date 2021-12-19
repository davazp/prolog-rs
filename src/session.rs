use crate::database::Database;
use crate::printer::print;
use crate::terms::{Clause, Functor, Goals, Variable};
use crate::unify::{unify_functors_in_env, Env};
use std::collections::VecDeque;

pub struct Session {
    db: Database,
}

impl Session {
    pub fn create(file: &str) -> Result<Session, ()> {
        let db = Database::from_file(file).map_err(|_| ())?;
        Ok(Session { db })
    }

    fn solve(&self, mut resolvent: Goals, env: Env, chr: i32) -> bool {
        match resolvent.0.pop_front() {
            None => true,
            Some(selection) => {
                let mut clauses = VecDeque::from(
                    self.db
                        .matching_clauses(&selection.name, selection.args.len()),
                );
                self.prove(selection, clauses, resolvent, env, chr)
            }
        }
    }

    fn prove(
        &self,
        goal: Functor,
        mut clauses: VecDeque<Clause>,
        mut resolvent: Goals,
        env: Env,
        chr: i32,
    ) -> bool {
        if clauses.is_empty() {
            false
        } else {
            let first = clauses[0].body.select_as_ref().unwrap();

            let mut newenv = env.clone();
            if unify_functors_in_env(&mut newenv, &goal, first) {
                let clause = clauses.pop_front().unwrap();
                for literal in clause.body.0.into_iter().rev() {
                    resolvent.0.push_front(literal);
                }
                self.solve(resolvent, newenv, chr + 1)
            } else {
                clauses.remove(0);
                self.prove(goal, clauses, resolvent, env, chr)
            }
        }
    }

    pub fn query(&self, query: Goals) {
        if self.solve(query, Env::new(), 0) {
            println!("solutions")
        } else {
            println!("false")
        }
    }
}
