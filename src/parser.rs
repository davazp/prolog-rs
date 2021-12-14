lalrpop_mod!(pub grammar);

mod tests {
    use super::grammar;

    #[test]
    fn grammar() {
        assert!(grammar::ExprParser::new().parse("22").is_ok());
        assert!(grammar::ExprParser::new().parse("true").is_ok());
        assert!(grammar::ExprParser::new().parse("false").is_ok());

        assert!(grammar::ExprParser::new().parse("plus(1,2,3)").is_ok());
        assert!(grammar::ExprParser::new().parse("plus(1,2,3,)").is_ok());
        assert!(grammar::ExprParser::new().parse("plus(1,2").is_err());

        assert!(grammar::ExprParser::new().parse("X(1,2)").is_err());

        assert!(grammar::ExprParser::new().parse("X").is_ok());
        assert!(grammar::ExprParser::new().parse("f(X)").is_ok());

        assert!(grammar::ExprParser::new().parse("2+X").is_ok());
        assert!(grammar::ExprParser::new().parse("2+X*2").is_ok());

        assert!(grammar::ExprParser::new().parse("(2+X)*2").is_ok());

        assert!(grammar::ExprParser::new().parse("g(f(2+X))").is_ok());

        assert!(grammar::ExprParser::new().parse("test :- a").is_ok());

        assert!(grammar::ExprParser::new().parse("f((a,b))").is_ok());
    }
}
