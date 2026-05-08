use crate::lexer::tk::{Delimiter::*, *};

#[derive(Debug)]
pub enum Error {
    EmptyExpr,
    UnexpChar(String),
    WrongLiteral(String),
}

enum State {
    Ini,
    Lit,
}

use State::{Ini, Lit};

fn is_num(c: char) -> bool {
    "0123456789".contains(c)
}

fn is_dot(c: char) -> bool {
    c == '.'
}

fn is_sp(c: char) -> bool {
    c == ' '
}

fn check_delimiter(c: char) -> Option<Delimiter> {
    match c {
        '+' => Some(Positive),
        '-' => Some(Negative), // ASCII hyphen
        '−' => Some(Negative), // Unicode minus (U+2212)
        '*' => Some(Asterisk),
        '/' => Some(Slash),
        '^' => Some(Exponent),
        'E' => Some(Scientific),
        '(' => Some(Open),
        ')' => Some(Close),
        _ => None,
    }
}
pub struct Tokens<'a> {
    expr: &'a str,
    pub tokens: Vec<Token>,
}

impl<'a> Tokens<'a> {
    pub fn from(expr: &'a str) -> Result<Self, Error> {
        if expr.len() == 0 {
            return Err(Error::EmptyExpr);
        }

        let mut iter = expr.chars().enumerate();

        let mut tokens = Vec::<Token>::new();

        let mut state = Ini;

        let mut buf: usize = 0;
        let mut dot: bool = false;

        loop {
            match state {
                Ini => {
                    let Some((i, c)) = iter.next() else {
                        break Ok(Self { expr, tokens });
                    };

                    if let Some(op) = check_delimiter(c) {
                        tokens.push(Token::Delim(i, op));
                    } else if is_num(c) {
                        buf = i;
                        state = Lit;
                    } else if !is_sp(c) {
                        break Err(Error::UnexpChar(format!(
                            "Wrong character {c} at {i} position."
                        )));
                    }
                }
                Lit => {
                    let Some((i, c)) = iter.next() else {
                        tokens.push(Token::Liter {
                            l: buf,
                            r: expr.len(),
                        });
                        break Ok(Self { expr, tokens });
                    };

                    if !is_num(c) && !is_dot(c) {
                        dot = false;
                        tokens.push(Token::Liter { l: buf, r: i });
                        state = Ini;

                        if let Some(op) = check_delimiter(c) {
                            tokens.push(Token::Delim(i, op));
                        } else if !is_sp(c) {
                            let mut indices = expr.char_indices().map(|(i, _)| i);
                            let l = indices.nth(buf).unwrap_or_default();
                            let r = indices.nth(i - buf - 1).unwrap_or(expr.len());
                            break Err(Error::WrongLiteral(format!(
                                "Wrong literal {} at {}",
                                &expr[l..r],
                                i
                            )));
                        }
                    } else if is_dot(c) && dot {
                        break Err(Error::WrongLiteral(String::from("Additional dot added.")));
                    } else if is_dot(c) {
                        dot = true;
                    }
                }
            }
        }
    }

    pub fn get_expr(&self) -> &str {
        self.expr
    }
}
