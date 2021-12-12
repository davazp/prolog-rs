#[derive(Debug, PartialEq, Eq)]
struct Atom(String);

#[derive(Debug, PartialEq, Eq)]
enum Term {
    Integer(i32),
    Functor { name: Atom, args: Vec<Term> },
}

fn atom(name: &str) -> Term {
    Term::Functor {
        name: Atom(name.to_string()),
        args: vec![],
    }
}

fn main() {
    let term_true = atom("true");
    let term_false = atom("false");

    let term = Term::Functor {
        name: Atom("plus".to_string()),
        args: vec![Term::Integer(1), Term::Integer(2), Term::Integer(3)],
    };

    println!("{:?}", term_true);
    println!("{:?}", term_false);
    println!("{:?}", term);

    println!("Hello, world!");
}
