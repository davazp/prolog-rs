use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(pub String);

impl Name {
    #[allow(dead_code)]
    pub fn from(name: &str) -> Self {
        Name(name.to_string())
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct Variable(pub String);

impl Variable {
    #[allow(dead_code)]
    pub fn from(name: &str) -> Self {
        Variable(name.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Functor {
    pub name: Name,
    pub args: Vec<Term>,
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
        Term::Var(Variable(name.to_string()))
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
