use crate::database::Database;
use crate::env::Env;
use crate::terms::{Clause, Functor, Goals};
use crate::unify::unify_functors_in_env;
use std::collections::VecDeque;
use std::io;

pub struct Session {
    db: Database,
    pub interactive: bool,
}

impl Session {
    pub fn create(file: &str) -> Result<Session, ()> {
        let db = Database::from_file(file).map_err(|_| ())?;
        Ok(Session {
            db,
            interactive: true,
        })
    }

    fn solve(&self, resolvent: &mut Goals, env: &mut Env, chr: u32) -> bool {
        match resolvent.0.pop_front() {
            None => {
                println!("solution:");
                env.print();
                println!("-------------------------");

                if self.interactive {
                    let mut line = String::new();
                    io::stdin().read_line(&mut line).expect("read line");
                    match line.trim() {
                        ";" => false,
                        _ => true,
                    }
                } else {
                    true
                }
            }
            Some(selection) => {
                let mut clauses = VecDeque::from(
                    self.db
                        .matching_clauses(&selection.name, selection.args.len()),
                );
                for c in clauses.iter_mut() {
                    c.rename(chr)
                }
                self.prove(selection, clauses, resolvent, env, chr)
            }
        }
    }

    fn prove(
        &self,
        goal: Functor,
        mut clauses: VecDeque<Clause>,
        resolvent: &mut Goals,
        env: &mut Env,
        chr: u32,
    ) -> bool {
        if clauses.is_empty() {
            false
        } else {
            env.push_frame();

            let clause = clauses.pop_front().unwrap();

            if unify_functors_in_env(env, &goal, &clause.head) {
                let mut newresolvent: Goals = Goals(VecDeque::with_capacity(
                    clause.body.0.len() + resolvent.0.len(),
                ));

                for lit in clause.body.0.iter() {
                    newresolvent.0.push_back(lit.clone());
                }
                for lit in resolvent.0.iter() {
                    newresolvent.0.push_back(lit.clone())
                }
                if self.solve(&mut newresolvent, env, chr + 1) {
                    true
                } else {
                    env.pop_frame();
                    self.prove(goal, clauses, resolvent, env, chr)
                }
            } else {
                env.pop_frame();
                self.prove(goal, clauses, resolvent, env, chr)
            }
        }
    }

    pub fn query(&self, mut query: Goals) -> bool {
        let mut env = Env::new();
        let result = self.solve(&mut query, &mut env, 1);
        if !result {
            println!("false");
        }
        result
    }
}
