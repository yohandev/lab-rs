#[cfg(test)]
mod tests;

mod lexer;

tokens!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum TokenKind
    {
        Whitespace = r"[ \n\t\f]+",
        Comment = r"#.+",
    
        LBrace = r"\{",
        RBrace = r"\}",
        LParen = r"\(",
        RParen = r"\)",

        Fn = r"fn",
        Let = r"let",
        Return = r"return",
        
        Plus = r"\+",
        Minus = r"\-",
        Asterisk = r"\*",
        Slash = r"/",
        Equals = r"=",

        Ident = r"[A-Za-z]+([A-Za-z0-9]+)?",
        Num = r"([0-9]+)(\.[0-9]+)?",
    }
);