use std::cmp::Ordering;
use crate::parser::op::{Data, Operator};
use crate::lexer::{Delimiter, Token, Tokens};

#[derive(Debug)]
pub enum Error {
    NoTokens,
    WrongToken,
    NumberExpected,
    NullToken,
    Stack,
}

pub struct RPN {
    pub data: Vec<Data>,
}

#[derive(PartialEq, Debug)]
pub enum StackItem {
    Op(Operator),
    LP,
}

impl RPN {
    fn iterate_stack(stack: &mut Vec<StackItem>, output: &mut Vec<Data>, op: Operator) {
        while let Some(si) = stack.pop() {
            let StackItem::Op(op_2) = si else {
                stack.push(si);
                break;
            };

            match op_2.have_precedence(&op) {
                Ordering::Equal => {
                    if op.is_left_associative() {
                        output.push(Data::Op(op_2));
                    } else {
                        stack.push(StackItem::Op(op_2));
                        break;
                    }
                }
                Ordering::Less => {
                    stack.push(StackItem::Op(op_2));
                    break;
                }
                Ordering::Greater => {
                    output.push(Data::Op(op_2));
                }
            };
        }
        stack.push(StackItem::Op(op));
    }

    pub fn from<'a>(tokens: Tokens) -> Result<Self, Error> {
        let iter = tokens.tokens.iter();

        let mut stack: Vec<StackItem> = Vec::new();
        let mut output: Vec<Data> = Vec::new();
        let mut last_token: Option<&Token> = None;

        'token_iter: for token in iter {
            match token {
                Token::Delim(_, delim) => match delim {
                    Delimiter::Positive => {
                        let Some(tk) = last_token else {
                            stack.push(StackItem::Op(Operator::Pos));
                            last_token = Some(token);
                            continue 'token_iter;
                        };
                        
                        match tk {
                            Token::Delim(_, _) => stack.push(StackItem::Op(Operator::Pos)),
                            Token::Liter { l: _, r: _ } => RPN::iterate_stack(&mut stack, &mut output, Operator::Sum),
                        }
                    }
                    Delimiter::Negative => {
                        let Some(tk) = last_token else {
                            stack.push(StackItem::Op(Operator::Neg));
                            last_token = Some(token);
                            continue 'token_iter;
                        };
                        
                        match tk {
                            Token::Delim(_, _) => stack.push(StackItem::Op(Operator::Neg)),
                            Token::Liter { l: _, r: _ } => RPN::iterate_stack(&mut stack, &mut output, Operator::Sub),
                        }
                    }
                    Delimiter::Asterisk => {
                        RPN::iterate_stack(&mut stack, &mut output, Operator::Mul)
                    }
                    Delimiter::Slash => RPN::iterate_stack(&mut stack, &mut output, Operator::Div),
                    Delimiter::Exponent => {
                        RPN::iterate_stack(&mut stack, &mut output, Operator::Exp)
                    }
                    Delimiter::Scientific => {
                        RPN::iterate_stack(&mut stack, &mut output, Operator::Sci)
                    }
                    Delimiter::Open => stack.push(StackItem::LP),
                    Delimiter::Close => {
                        while let Some(item) = stack.pop() {
                            match item {
                                StackItem::LP => continue 'token_iter,
                                StackItem::Op(op) => {
                                    output.push(Data::Op(op));
                                }
                            };
                        }
                        panic!("Parentheses are not closed.");
                    }
                },
                Token::Liter { l, r } => {
                    let mut indices = tokens.get_expr().char_indices().map(|(i, _)| i);
                    let start = indices.nth(*l).unwrap_or_default();
                    let end = indices.nth(r - l - 1).unwrap_or(tokens.get_expr().len());

                    let Ok(val) = tokens.get_expr()[start..end].parse() else {
                        panic!(
                            "Could not parse a literal {} at {l} {r}",
                            &tokens.get_expr()[start..end]
                        );
                    };

                    output.push(Data::Val(val));
                }
            }
            last_token = Some(token);
        }

        if !stack.is_empty() {
            while let Some(si) = stack.pop() {
                if let StackItem::Op(op) = si {
                    output.push(Data::Op(op));
                } else {
                    panic!("Parentheses ar not closed on finish.")
                }
            }
        }

        Ok(RPN { data: output })
    }
}
