use meowc::lexer::{token::Token, token::Token::*, Lexer};
use std::path::Path;

fn test_tokens(input: &str, mut expected: Vec<Token>) {
    let mut tokens = Vec::new();
    let lexer = Lexer::new(Path::new("main.mw"), input).collect::<Vec<_>>();
    lexer
        .iter()
        .for_each(|(token, _)| tokens.push(token.clone()));

    // impicit Eof
    expected.push(Eof);

    assert_eq!(tokens, expected)
}

#[test]
fn individual_tokens() {
    test_tokens(
        r#"ident "string" 0 0.0 true false and else for fun if import let match mut not or return while = == != > < >= <= + - * / ( ) [ ] { } ; , . & | ! \"#,
        vec![
            Ident("ident".to_string()),
            String("string".to_string()),
            Int("0".to_string()),
            Float("0.0".to_string()),
            Bool("true".to_string()),
            Bool("false".to_string()),
            And,
            Else,
            For,
            Fun,
            If,
            Import,
            Let,
            Match,
            Mut,
            Not,
            Or,
            Return,
            While,
            Eq,
            EqEq,
            Neq,
            Gt,
            Lt,
            GtEq,
            LtEq,
            Plus,
            Minus,
            Star,
            Slash,
            OpenParen,
            CloseParen,
            OpenBracket,
            CloseBracket,
            OpenBrace,
            CloseBrace,
            Semi,
            Comma,
            Dot,
            Amp,
            Pipe,
            Bang,
            Backslash,
        ],
    )
}
