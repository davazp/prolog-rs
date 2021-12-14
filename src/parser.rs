lalrpop_mod!(pub grammar);

use crate::terms::Term;

pub fn parse_expr(str: &str) -> Result<Term, ()> {
    let parser = grammar::ExprParser::new();
    parser.parse(&str).map_err(|_| ())
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
        assert!(parse_expr("plus(1,2,3,)").is_ok());
        assert!(parse_expr("plus(1,2").is_err());

        assert!(parse_expr("X(1,2)").is_err());

        assert!(parse_expr("X").is_ok());
        assert!(parse_expr("f(X)").is_ok());

        assert!(parse_expr("2+X").is_ok());
        assert!(parse_expr("2+X*2").is_ok());

        assert!(parse_expr("(2+X)*2").is_ok());

        assert!(parse_expr("g(f(2+X))").is_ok());

        assert!(parse_expr("test :- a").is_ok());

        assert!(parse_expr("f((a,b))").is_ok());
    }
}
