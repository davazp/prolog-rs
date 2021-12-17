#[macro_use]
extern crate lalrpop_util;

use clap::Parser;

mod database;
mod parser;
mod printer;
mod terms;
mod unify;

use database::Database;
use unify::unify;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(default_value_t = String::from("load.pl"))]
    file: String,
}

fn main() {
    let args = Args::parse();

    let db = Database::from_file(&args.file).expect("could not read database");

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let query = parser::parse_query(&line).expect("invalid query");

    for clause in db.clauses.iter() {
        match unify(&query, clause) {
            Some(env) => {
                println!("{:?}", env);
            }
            None => {
                println!("false");
            }
        }
    }
}
