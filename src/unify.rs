use crate::env::Env;
use crate::terms::{Functor, Term, Variable};

pub fn unify_functors_in_env(env: &mut Env, f1: &Functor, f2: &Functor) -> bool {
    match (f1, f2) {
        (
            Functor {
                name: name1,
                args: args1,
            },
            Functor {
                name: name2,
                args: args2,
            },
        ) if name1 == name2 && args1.len() == args2.len() => {
            for (x, y) in args1.iter().zip(args2.iter()) {
                if !unify_in_env(env, x, y) {
                    return false;
                }
            }
            true
        }
        // impossible? but seems necessary to make the compiler happy
        (_, _) => false,
    }
}

fn unify_in_env(env: &mut Env, t1: &Term, t2: &Term) -> bool {
    match (t1, t2) {
        (Term::Integer(x), Term::Integer(y)) => x == y,
        (Term::Var(x), Term::Var(y)) if x == y => true,
        (Term::Var(x), value) => bind_var(env, x, value.clone()),
        (value, Term::Var(x)) => bind_var(env, x, value.clone()),
        (Term::Fun(f1), Term::Fun(f2)) => unify_functors_in_env(env, f1, f2),
        _ => false,
    }
}

fn bind_var(env: &mut Env, var: &Variable, value: Term) -> bool {
    if occur_check(var, &value) {
        return false;
    }
    let binding = env.lookup(var).map(|x| x.clone());
    if let Some(bound_value) = binding {
        unify_in_env(env, &value, &bound_value)
    } else {
        env.bind(var.clone(), value);
        true
    }
}

fn occur_check(var: &Variable, term: &Term) -> bool {
    match term {
        Term::Var(v) if v == var => true,
        Term::Fun(Functor { args, .. }) => args.iter().find(|a| occur_check(var, a)).is_some(),
        _ => false,
    }
}

pub fn substitute(env: &Env, term: &Term) -> Term {
    match term {
        Term::Fun(Functor { name, args }) => Term::Fun(Functor {
            name: name.clone(),
            args: args.iter().map(|e| substitute(env, e)).collect(),
        }),
        Term::Var(v) => {
            if let Some(value) = env.lookup(v) {
                substitute(env, value)
            } else {
                Term::Var(v.clone())
            }
        }
        _ => term.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    fn unify(t1: &Term, t2: &Term) -> Option<Env> {
        let mut env = Env::new();
        env.push_frame();
        if unify_in_env(&mut env, t1, t2) {
            Some(env)
        } else {
            None
        }
    }

    fn parse_expr(s: &str) -> Term {
        parser::parse_expr(s).unwrap()
    }

    fn unify_exprs(e1: &str, e2: &str) -> Option<Env> {
        let t1 = parse_expr(e1);
        let t2 = parse_expr(e2);
        let result = unify(&t1, &t2);
        return result;
    }

    fn unifier(e1: &str, e2: &str) -> Term {
        let t1 = parse_expr(e1);
        let t2 = parse_expr(e2);
        let env = unify(&t1, &t2).expect("expressions do not unify");
        substitute(&env, &t1)
    }

    #[test]
    fn test_identical_var() {
        assert!(unify_exprs("X", "X").is_some());
    }

    #[test]
    fn test_free_vars_unify() {
        assert!(unify_exprs("X", "Y").is_some());
    }

    #[test]
    fn constants_values() {
        assert!(unify_exprs("2", "3").is_none());
    }

    #[test]
    fn duplicated_vars() {
        assert!(unify_exprs("f(X,X)", "f(1,2)").is_none());
    }

    #[test]
    fn test_functors() {
        assert!(unify_exprs("plus(X,2)", "plus(1,Y)").is_some());
    }

    #[test]
    fn test_occur_check() {
        assert!(unify_exprs("X", "plus(1,X)").is_none());
    }

    #[test]
    fn test_complex_unification() {
        assert!(unify_exprs("f(g(X, Y, U), U)", "f(g(a, U, Z), Y)").is_some());
    }

    #[test]
    fn test_unifiers() {
        assert_eq!(unifier("plus(X,2)", "plus(1,Y)"), parse_expr("plus(1,2)"));
    }

    #[test]
    fn test_transitive_unification() {
        assert_eq!(unifier("f(X,Y)", "f(Y,1)"), parse_expr("f(1,1)"));
    }
}
