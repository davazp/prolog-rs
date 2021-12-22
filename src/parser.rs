lalrpop_mod!(pub grammar);

use crate::terms::{Goals, Term};

#[allow(dead_code)]
pub fn parse_expr(str: &str) -> Result<Term, ()> {
    let parser = grammar::ExprParser::new();
    parser.parse(&str).map_err(|_| ())
}

pub fn parse_query(str: &str) -> Result<Goals, ()> {
    let parser = grammar::QueryParser::new();
    parser
        .parse(&str)
        .map_err(|_| ())
        .and_then(|t| t.as_goals().ok_or(()))
}

pub fn parse_queries(str: &str) -> Result<Vec<Goals>, ()> {
    let parser = grammar::QueriesParser::new();
    parser
        .parse(&str)
        .map_err(|_| ())
        .and_then(|t| t.into_iter().map(|t| t.as_goals().ok_or(())).collect())
}

#[cfg(test)]
mod tests {
    use super::parse_expr;
    #[test]
    fn grammar() {
        assert!(parse_expr("22").is_ok());
        assert!(parse_expr("true").is_ok());
        assert!(parse_expr("false").is_ok());

        assert!(parse_expr("plus(1,2,3)").is_ok());
        assert!(parse_expr("plus(1,2").is_err());

        assert!(parse_expr("plus(1 2 3)").is_err());

        assert!(parse_expr("X(1,2)").is_err());

        assert!(parse_expr("X").is_ok());
        assert!(parse_expr("f(X)").is_ok());

        assert!(parse_expr("2+X").is_ok());
        assert!(parse_expr("2+X*2").is_ok());

        assert!(parse_expr("(2+X)*2").is_ok());

        assert!(parse_expr("g(f(2+X))").is_ok());

        assert!(parse_expr("test :- a").is_ok());

        assert!(parse_expr("f((a,b))").is_ok());

        assert!(parse_expr("[]").is_ok());
        assert!(parse_expr("[1,2,3]").is_ok());
        assert!(parse_expr("[1+X,2,3]").is_ok());
        assert!(parse_expr("[1+X,2,]").is_err());
        assert!(parse_expr("[[1],[2],[3]]").is_ok());
    }
}
