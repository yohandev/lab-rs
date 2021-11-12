use std::ops::Range;

use regex::Regex;

#[derive(Debug)]
pub struct Tokens<'a>
{
    pat: Box<[(TokenKind, Regex)]>,
    
    src: &'a str,
    next: usize,
}

/// Some token extracted from source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a>
{
    kind: TokenKind,
    span: Range<usize>,
    text: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    Num,        // Integer or floating point
}

impl<'a> Tokens<'a>
{
    pub fn new(src: &'a str) -> Self
    {
        let pat = Box::new([
            (TokenKind::Whitespace, Regex::new(r"^( +)").unwrap()),
            (TokenKind::LBrace, Regex::new(r"^{").unwrap()),
            (TokenKind::RBrace, Regex::new(r"^}").unwrap()),
            (TokenKind::LParen, Regex::new(r"^\(").unwrap()),
            (TokenKind::RParen, Regex::new(r"^\)").unwrap()),

            (TokenKind::Fn, Regex::new(r"^fn").unwrap()),
            (TokenKind::Return, Regex::new(r"^return").unwrap()),

            (TokenKind::Plus, Regex::new(r"^\+").unwrap()),
            (TokenKind::Minus, Regex::new(r"^-").unwrap()),
            (TokenKind::Asterisk, Regex::new(r"^\*").unwrap()),
            (TokenKind::Slash, Regex::new(r"^\/").unwrap()),
            
            (TokenKind::Ident, Regex::new(r"^[A-Za-z][A-Za-z0-9]+").unwrap()),
            (TokenKind::Num, Regex::new(r"^([0-9]+)(\.[0-9]+)?").unwrap()),
        ]);

        Self { next: 0, pat, src }
    }
}

impl<'a> Iterator for Tokens<'a>
{
    type Item = Result<Token<'a>, ()>;

    fn next(&mut self) -> Option<Self::Item>
    {
        // Exhausted source text
        if self.next == self.src.len()
        {
            return None;
        }

        let src = &self.src[self.next..];

        // Disambiguate tokens by order they're evaluated
        for (kind, pat) in &*self.pat
        {
            // Pattern will only match start of line(via '\A')
            let mat = pat.find(src);

            if let Some(mat) = mat
            {
                // Next token
                self.next = mat.end();

                return Some(Ok(Token
                {
                    kind: *kind,
                    span: mat.range(),
                    text: mat.as_str(),
                }));
            }
        }
        // Unknown token, TODO: skip unknown, continue with other tokens
        Some(Err(()))
    }
}