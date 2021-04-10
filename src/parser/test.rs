#[cfg(test)]
mod tests {
    pub use super::super::lexer::*;

    #[test]
    fn test_parse_number() {
        let tokens = lex(&"30".to_string());

        assert_eq!(1,tokens.len());
        assert_eq!(&Token{
            kind: TokenKind::Number,
            val: "30".to_string(),
        },tokens.get(0).unwrap());
        
        let tokens = lex(&"30 40 50".to_string());
        assert_eq!(3, tokens.len());

        assert_eq!(&Token{
            kind: TokenKind::Number,
            val: "30".to_string(),
        },tokens.get(0).unwrap());
        assert_eq!(&Token{
            kind: TokenKind::Number,
            val: "40".to_string(),
        },tokens.get(1).unwrap());
        assert_eq!(&Token{
            kind: TokenKind::Number,
            val: "50".to_string(),
        },tokens.get(2).unwrap());
    }
}
