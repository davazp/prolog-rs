use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use prolog_rs::*;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(default_value_t = String::from("load.pl"))]
    file: String,
}

fn repl(file: &str) {
    let session = Session::create(file).expect("could not start session");

    let mut rl = Editor::<()>::new();
    let history_file = "prolog_rs_history";

    let _ = rl.load_history(history_file);

    loop {
        let readline = rl.readline("?- ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                let query = match parse_query(line.trim_end()) {
                    Ok(query) => query,
                    Err(_) => {
                        println!("invalid query");
                        continue;
                    }
                };
                session.query(query);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(history_file).unwrap();
}

fn main() {
    let args = Args::parse();
    repl(&args.file);
}
