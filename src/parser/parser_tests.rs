#[cfg(test)]
mod tests {
    use crate::parser::{Expr, Instr};

    #[test]
    fn test_parse_expr() {
        assert_eq!(Expr::parse("42"), Ok(("", Expr::Int(42),)),);
        assert_eq!(Expr::parse("x"), Ok(("", Expr::Ident("x"),)),);
    }

    #[test]
    fn test_parse_instr() {
        assert_eq!(
            Instr::parse("print 42"),
            Ok(("", Instr::Print(vec![Expr::Int(42)]),)),
        );
        assert_eq!(
            Instr::parse("let x = 42"),
            Ok(("", Instr::Assign(Expr::Ident("x"), Expr::Int(42)),)),
        );
        // Test malformed
        assert!(Instr::parse("let x =").is_err());
    }

    #[test]
    fn test_precedence() {
        assert_eq!(
            Expr::parse("1 + 2 * 3"),
            Ok((
                "",
                Expr::Add(
                    Box::new(Expr::Int(1)),
                    Box::new(Expr::Mul(Box::new(Expr::Int(2)), Box::new(Expr::Int(3))))
                )
            )),
        );
        assert_eq!(
            Expr::parse("1 * 2 + 3"),
            Ok((
                "",
                Expr::Add(
                    Box::new(Expr::Mul(Box::new(Expr::Int(1)), Box::new(Expr::Int(2)))),
                    Box::new(Expr::Int(3))
                )
            )),
        );
    }

    #[test]
    fn test_string() {
        assert_eq!(
            Expr::parse("\"hello\""),
            Ok(("", Expr::String("hello"),)),
        );

        assert_eq!(
            Expr::parse("\"hello\" + \"world\""),
            Ok((
                "",
                Expr::Add(
                    Box::new(Expr::String("hello")),
                    Box::new(Expr::String("world"))
                )
            )),
        );

        assert_eq!(
            Instr::parse("PRINT 1, \"world!\""),
            Ok((
                "",
                Instr::Print(vec![Expr::Int(1), Expr::String("world!")])
            )),
        );

        assert_eq!(
            Instr::parse("PRINT \"Hello,\", \"world!\""),
            Ok((
                "",
                Instr::Print(vec![Expr::String("Hello,"), Expr::String("world!")])
            )),
        );
    }
}
