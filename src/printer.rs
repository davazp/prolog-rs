use itertools::Itertools;

use crate::terms::{Atom, Functor, Term, Variable};

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
                    "[]" => "[]".to_string(),
                    "[|]" => {
                        let left = args.get(0).unwrap();
                        let right = args.get(1).unwrap();
                        format!("{} | {}", print(left), print(right))
                    }
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
