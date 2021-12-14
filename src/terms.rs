#[derive(Debug, PartialEq, Eq)]
pub struct Name(pub String);

impl Name {
    #[allow(dead_code)]
    pub fn from(name: &str) -> Self {
        Name(name.to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Variable(pub String);

impl Variable {
    #[allow(dead_code)]
    pub fn from(name: &str) -> Self {
        Variable(name.to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Term {
    Integer(i32),
    Functor { name: Name, args: Vec<Term> },
    Var(Variable),
}

#[allow(dead_code)]
pub fn name(name: &str) -> Term {
    Term::Functor {
        name: Name(name.to_string()),
        args: vec![],
    }
}

pub fn functor2(name: &str, arg1: Term, arg2: Term) -> Term {
    Term::Functor {
        name: Name(name.to_string()),
        args: vec![arg1, arg2],
    }
}
