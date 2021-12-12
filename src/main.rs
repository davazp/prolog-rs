#[macro_use]
extern crate lalrpop_util;

mod terms;

use terms::{atom, Atom, Term};

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

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[test]
fn grammar() {
    assert!(grammar::ExprParser::new().parse("22").is_ok());
    assert!(grammar::ExprParser::new().parse("true").is_ok());
    assert!(grammar::ExprParser::new().parse("false").is_ok());

    assert!(grammar::ExprParser::new().parse("plus(1,2,3)").is_ok());
    assert!(grammar::ExprParser::new().parse("plus(1,2,3,)").is_ok());
    assert!(grammar::ExprParser::new().parse("plus(1,2").is_err());

    assert!(grammar::ExprParser::new().parse("X(1,2)").is_err());

    assert!(grammar::ExprParser::new().parse("X").is_ok());
    assert!(grammar::ExprParser::new().parse("f(X)").is_ok());

    assert!(grammar::ExprParser::new().parse("2+X").is_ok());
    assert!(grammar::ExprParser::new().parse("2+X*2").is_ok());

    assert!(grammar::ExprParser::new().parse("(2+X)*2").is_ok());

    assert!(grammar::ExprParser::new().parse("g(f(2+X))").is_ok());
}
