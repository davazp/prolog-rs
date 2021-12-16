#[macro_use]
extern crate lalrpop_util;

use clap::Parser;

mod database;
mod parser;
mod printer;
mod terms;
mod unify;

use database::Database;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(default_value_t = String::from("load.pl"))]
    file: String,
}

fn main() {
    let args = Args::parse();

    let db = Database::from_file(&args.file).expect("could not read database");

    // for clause in ast.iter() {
    //     println!("{}", printer::print(&clause));
    // }
}
