use itertools::Itertools;

use crate::terms::{Functor, Name, Term, Variable};

pub fn print(term: &Term) -> String {
    match term {
        Term::Integer(value) => {
            format!("{}", value)
        }
        Term::Fun(Functor { name, args }) => {
            if args.is_empty() {
                format!("{}", name_as_string(name))
            } else {
                format!(
                    "{}({})",
                    name_as_string(name),
                    Itertools::intersperse(args.iter().map(|a| print(a)), ",".to_string())
                        .collect::<String>()
                )
            }
        }
        Term::Var(v) => {
            format!("{}", variable_name(v))
        }
    }
}

fn name_as_string(name: &Name) -> &str {
    let Name(str) = name;
    str
}

fn variable_name(var: &Variable) -> &str {
    let Variable(str) = var;
    str
}
