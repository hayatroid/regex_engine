use thiserror::Error;

#[derive(Debug)]
pub enum AST {
    Char(char),
    Plus(Box<AST>),
    Star(Box<AST>),
    Question(Box<AST>),
    Or(Box<AST>, Box<AST>),
    Seq(Vec<AST>),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("ParseError: invalid escape: pos = {0}, char = '{1}'")]
    InvalidEscape(usize, char),
    #[error("ParseError: invalid right parenthesis: pos = {0}")]
    InvalidRightParen(usize),
    #[error("ParseError: no previous expression: pos = {0}")]
    NoPrev(usize),
    #[error("ParseError: no right parenthesis")]
    NoRightParen,
    #[error("ParseError: empty expression")]
    Empty,
}
