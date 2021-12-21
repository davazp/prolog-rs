use crate::database::Database;
use crate::env::Env;
use crate::terms::{Clause, Functor, Goals};
use crate::unify::unify_functors_in_env;
use std::collections::VecDeque;
use std::io;
use std::rc::Rc;

#[derive(Clone)]
struct Resolvent(Rc<ResolventNode>);
enum ResolventNode {
    Empty,
    Item { head: Functor, rest: Resolvent },
}

impl Resolvent {
    fn append(&self, goals: Vec<Functor>) -> Resolvent {
        let mut resolvent = self.clone();
        for g in goals.into_iter().rev() {
            let node = ResolventNode::Item {
                head: g,
                rest: resolvent,
            };
            resolvent = Resolvent(Rc::new(node))
        }
        resolvent
    }

    fn empty() -> Resolvent {
        Resolvent(Rc::new(ResolventNode::Empty))
    }

    fn from_goal(g: Functor) -> Resolvent {
        let node = ResolventNode::Item {
            head: g,
            rest: Resolvent::empty(),
        };
        Resolvent(Rc::new(node))
    }
}

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

    fn solve(&self, resolvent: &Resolvent, env: &mut Env, chr: u32) -> bool {
        match &*resolvent.0 {
            ResolventNode::Empty => {
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
            ResolventNode::Item {
                head: first,
                rest: remaining,
            } => {
                let mut clauses =
                    VecDeque::from(self.db.matching_clauses(&first.name, first.args.len()));
                for c in clauses.iter_mut() {
                    c.rename(chr)
                }
                self.prove(&first, clauses, &remaining, env, chr)
            }
        }
    }

    fn prove(
        &self,
        goal: &Functor,
        mut clauses: VecDeque<Clause>,
        resolvent: &Resolvent,
        env: &mut Env,
        chr: u32,
    ) -> bool {
        if clauses.is_empty() {
            false
        } else {
            env.push_frame();

            let clause = clauses.pop_front().unwrap();

            if unify_functors_in_env(env, &goal, &clause.head) {
                let newresolvent = resolvent.append(Vec::from(clause.body.0));
                if self.solve(&newresolvent, env, chr + 1) {
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

    pub fn query(&self, query: Goals) -> bool {
        let mut env = Env::new();
        let resolvent = Resolvent::append(&Resolvent::empty(), Vec::from(query.0));
        let result = self.solve(&resolvent, &mut env, 1);
        if !result {
            println!("false");
        }
        result
    }
}
