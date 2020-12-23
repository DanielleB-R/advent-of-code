use std::{collections::VecDeque, str::FromStr};

use itertools::{Itertools, PeekingNext};
use logos::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,

    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Number(isize),

    #[token("+")]
    Plus,

    #[token("*")]
    Star,

    #[token("(")]
    Open,

    #[token(")")]
    Close,
}

pub fn parse_tokens(s: &str) -> Vec<Token> {
    Token::lexer(s).collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn operate(self, lhs: isize, rhs: isize) -> isize {
        match self {
            Self::Add => lhs + rhs,
            Self::Multiply => lhs * rhs,
        }
    }
}

pub fn evaluate_tokens(tokens: &[Token]) -> isize {
    let mut stack: Vec<(isize, Operator)> = Default::default();
    let mut accumulator = 0;
    let mut operator = Operator::Add;

    for token in tokens {
        match token {
            Token::Number(n) => {
                accumulator = dbg!(operator.operate(accumulator, *n));
            }
            Token::Plus => {
                operator = Operator::Add;
            }
            Token::Star => {
                operator = Operator::Multiply;
            }
            Token::Open => {
                stack.push((accumulator, operator));
                accumulator = 0;
                operator = Operator::Add;
            }
            Token::Close => {
                let (old_accumulator, old_operator) = stack.pop().unwrap();
                accumulator = old_operator.operate(old_accumulator, accumulator);
                operator = Operator::Add;
            }
            _ => panic!("Error!"),
        }
    }
    if stack.len() > 0 {
        panic!("unmatched!");
    }
    accumulator
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(isize),
    Sum(Box<Expression>, Box<Expression>),
    Product(Box<Expression>, Box<Expression>),
}

fn take_bracketed_expression(input: &mut VecDeque<Token>) -> VecDeque<Token> {
    let mut depth = 0;
    let mut result = VecDeque::new();

    while !(input[0] == Token::Close && depth == 0) {
        let token = input.pop_front().unwrap();
        if token == Token::Open {
            depth += 1;
        }
        if token == Token::Close {
            depth -= 1;
        }
        result.push_back(token);
    }
    // Remove the final close bracket
    input.pop_front();
    result
}

impl Expression {
    pub fn evaluate(self) -> isize {
        match self {
            Self::Number(n) => n,
            Self::Sum(l, r) => l.evaluate() + r.evaluate(),
            Self::Product(l, r) => l.evaluate() * r.evaluate(),
        }
    }

    pub fn parse_from_tokens(mut input: VecDeque<Token>) -> Self {
        let lhs = match input.pop_front().unwrap() {
            Token::Open => Self::parse_from_tokens(take_bracketed_expression(&mut input)),
            Token::Number(n) => Self::Number(n),
            _ => panic!("invalid token"),
        };

        let operator = match input.pop_front() {
            Some(Token::Plus) => Token::Plus,
            Some(Token::Star) => Token::Star,
            // If we're at the end then the LHS is the current expression
            None => return lhs,
            _ => panic!("invalid operator"),
        };

        let rhs = Self::parse_from_tokens(input);

        match operator {
            Token::Plus => Self::Sum(Box::new(lhs), Box::new(rhs)),
            Token::Star => Self::Product(Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        }
    }
}

pub fn parse_expression(input: &str) -> Expression {
    let mut tokens = Token::lexer(&input).collect_vec();
    tokens.reverse();

    Expression::parse_from_tokens(tokens.into())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_token_evaluation() {
        assert_eq!(evaluate_tokens(&parse_tokens("2 * 3 + (4 * 5)")), 26);
    }

    #[test]
    fn test_take_bracketed_expression() {
        let mut tokens: VecDeque<_> = Token::lexer("(1 + (2 + 3)) + 5").collect();
        tokens.pop_front();
        let inner_tokens = take_bracketed_expression(&mut tokens);
        assert_eq!(tokens, VecDeque::from(vec![Token::Plus, Token::Number(5)]));
        assert_eq!(
            inner_tokens,
            VecDeque::from(vec![
                Token::Number(1),
                Token::Plus,
                Token::Open,
                Token::Number(2),
                Token::Plus,
                Token::Number(3),
                Token::Close
            ])
        )
    }

    // #[test]
    // fn test_expressions() {
    //     let expression = parse_expression("2 * 3 + (4 * 5)");
    //     // assert_eq!(expression, Expression::);
    //     assert_eq!(expression.evaluate(), 26);
    // }
}
