use std::{marker::PhantomData, ops::Range};

#[derive(Debug)]
pub struct Tokens<'a, T>
{    
    src: &'a str,
    next: usize,

    _kind: PhantomData<T>,
}

/// Some token extracted from source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a, T>
{
    kind: T,
    span: Range<usize>,
    text: &'a str,
}

/// Implemented by `enum` over types of tokens. Don't try to implement this
/// yourself, use the `tokens!` macro.
pub trait TokenKind: Sized + Copy
{
    /// Get some token at the beginning of the string `src`, or `None`
    fn find(src: &str) -> Option<(Self, regex::Match<'_>)>;
}

#[macro_export]
macro_rules! tokens
{
    {
        $(#[$outer:meta])*
        $vis:vis enum $enum_name:ident
        {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $name:ident = $regex:literal
            ),* $(,)?
        }
    } =>
    {
        $(#[$outer])*
        $vis enum $enum_name
        {
            $(
                $(#[$inner $($args)*])*
                $name
            ),*
        }

        impl crate::lexer::TokenKind for $enum_name
        {
            #[allow(non_upper_case_globals)]
            fn find(src: &str) -> Option<(Self, regex::Match<'_>)>
            {
                // Precompute the regex patterns
                lazy_static::lazy_static!
                {$(
                    static ref $name: regex::Regex = regex::Regex::new(&*("^(".to_owned() + $regex + ")")).unwrap();
                )*}
                $(
                    // Go through every token kind's regex
                    if let Some(mat) = $name.find(src)
                    {
                        return Some(($enum_name::$name, mat));
                    }
                )*
                // None found
                return None
            }
        }
    };
}

impl<'a, T> Tokens<'a, T>
{
    pub fn new(src: &'a str) -> Self
    {
        Self { next: 0, src, _kind: PhantomData }
    }
}

impl<'a, T: TokenKind> Iterator for Tokens<'a, T>
{
    type Item = Result<Token<'a, T>, ()>;

    fn next(&mut self) -> Option<Self::Item>
    {
        // Exhausted source text
        if self.next == self.src.len()
        {
            return None;
        }

        // 1. Disambiguate tokens by order they're evaluated
        // 2. Pattern will only match start of line(via '^')
        if let Some((kind, mat)) = T::find(&self.src[self.next..])
        {
            // Next token
            self.next += mat.end();

            return Some(Ok(Token
            {
                kind,
                span: self.next - mat.end()..self.next,
                text: mat.as_str(),
            }));
        }
        // Unknown token, TODO: skip unknown, continue with other tokens
        self.next = self.src.len();
        Some(Err(()))
    }
}

impl<'a, T: TokenKind> Token<'a, T>
{
    pub fn kind(&self) -> T
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