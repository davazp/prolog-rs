use itertools::Itertools;

use crate::terms::{Atom, Functor, Term, Variable};

fn print_list(term: &Term) -> String {
    let mut out = String::new();
    let mut cursor = term;

    // First element, if any

    if cursor.is_functor0("[]") {
        return "[]".to_string();
    } else if let Some((first, rest)) = cursor.is_functor2("[|]") {
        out.push_str("[");
        out.push_str(&print(first));
        cursor = rest;
    } else {
        panic!("Not a list.")
    }

    // List elements
    while let Some((first, rest)) = cursor.is_functor2("[|]") {
        out.push_str(", ");
        out.push_str(&print(first));
        cursor = rest;
    }

    // The tail
    if !cursor.is_functor0("[]") {
        out.push_str(" | ");
        out.push_str(&print(cursor));
    }
    out.push_str("]");

    out
}

pub fn print(term: &Term) -> String {
    match term {
        Term::Integer(value) => {
            format!("{}", value)
        }
        Term::Fun(Functor { name, args }) => {
            if args.is_empty() {
                format!("{}", name_as_string(name))
            } else {
                let name = name_as_string(name);
                match name {
                    "[]" => print_list(term),
                    "[|]" => print_list(term),
                    _ => {
                        format!(
                            "{}({})",
                            name,
                            Itertools::intersperse(args.iter().map(|a| print(a)), ",".to_string())
                                .collect::<String>()
                        )
                    }
                }
            }
        }
        Term::Var(v) => {
            format!("{}", variable_name(v))
        }
    }
}

fn name_as_string(name: &Atom) -> &str {
    let Atom(str) = name;
    str
}

fn variable_name(var: &Variable) -> &str {
    let Variable(str, _) = var;
    str
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_and_print(s: &str) -> String {
        let parsed = crate::parser::parse_expr(s).expect("valid");
        print(&parsed)
    }

    #[test]
    fn print_lists() {
        assert_eq!(parse_and_print("[1,2,3|X]"), "[1, 2, 3 | X]");
        assert_eq!(parse_and_print("[1,2,3|[]]"), "[1, 2, 3]");
    }
}
