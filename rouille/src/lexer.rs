use std::ops::Range;

/// Some token extracted from source code
pub struct Token<'a>
{
    kind: TokenKind,
    span: Range<usize>,
    src: &'a str,
}

pub enum TokenKind
{
    Whitespace, // ' '
    
    LBrace,     // {
    RBrace,     // }
    LParen,     // (
    RParen,     // )

    Fn,         // fn
    Return,     // return
    
    Plus,       // +
    Minus,      // -
    Asterisk,   // *
    Slash,      // /

    Ident,      // Alphanumeric identifier
    Num,        // Integer, TODO: float's
}