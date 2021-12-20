use prolog_rs::*;
use std::fs;

#[test]
fn run_tests_pl() {
    let mut session = Session::create("load.pl").expect("could not start session");
    session.interactive = false;

    let queries_string = fs::read_to_string("tests.pl").unwrap();

    match parse_queries(&queries_string) {
        Ok(queries) => {
            for query in queries {
                println!("{}", print(&query.as_term()));
                assert!(session.query(query));
            }
        }
        Err(_) => {
            println!("failing");
            panic!("could not parse tests.pl");
        }
    }
}
