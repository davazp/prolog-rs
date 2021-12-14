#[macro_use]
extern crate lalrpop_util;

use clap::Parser;
use std::fs;

mod parser;
mod printer;
mod terms;
mod unify;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(default_value_t = String::from("load.pl"))]
    file: String,
}

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(args.file).expect("Could not read file");

    let parser = parser::grammar::ProgramParser::new();
    let ast = parser.parse(&content).expect("could not parse");

    for clause in ast.iter() {
        println!("{}", printer::print(&clause));
    }
}
