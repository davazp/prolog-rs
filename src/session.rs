use crate::database::Database;
use crate::env::Env;
use crate::terms::{Clause, Functor, Goals};
use crate::unify::unify_functors_in_env;
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
        goals.into_iter().rfold(self.clone(), |acc, g| {
            let node = ResolventNode::Item { head: g, rest: acc };
            Resolvent(Rc::new(node))
        })
    }

    fn add(&self, goal: Functor) -> Resolvent {
        let node = ResolventNode::Item {
            head: goal,
            rest: self.clone(),
        };
        Resolvent(Rc::new(node))
    }

    fn empty() -> Resolvent {
        Resolvent(Rc::new(ResolventNode::Empty))
    }

    fn from_slice(v: &[Functor]) -> Resolvent {
        Resolvent::empty().append(Vec::from(v))
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

    fn answer(&self, env: &Env) -> bool {
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

    fn solve(&self, resolvent: &Resolvent, env: &mut Env, chr: u32) -> bool {
        match &*resolvent.0 {
            ResolventNode::Empty => self.answer(env),
            ResolventNode::Item {
                head: first,
                rest: remaining,
            } => {
                let name = &first.name;
                let arity = first.args.len();

                match (name.0.as_str(), arity) {
                    (";", 2) => {
                        let left = first
                            .args
                            .get(0)
                            .and_then(|t| t.clone().as_functor())
                            .unwrap();
                        let right = first
                            .args
                            .get(1)
                            .and_then(|t| t.clone().as_functor())
                            .unwrap();

                        if self.solve(&remaining.add(left), env, chr) {
                            return true;
                        }
                        if self.solve(&remaining.add(right), env, chr) {
                            return true;
                        }
                        false
                    }
                    (_, _) => {
                        let mut clauses = self.db.matching_clauses(name, arity);
                        for c in clauses.iter_mut() {
                            c.rename(chr)
                        }
                        self.prove(&first, &clauses, &remaining, env, chr)
                    }
                }
            }
        }
    }

    fn prove(
        &self,
        goal: &Functor,
        clauses: &[Clause],
        resolvent: &Resolvent,
        env: &mut Env,
        chr: u32,
    ) -> bool {
        for clause in clauses {
            env.push_frame();
            if unify_functors_in_env(env, &goal, &clause.head) {
                let newresolvent = resolvent.append(clause.body.0.clone());
                if self.solve(&newresolvent, env, chr + 1) {
                    env.pop_frame();
                    return true;
                }
            }
            env.pop_frame();
        }
        false
    }

    pub fn query(&self, query: Goals) -> bool {
        let mut env = Env::new();
        let resolvent = Resolvent::from_slice(&query.0);
        let result = self.solve(&resolvent, &mut env, 1);
        if !result {
            println!("false");
        }
        result
    }
}
