use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(pub String);

impl Name {
    #[allow(dead_code)]
    pub fn from(name: &str) -> Self {
        Name(name.to_string())
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct Variable(pub String, pub u32);

impl Variable {
    #[allow(dead_code)]
    pub fn from(name: &str) -> Self {
        Variable(name.to_string(), 0)
    }
}

#[derive(Clone)]
pub struct Goals(pub VecDeque<Functor>);

impl Goals {
    pub fn empty() -> Goals {
        Goals(VecDeque::new())
    }
    pub fn append(&mut self, other: &mut Goals) {
        self.0.append(&mut other.0);
    }
    pub fn select(&mut self) -> Option<Functor> {
        self.0.pop_front()
    }
    pub fn select_as_ref(&self) -> Option<&Functor> {
        self.0.get(0)
    }

    pub fn rename(&mut self, chr: u32) {
        for functor in self.0.iter_mut() {
            functor.rename(chr)
        }
    }

    pub fn as_term(&self) -> Term {
        self.0
            .iter()
            .map(|f| f.as_term())
            .reduce(|acc, next| Term::functor2(",", acc, next))
            .unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Functor {
    pub name: Name,
    pub args: Vec<Term>,
}

impl Functor {
    pub fn rename(&mut self, chr: u32) {
        for arg in self.args.iter_mut() {
            arg.rename(chr);
        }
    }

    pub fn as_term(&self) -> Term {
        Term::Fun(self.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Integer(i32),
    Fun(Functor),
    Var(Variable),
}

impl Term {
    #[allow(dead_code)]
    pub fn name(name: &str) -> Term {
        Term::Fun(Functor {
            name: Name(name.to_string()),
            args: vec![],
        })
    }

    #[allow(dead_code)]
    pub fn variable(name: &str) -> Term {
        Term::Var(Variable(name.to_string(), 0))
    }

    pub fn functor2(name: &str, arg1: Term, arg2: Term) -> Term {
        Term::Fun(Functor {
            name: Name(name.to_string()),
            args: vec![arg1, arg2],
        })
    }

    pub fn variables(&self) -> HashSet<&Variable> {
        let mut set = HashSet::new();
        term_variables_in_set(self, &mut set);
        set
    }

    pub fn as_functor(self) -> Option<Functor> {
        match self {
            Term::Fun(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_functor_ref(&self) -> Option<&Functor> {
        match self {
            Term::Fun(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_goals(self) -> Option<Goals> {
        match self.as_functor()? {
            Functor { name, mut args } if name == Name(",".to_string()) && args.len() == 2 => {
                let mut other = args.pop()?.as_goals()?;
                let mut query = args.pop()?.as_goals()?;
                query.append(&mut other);
                Some(query)
            }
            fun => Some(Goals(VecDeque::from([fun]))),
        }
    }

    pub fn rename(&mut self, chr: u32) {
        match self {
            Term::Fun(Functor { name, args }) => {
                for arg in args.iter_mut() {
                    arg.rename(chr);
                }
            }
            Term::Var(ref mut v) => {
                v.1 = chr;
            }
            _ => {}
        }
    }
}

fn term_variables_in_set<'a>(term: &'a Term, set: &mut HashSet<&'a Variable>) {
    match term {
        Term::Var(v) => {
            set.insert(v);
        }
        Term::Fun(Functor { args, .. }) => {
            for e in args.iter() {
                term_variables_in_set(e, set);
            }
        }
        _ => {}
    }
}

#[derive(Clone)]
pub struct Clause {
    pub head: Functor,
    pub body: Goals,
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
                    let body = args.pop().unwrap().as_goals().unwrap();
                    let head = args.pop().and_then(|h| h.as_functor()).unwrap();
                    Ok(Clause { head, body })
                }
                _ => Err(()),
            },
            Some(head) => Ok(Clause {
                head,
                body: Goals::empty(),
            }),
            None => Err(()),
        }
    }

    pub fn rename(&mut self, chr: u32) {
        self.head.rename(chr);
        self.body.rename(chr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn term_variables() {
        let term = parser::parse_expr("f(x,Y)").unwrap();
        let vars = term.variables();
        assert_eq!(vars.len(), 1);
        assert!(vars.get(&Variable::from("Y")).is_some());
    }
}
