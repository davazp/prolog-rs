#[macro_use]
extern crate lalrpop_util;

mod printer;
mod terms;

use terms::{name, Name, Term};

fn main() {
    let term_true = name("true");
    let term_false = name("false");

    let term = Term::Functor {
        name: Name("plus".to_string()),
        args: vec![Term::Integer(1), Term::Integer(2), Term::Integer(3)],
    };

    println!("{:?}", term_true);
    println!("{:?}", term_false);
    println!("{:?}", term);

    println!("Hello, world!");

    let t = grammar::ExprParser::new().parse("f((a,b))").unwrap();

    println!("{}", printer::print(&t));
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

    assert!(grammar::ExprParser::new().parse("test :- a").is_ok());

    assert!(grammar::ExprParser::new().parse("f((a,b))").is_ok());
}
