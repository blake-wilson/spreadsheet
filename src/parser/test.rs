#[cfg(test)]
mod tests {
    pub use super::super::lexer::*;

    #[test]
    fn test_parse_number() {
        let input = "30";
        let tokens = lex(&input);

        assert_eq!(1,tokens.len());
        assert_eq!(&Token{
            kind: TokenKind::Number,
            val: "30".to_string(),
        },tokens.get(0).unwrap());
    }
}
