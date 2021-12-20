use clap::Parser;
use std::io::{self, Write};

use prolog_rs::*;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(default_value_t = String::from("load.pl"))]
    file: String,
}

fn main() {
    let args = Args::parse();

    let session = Session::create(&args.file).expect("could not start session");

    loop {
        let mut line = String::new();
        print!("?- ");
        io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut line).unwrap();

        let query = match parse_query(line.trim_end()) {
            Ok(query) => query,
            Err(_) => {
                println!("invalid query");
                continue;
            }
        };

        session.query(query);
    }
}
