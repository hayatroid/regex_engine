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

fn parse_escape(pos: usize, c: char) -> Result<AST, ParseError> {
    match c {
        '\\' | '(' | ')' | '|' | '+' | '*' | '?' => Ok(AST::Char(c)),
        _ => Err(ParseError::InvalidEscape(pos, c)),
    }
}

enum PSQ {
    Plus,
    Star,
    Question,
}

fn parse_plus_star_question(
    seq: &mut Vec<AST>,
    ast_type: PSQ,
    pos: usize,
) -> Result<(), ParseError> {
    if let Some(prev) = seq.pop() {
        let ast = match ast_type {
            PSQ::Plus => AST::Plus(Box::new(prev)),
            PSQ::Star => AST::Star(Box::new(prev)),
            PSQ::Question => AST::Question(Box::new(prev)),
        };
        seq.push(ast);
        Ok(())
    } else {
        Err(ParseError::NoPrev(pos))
    }
}
