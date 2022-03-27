use nom::branch::alt;
use nom::character::complete::{char, multispace0};
use nom::combinator::map;
use nom::sequence::{delimited, preceded, tuple};
use nom::{Finish, IResult};

use crate::parse::{Expression, Node, Operator};

/// Parse literal. like "123", "0.3"
fn literal_num(input: &str) -> IResult<&str, Expression> {
    map(
        preceded(multispace0, nom::number::complete::double),
        Expression::Literal,
    )(input)
}

/// Parse '+' or '-' operator.
fn op_add(input: &str) -> IResult<&str, Operator> {
    map(
        preceded(multispace0, alt((char('+'), char('-')))),
        |op: char| match op {
            '+' => Operator::Add,
            '-' => Operator::Sub,
            _ => unreachable!(),
        },
    )(input)
}

/// Parse '*' or '/' operator.
fn op_mul(input: &str) -> IResult<&str, Operator> {
    // FIXME: ov_lvl_ func duplication

    map(
        preceded(multispace0, alt((char('*'), char('/')))),
        |op: char| match op {
            '*' => Operator::Mul,
            '/' => Operator::Div,
            _ => unreachable!(),
        },
    )(input)
}

/// Parse nested expression. like '( 10 * 20 )'
fn nest(input: &str) -> IResult<&str, Expression> {
    delimited(
        preceded(multispace0, char('(')),
        add,
        preceded(multispace0, char(')')),
    )(input)
}

fn lit_or_nest(input: &str) -> IResult<&str, Expression> {
    alt((literal_num, nest))(input)
}

/// Parse mul expression.
fn mul(input: &str) -> IResult<&str, Expression> {
    alt((
        map(
            tuple((lit_or_nest, op_mul, mul)),
            |(left, operator, right)| Expression::ast(Node::new(left, operator, right)),
        ),
        lit_or_nest,
    ))(input)
}

fn add(input: &str) -> IResult<&str, Expression> {
    alt((
        map(tuple((mul, op_add, add)), |(left, operator, right)| {
            Expression::ast(Node::new(left, operator, right))
        }),
        mul,
    ))(input)
}

pub(crate) fn parse_line(input: &str) -> Result<(&str, Expression), nom::error::Error<&str>> {
    add(input).finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! lit {
        ($lit:expr) => {
            Expression::from($lit)
        };
    }

    macro_rules! node {
        ($left:expr, $op:expr, $right:expr) => {
            Expression::ast(Node::new(
                Expression::from($left),
                Operator::try_from($op).unwrap(),
                Expression::from($right),
            ))
        };
    }

    #[test]
    fn parse_literal() {
        assert_eq!(literal_num("4"), Ok(("", lit!(4.))));
        assert_eq!(literal_num("  4"), Ok(("", lit!(4.))));
        assert_eq!(literal_num("4  "), Ok(("  ", lit!(4.))));
        assert_eq!(literal_num("-4"), Ok(("", lit!(-4.))));
    }

    #[test]
    fn parse_operator() {
        assert_eq!(op_add("+"), Ok(("", Operator::Add)));
        assert_eq!(op_add("  +"), Ok(("", Operator::Add)));
        assert_eq!(op_add("  + "), Ok((" ", Operator::Add)));

        assert_eq!(op_add("-"), Ok(("", Operator::Sub)));
        assert_eq!(op_add("  -"), Ok(("", Operator::Sub)));
        assert_eq!(op_add("  - "), Ok((" ", Operator::Sub)));

        assert_eq!(op_mul("*"), Ok(("", Operator::Mul)));
        assert_eq!(op_mul("  *"), Ok(("", Operator::Mul)));
        assert_eq!(op_mul("  * "), Ok((" ", Operator::Mul)));

        assert_eq!(op_mul("/"), Ok(("", Operator::Div)));
        assert_eq!(op_mul("  /"), Ok(("", Operator::Div)));
        assert_eq!(op_mul("  / "), Ok((" ", Operator::Div)));
    }

    #[test]
    fn parse_nest() {
        assert_eq!(nest("(100)"), Ok(("", lit!(100.))));
        assert_eq!(nest(" ( 100 )"), Ok(("", lit!(100.))));
        assert_eq!(nest(" ( 100 ) "), Ok((" ", lit!(100.))));
    }

    #[test]
    fn parse_mul() {
        assert_eq!(mul("10 * 20"), Ok(("", node!(10, '*', 20))));
        assert_eq!(mul("10*20"), Ok(("", node!(10, '*', 20))));
        assert_eq!(mul("10*20 "), Ok((" ", node!(10, '*', 20))));

        assert_eq!(mul("2 * 3 * 4"), Ok(("", node!(2, '*', node!(3, '*', 4)))));
        assert_eq!(
            mul("2 * 3 * 4 * 5"),
            Ok(("", node!(2, '*', node!(3, '*', node!(4, '*', 5)))))
        );
    }

    #[test]
    fn parse_expression() {
        assert_eq!(add("1 + 2 + 3"), Ok(("", node!(1, '+', node!(2, '+', 3)))),);
        assert_eq!(
            add("1 + 2 + 3 + 4"),
            Ok(("", node!(1, '+', node!(2, '+', node!(3, '+', 4))))),
        );
        assert_eq!(add("100"), Ok(("", lit!(100))));
        assert_eq!(add("2 * 3"), Ok(("", node!(2, '*', 3))));
        assert_eq!(add("2 * 3 + 4"), Ok(("", node!(node!(2, '*', 3), '+', 4))));
        assert_eq!(
            add("2 * 3 * 4 + 5"),
            Ok(("", node!(node!(2, '*', node!(3, '*', 4)), '+', 5)))
        );
        assert_eq!(
            add("2 / 3 / 4 - 5"),
            Ok(("", node!(node!(2, '/', node!(3, '/', 4)), '-', 5)))
        );
        assert_eq!(
            add("(1 + 2) * (4 + 5)"),
            Ok(("", node!(node!(1, '+', 2), '*', node!(4, '+', 5))))
        );
    }
}
