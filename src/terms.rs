#[derive(Debug, PartialEq, Eq)]
pub struct Atom(pub String);

#[derive(Debug, PartialEq, Eq)]
pub struct Variable(pub String);

#[derive(Debug, PartialEq, Eq)]
pub enum Term {
    Integer(i32),
    Functor { name: Atom, args: Vec<Term> },
    Var(Variable),
}

pub fn atom(name: &str) -> Term {
    Term::Functor {
        name: Atom(name.to_string()),
        args: vec![],
    }
}

pub fn functor2(name: &str, arg1: Term, arg2: Term) -> Term {
    Term::Functor {
        name: Atom(name.to_string()),
        args: vec![arg1, arg2],
    }
}
