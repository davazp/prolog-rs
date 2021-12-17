#[macro_use]
extern crate lalrpop_util;

use clap::Parser;

mod database;
mod parser;
mod printer;
mod terms;
mod unify;

use database::{Clause, Database};
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

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        let query = match parser::parse_query(line.trim_end()) {
            Ok(query) => query,
            Err(_) => continue,
        };

        for Clause { head, body } in db.clauses.iter() {
            if let Some(env) = unify(&query, head) {
                println!("-------");
                for (key, value) in env.map.iter() {
                    println!("{} = {}", key.0, printer::print(value));
                }
            }
        }

        println!("-------");
        println!("false");
    }
}
