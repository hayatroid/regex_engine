use std::mem::take;

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

fn fold_or(mut seq_or: Vec<AST>) -> Option<AST> {
    if seq_or.len() > 1 {
        let mut ast = seq_or.pop().unwrap();
        seq_or.reverse();
        for s in seq_or {
            ast = AST::Or(Box::new(s), Box::new(ast));
        }
        Some(ast)
    } else {
        seq_or.pop()
    }
}

pub fn parse(expr: &str) -> Result<AST, ParseError> {
    enum ParseState {
        Char,
        Escape,
    }
    let mut seq = Vec::new();
    let mut seq_or = Vec::new();
    let mut stack = Vec::new();
    let mut state = ParseState::Char;
    for (i, c) in expr.chars().enumerate() {
        match &state {
            ParseState::Char => match c {
                '+' => parse_plus_star_question(&mut seq, PSQ::Plus, i)?,
                '*' => parse_plus_star_question(&mut seq, PSQ::Star, i)?,
                '?' => parse_plus_star_question(&mut seq, PSQ::Question, i)?,
                '(' => {
                    let prev = take(&mut seq);
                    let prev_or = take(&mut seq_or);
                    stack.push((prev, prev_or));
                }
                ')' => {
                    if let Some((mut prev, prev_or)) = stack.pop() {
                        if !seq.is_empty() {
                            seq_or.push(AST::Seq(seq));
                        }
                        if let Some(ast) = fold_or(seq_or) {
                            prev.push(ast);
                        }
                        seq = prev;
                        seq_or = prev_or;
                    } else {
                        return Err(ParseError::InvalidRightParen(i));
                    }
                }
                '|' => {
                    if seq.is_empty() {
                        return Err(ParseError::NoPrev(i));
                    } else {
                        let prev = take(&mut seq);
                        seq_or.push(AST::Seq(prev));
                    }
                }
                '\\' => state = ParseState::Escape,
                _ => seq.push(AST::Char(c)),
            },
            ParseState::Escape => {
                let ast = parse_escape(i, c)?;
                seq.push(ast);
                state = ParseState::Char;
            }
        }
    }
    if !stack.is_empty() {
        return Err(ParseError::NoRightParen);
    }
    if !seq.is_empty() {
        seq_or.push(AST::Seq(seq));
    }
    if let Some(ast) = fold_or(seq_or) {
        Ok(ast)
    } else {
        Err(ParseError::Empty)
    }
}
