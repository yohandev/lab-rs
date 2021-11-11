# Rouille
A made-up programming language inspired by Rust

## Why?
Without any prior research, I wanted to see if I can design a simple programming language comprised of its lexer, parser, and evaluator. I've written quite a few procedural plugins in the Rust programming language, which dives deep into its own AST, so I have a grasp of how compilers operate at a (very) high level. So, this is a learning project.

**Note**: This is a work in progress

## Plan
My idea was to segment the project into a lexer, which segments source code into tokens that are then fed into the parser which looks up a pre-determined token tree to create statements and expressions, that are then evaluated dynamically. 