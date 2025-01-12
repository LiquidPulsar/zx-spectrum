#[cfg(test)]
mod tests {
    use crate::parser::{parse_tools::{ident, NomErr}, Expr, Instr};

    fn success <'a, T> (instr: T) -> Result<(&'a str, T), nom::Err<NomErr<'a>>> {
        Ok(("", instr))
    }

    #[test]
    fn test_parse_expr() {
        assert_eq!(Expr::parse("42"), Ok(("", Expr::Int(42))));
        assert_eq!(Expr::parse("x"), Ok(("", ident("x"))));
    }

    #[test]
    fn test_parse_instr() {
        assert_eq!(
            Instr::parse("print 42"),
            success(Instr::Print(vec![Expr::Int(42)]))
        );
        assert_eq!(
            Instr::parse("let x = 42"),
            success(Instr::Assign(ident("x"), Expr::Int(42)))
        );
        // Test malformed
        assert!(Instr::parse("let x =").is_err());
    }

    #[test]
    fn test_precedence() {
        assert_eq!(
            Expr::parse("1 + 2 * 3"),
            success(
                Expr::Add(
                    Box::new(Expr::Int(1)),
                    Box::new(Expr::Mul(Box::new(Expr::Int(2)), Box::new(Expr::Int(3))))
                )
            )
        );
        assert_eq!(
            Expr::parse("1 * 2 + 3"),
            success(
                Expr::Add(
                    Box::new(Expr::Mul(Box::new(Expr::Int(1)), Box::new(Expr::Int(2)))),
                    Box::new(Expr::Int(3))
                )
            )
        );
    }

    #[test]
    fn test_precedence_brackets() {
        assert_eq!(
            Expr::parse("(1 + 2) * 3"),
            success(
                Expr::Mul(
                    Box::new(Expr::Add(Box::new(Expr::Int(1)), Box::new(Expr::Int(2)))),
                    Box::new(Expr::Int(3))
                )
            )
        );
        assert_eq!(
            Expr::parse("1 * (2 + 3)"),
            success(
                Expr::Mul(
                    Box::new(Expr::Int(1)),
                    Box::new(Expr::Add(Box::new(Expr::Int(2)), Box::new(Expr::Int(3))))
                )
            )
        );
    }

    #[test]
    fn test_string() {
        assert_eq!(Expr::parse("\"hello\""), success(Expr::String("hello")));

        assert_eq!(
            Expr::parse("\"hello\" + \"world\""),
            success(
                Expr::Add(
                    Box::new(Expr::String("hello")),
                    Box::new(Expr::String("world"))
                )
            )
        );

        assert_eq!(
            Instr::parse("PRINT 1, \"world!\""),
            success(Instr::Print(vec![Expr::Int(1), Expr::String("world!")]))
        );

        assert_eq!(
            Instr::parse("PRINT \"Hello,\", \"world!\""),
            success(
                Instr::Print(vec![Expr::String("Hello,"), Expr::String("world!")])
            )
        );
    }

    #[test]
    fn test_multi_instr() {
        assert_eq!(
            Instr::parse("PRINT 1:PRINT 2"),
            success(
                Instr::Multi(vec![
                    Instr::Print(vec![Expr::Int(1)]),
                    Instr::Print(vec![Expr::Int(2)])
                ])
            )
        );
    }

    #[test]
    fn test_operators() {
        assert_eq!(
            Expr::parse("1 > 2"),
            success(Expr::Gt(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))))
        );
        assert_eq!(
            Expr::parse("1 < 2"),
            success(Expr::Lt(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))))
        );
        assert_eq!(
            Expr::parse("1 = 2"),
            success(Expr::Eq(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))))
        );
        assert_eq!(
            Expr::parse("1 >= 2"),
            success(Expr::Ge(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))))
        );
        assert_eq!(
            Expr::parse("1 <= 2"),
            success(Expr::Le(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))))
        );
        assert_eq!(
            Expr::parse("1 <> 2"),
            success(Expr::Ne(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))))
        );
    }
}
