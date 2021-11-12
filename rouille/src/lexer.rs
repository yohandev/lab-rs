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
    Let,        // let
    Return,     // return
    
    Plus,       // +
    Minus,      // -
    Asterisk,   // *
    Slash,      // /
    Equals,     // =

    Ident,      // Alphanumeric identifier
    Num,        // Integer or floating point
}

impl<'a> Tokens<'a>
{
    pub fn new(src: &'a str) -> Self
    {
        let pat = Box::new([
            (TokenKind::Whitespace, Regex::new(r"^([ \n\t\f]+)").unwrap()),
            (TokenKind::LBrace, Regex::new(r"^\{").unwrap()),
            (TokenKind::RBrace, Regex::new(r"^\}").unwrap()),
            (TokenKind::LParen, Regex::new(r"^\(").unwrap()),
            (TokenKind::RParen, Regex::new(r"^\)").unwrap()),

            (TokenKind::Fn, Regex::new(r"^fn").unwrap()),
            (TokenKind::Let, Regex::new(r"^let").unwrap()),
            (TokenKind::Return, Regex::new(r"^return").unwrap()),

            (TokenKind::Plus, Regex::new(r"^\+").unwrap()),
            (TokenKind::Minus, Regex::new(r"^-").unwrap()),
            (TokenKind::Asterisk, Regex::new(r"^\*").unwrap()),
            (TokenKind::Slash, Regex::new(r"^/").unwrap()),
            (TokenKind::Equals, Regex::new(r"^=").unwrap()),
            
            (TokenKind::Ident, Regex::new(r"^[A-Za-z]+([A-Za-z0-9]+)?").unwrap()),
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

        // Disambiguate tokens by order they're evaluated
        for (kind, pat) in &*self.pat
        {
            // Pattern will only match start of line(via '^')
            let mat = pat.find(&self.src[self.next..]);

            if let Some(mat) = mat
            {
                // Next token
                self.next += mat.end();

                return Some(Ok(Token
                {
                    kind: *kind,
                    span: self.next - mat.end()..self.next,
                    text: mat.as_str(),
                }));
            }
        }
        // Unknown token, TODO: skip unknown, continue with other tokens
        self.next = self.src.len();
        Some(Err(()))
    }
}

impl<'a> Token<'a>
{
    pub fn kind(&self) -> TokenKind
    {
        self.kind
    }

    pub fn span(&self) -> Range<usize>
    {
        self.span.clone()
    }

    pub fn as_str(&self) -> &'a str
    {
        self.text
    }
}