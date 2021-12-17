use clap::Parser;

use prolog_rs::*;

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

        let query = match parse_query(line.trim_end()) {
            Ok(query) => query,
            Err(_) => continue,
        };

        db.query(query);
    }
}
