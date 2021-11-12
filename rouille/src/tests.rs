use std::ops::Range;

use crate::lexer::Tokens;
use crate::TokenKind;

fn check(iter: &mut Tokens<TokenKind>, kind: TokenKind, text: &str, span: impl Into<Option<Range<usize>>>)
{
    let token = iter
        .next()
        .unwrap()
        .unwrap();

    assert_eq!(token.kind(), kind);
    assert_eq!(token.as_str(), text);
    
    // On some tests I'm too lazy to check the span
    if let Some(span) = span.into()
    {
        assert_eq!(token.span(), span);
    }
    dbg!(token);
}

#[test]
fn test_lexer_00()
{
    let mut tokens = Tokens::new("hello world");

    check(&mut tokens, TokenKind::Ident, "hello", None);
    check(&mut tokens, TokenKind::Whitespace, " ", None);
    check(&mut tokens, TokenKind::Ident, "world", None);
}

#[test]
fn test_lexer_01()
{
    let mut tokens = Tokens::new("x return world fn");

    check(&mut tokens, TokenKind::Ident, "x", None);
    check(&mut tokens, TokenKind::Whitespace, " ", None);
    check(&mut tokens, TokenKind::Return, "return", None);
    check(&mut tokens, TokenKind::Whitespace, " ", None);
    check(&mut tokens, TokenKind::Ident, "world", None);
    check(&mut tokens, TokenKind::Whitespace, " ", None);
    check(&mut tokens, TokenKind::Fn, "fn", None);
}

#[test]
fn test_lexer_02()
{
    let mut tokens = Tokens::new("let x = 42");

    check(&mut tokens, TokenKind::Let, "let", None);
    check(&mut tokens, TokenKind::Whitespace, " ", None);
    check(&mut tokens, TokenKind::Ident, "x", None);
    check(&mut tokens, TokenKind::Whitespace, " ", None);
    check(&mut tokens, TokenKind::Equals, "=", None);
    check(&mut tokens, TokenKind::Whitespace, " ", None);
    check(&mut tokens, TokenKind::Num, "42", None);
}

#[test]
fn test_lexer_03()
{
    let mut tokens = Tokens::new("
    let x = 42
    let y = 50.20
    ");

    check(&mut tokens, TokenKind::Whitespace, "\n    ", None);

    check(&mut tokens, TokenKind::Let, "let", None);
    check(&mut tokens, TokenKind::Whitespace, " ", None);
    check(&mut tokens, TokenKind::Ident, "x", None);
    check(&mut tokens, TokenKind::Whitespace, " ", None);
    check(&mut tokens, TokenKind::Equals, "=", None);
    check(&mut tokens, TokenKind::Whitespace, " ", None);
    check(&mut tokens, TokenKind::Num, "42", None);

    check(&mut tokens, TokenKind::Whitespace, "\n    ", None);

    check(&mut tokens, TokenKind::Let, "let", None);
    check(&mut tokens, TokenKind::Whitespace, " ", None);
    check(&mut tokens, TokenKind::Ident, "y", None);
    check(&mut tokens, TokenKind::Whitespace, " ", None);
    check(&mut tokens, TokenKind::Equals, "=", None);
    check(&mut tokens, TokenKind::Whitespace, " ", None);
    check(&mut tokens, TokenKind::Num, "50.20", None);
}

#[test]
fn test_lexer_04()
{
    let tokens = Tokens::<TokenKind>::new("let foo = (42 + 50) / 1");
    let tokens = tokens
        .map(|tk| tk.unwrap().as_str())
        .collect::<Vec<_>>();
    
    assert_eq!(tokens, vec!
    [
        "let", " ", "foo", " ", "=",
        " ", "(", "42", " ", "+", " ",
        "50", ")", " ", "/", " ", "1"
    ]);
}