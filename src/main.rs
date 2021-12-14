#[macro_use]
extern crate lalrpop_util;

mod printer;
mod terms;

use clap::Parser;
use std::fs;
use terms::{name, Name, Term};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(default_value_t = String::from("load.pl"))]
    file: String,
}

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(args.file).expect("Could not read file");

    println!("{}", content);

    let _term_true = name("true");
    let _term_false = name("false");

    let term = Term::Functor {
        name: Name::from("plus"),
        args: vec![Term::Integer(1), Term::Integer(2), Term::Integer(3)],
    };

    println!("{}", printer::print(&term));
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
