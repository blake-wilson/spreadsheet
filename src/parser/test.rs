#[cfg(test)]
mod tests {
    pub use super::super::lexer::*;
    pub use super::super::parser::*;
    use std::collections::VecDeque;

    #[test]
    fn test_parse_number() -> Result<(), &'static str> {
        let tokens = lex(&"30".to_string())?;

        assert_eq!(1, tokens.len());
        assert_eq!(
            &Token {
                kind: TokenKind::Number,
                val: "30".to_string(),
            },
            tokens.get(0).unwrap()
        );

        let tokens = lex(&"30 40 50".to_string())?;
        assert_eq!(3, tokens.len());

        assert_eq!(
            &Token {
                kind: TokenKind::Number,
                val: "30".to_string(),
            },
            tokens.get(0).unwrap()
        );
        assert_eq!(
            &Token {
                kind: TokenKind::Number,
                val: "40".to_string(),
            },
            tokens.get(1).unwrap()
        );
        assert_eq!(
            &Token {
                kind: TokenKind::Number,
                val: "50".to_string(),
            },
            tokens.get(2).unwrap()
        );
        Ok(())
    }

    #[test]
    fn test_lex_id() -> Result<(), &'static str> {
        let tokens = lex(&"this_id".to_string())?;

        assert_eq!(1, tokens.len());
        assert_eq!(
            &Token {
                kind: TokenKind::ID,
                val: "this_id".to_string(),
            },
            tokens.get(0).unwrap()
        );
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<(), Error> {
        let tokens = &mut VecDeque::from(vec![
            Token {
                kind: TokenKind::Number,
                val: "30".to_string(),
            },
            Token {
                kind: TokenKind::BinaryExpr,
                val: "+".to_string(),
            },
            Token {
                kind: TokenKind::Number,
                val: "40".to_string(),
            },
        ]);
        let node = parse_internal(tokens)?;

        assert_eq!(
            node,
            ASTNode::BinaryExpr {
                op: Operator::Add,
                lhs: Box::new(ASTNode::Number(30f64)),
                rhs: Box::new(ASTNode::Number(40f64)),
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_function() -> Result<(), Error> {
        let tokens = &mut VecDeque::from(vec![
            Token {
                kind: TokenKind::ID,
                val: "ABC".to_string(),
            },
            Token {
                kind: TokenKind::LParen,
                val: "(".to_string(),
            },
            Token {
                kind: TokenKind::Number,
                val: "20".to_string(),
            },
            Token {
                kind: TokenKind::BinaryExpr,
                val: "+".to_string(),
            },
            Token {
                kind: TokenKind::Number,
                val: "40".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                val: ",".to_string(),
            },
            Token {
                kind: TokenKind::Number,
                val: "60".to_string(),
            },
            Token {
                kind: TokenKind::RParen,
                val: ")".to_string(),
            },
        ]);
        let node = parse_internal(tokens)?;

        assert_eq!(
            node,
            ASTNode::Function {
                name: "ABC".to_string(),
                args: vec![
                    Box::new(ASTNode::BinaryExpr {
                        op: Operator::Add,
                        lhs: Box::new(ASTNode::Number(20f64)),
                        rhs: Box::new(ASTNode::Number(40f64)),
                    }),
                    Box::new(ASTNode::Number(60f64)),
                ],
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_cell_ref() -> Result<(), Error> {
        let tokens = &mut VecDeque::from(vec![Token {
            kind: TokenKind::ID,
            val: "B1".to_string(),
        }]);
        let node = parse_internal(tokens)?;
        assert_eq!(node, ASTNode::Ref(CellRef { col: 1, row: 0 }));
        Ok(())
    }
}
