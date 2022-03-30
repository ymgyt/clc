use crate::expression::{Constant, Expression, FuncCall, Lambda, Node, Operator, Variable};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, multispace0};
use nom::combinator::{map, opt, recognize};
use nom::error::ParseError;
use nom::multi::{many0_count, separated_list0, separated_list1};
use nom::number::complete::double;
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::{Finish, IResult, Parser};

/// helper white space consumer.
/// https://docs.rs/nom/latest/nom/recipes/index.html#wrapper-combinators-that-eat-whitespace-before-and-after-a-parser
fn preceded_ws<'a, F: 'a, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    preceded(multispace0, inner)
}

/// Parse constant. like 'pi' or 'e'
fn constant(input: &str) -> IResult<&str, Expression> {
    map(preceded_ws(alt((tag("pi"), tag("e")))), |c: &str| match c {
        "pi" => Constant::Pi,
        "e" => Constant::E,
        _ => unreachable!(),
    })
    .map(Expression::constant)
    .parse(input)
}

/// Parse literal. like "123", "0.3"
fn literal_num(input: &str) -> IResult<&str, Expression> {
    map(preceded_ws(double), Expression::Literal)(input)
}

/// Parse '+' or '-' operator.
fn op_add(input: &str) -> IResult<&str, Operator> {
    map(
        preceded_ws(alt((char('+'), char('-')))),
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
        preceded_ws(alt((char('*'), char('/'), char('^')))),
        |op: char| match op {
            '*' => Operator::Mul,
            '/' => Operator::Div,
            '^' => Operator::Pow,
            _ => unreachable!(),
        },
    )(input)
}

/// Parse variable identifier 'x'.
fn variable_ident(input: &str) -> IResult<&str, Variable> {
    preceded_ws(alpha1).map(Variable::new).parse(input)
}

fn variable_exp(input: &str) -> IResult<&str, Expression> {
    map(variable_ident, Expression::variable)(input)
}

/// Parse lambda arg. '|x| x^2' -> x.
fn lambda_arg(input: &str) -> IResult<&str, Variable> {
    delimited(preceded_ws(tag("|")), variable_ident, preceded_ws(tag("|"))).parse(input)
}

/// Parse lambda body. '|x| x^2' -> x '^' 2.
/// handle '|x| { x^2 }' and '|x| x^2 ' cases.
fn lambda_body(input: &str) -> IResult<&str, Expression> {
    alt((
        delimited(preceded_ws(tag("{")), add, preceded_ws(tag("}"))),
        add,
    ))(input)
}

/// Parse lambda expression. '|x| x^2'.
fn lambda(input: &str) -> IResult<&str, Lambda> {
    pair(lambda_arg, lambda_body)
        .map(|(arg, body)| Lambda::new(arg, body))
        .parse(input)
}

/// Parse function identifier. 'sqrt(100)' -> 'sqrt'
fn func_ident(input: &str) -> IResult<&str, &str> {
    preceded_ws(recognize(pair(
        preceded_ws(alpha1),
        many0_count(alt((alphanumeric1, tag("_")))),
    )))(input)
}

/// Parse function body. '(10, 2+3, sqrt(100))'
fn func_body(input: &str) -> IResult<&str, (Vec<Expression>, Option<Lambda>)> {
    delimited(
        tag("("),
        alt((
            pair(
                separated_list0(tag(","), add),
                preceded(opt(tag(",")), lambda),
            )
            .map(|(args, l)| (args, Some(l))),
            separated_list1(tag(","), add).map(|args| (args, None)),
        )),
        preceded_ws(tag(")")),
    )(input)
}

/// Parse function call. like 'sqrt(100)`.
/// handle nested case. 'sqrt(sqrt(16))'
fn func_call(input: &str) -> IResult<&str, Expression> {
    map(
        pair(func_ident, func_body),
        |(ident, (body, maybe_lambda))| {
            Expression::func_call(FuncCall::new(ident, body, maybe_lambda))
        },
    )(input)
}

/// Parse nested expression. like '( 10 * 20 )'
fn nest(input: &str) -> IResult<&str, Expression> {
    delimited(preceded_ws(char('(')), add, preceded_ws(char(')')))(input)
}

/// Parse non operator expression.
fn lit_or_nest(input: &str) -> IResult<&str, Expression> {
    // Order is matter.
    alt((literal_num, nest, func_call, constant, variable_exp))(input)
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
    use crate::expression::macros::{cst, cst_exp, fc_exp, lambda, lit, node, var};

    #[test]
    fn parse_constant() {
        assert_eq!(constant("pi"), Ok(("", cst_exp!("pi"))));
        assert_eq!(constant("  pi"), Ok(("", cst_exp!("pi"))));
        assert_eq!(constant("e"), Ok(("", cst_exp!("e"))));
        assert_eq!(constant("  e"), Ok(("", cst_exp!("e"))));
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

        assert_eq!(op_mul("^"), Ok(("", Operator::Pow)));
        assert_eq!(op_mul("  ^"), Ok(("", Operator::Pow)));
        assert_eq!(op_mul("  ^ "), Ok((" ", Operator::Pow)));
    }

    #[test]
    fn parse_lambda_args() {
        assert_eq!(lambda_arg("|x|"), Ok(("", Variable::new("x"))));
        assert_eq!(lambda_arg(" | x | "), Ok((" ", Variable::new("x"))));
        assert_eq!(lambda_arg(" |x |"), Ok(("", Variable::new("x"))));
        assert_eq!(lambda_arg(" | x|"), Ok(("", Variable::new("x"))));
    }

    #[test]
    fn parse_lambda_body() {
        assert_eq!(
            lambda_body("{ x ^ 2 }"),
            Ok(("", node!(Variable::new("x"), '^', lit!(2))))
        );
        assert_eq!(
            lambda_body("  { x ^ 2 } "),
            Ok((" ", node!(Variable::new("x"), '^', lit!(2))))
        );
        assert_eq!(
            lambda_body("  {x^2}"),
            Ok(("", node!(Variable::new("x"), '^', lit!(2))))
        );
        assert_eq!(
            lambda_body("x ^ 2 )"),
            Ok((" )", node!(Variable::new("x"), '^', lit!(2))))
        );
    }

    #[test]
    fn parse_lambda() {
        assert_eq!(
            lambda("|x|{x^2}"),
            Ok(("", lambda!([var!("x")], node!(var!("x"), '^', 2)))),
        );
    }

    #[test]
    fn parse_func_ident() {
        assert_eq!(func_ident("sqrt(100)"), Ok(("(100)", "sqrt")));
        assert_eq!(func_ident(" sqrt(100)"), Ok(("(100)", "sqrt")));
        assert_eq!(func_ident("f_xxx2(100)"), Ok(("(100)", "f_xxx2")));
    }

    #[test]
    fn parse_func_body() {
        assert_eq!(
            func_body("(1,2)"),
            Ok(("", (vec![lit!(1.), lit!(2.)], None)))
        );
        assert_eq!(
            func_body("(1,2 )"),
            Ok(("", (vec![lit!(1.), lit!(2.)], None)))
        );
        assert_eq!(
            func_body("(1, 2+3)"),
            Ok(("", (vec![lit!(1.), node!(2, '+', 3)], None)))
        );
        assert_eq!(
            func_body("(1,sqrt(100))"),
            Ok(("", (vec![lit!(1.), fc_exp!("sqrt", 100.)], None)))
        );
    }

    #[test]
    fn parse_func_call() {
        assert_eq!(func_call("sqrt(100)"), Ok(("", fc_exp!("sqrt", 100))));
        assert_eq!(
            func_call("f0(f1(f2(sqrt(100))))"),
            Ok((
                "",
                fc_exp!("f0", fc_exp!("f1", fc_exp!("f2", fc_exp!("sqrt", 100)))),
            ))
        );
        assert_eq!(func_call("abs(-1)"), Ok(("", fc_exp!("abs", -1.))));
        assert_eq!(func_call("abs(1)-"), Ok(("-", fc_exp!("abs", 1.))));

        assert_eq!(
            func_call("sig(1,2,|x| x^2)"),
            Ok((
                "",
                fc_exp!(
                    "sig",
                    [1, 2],
                    lambda!([var!("x")], node!(var!("x"), '^', 2))
                )
            ))
        );
        assert_eq!(
            func_call("sig(1,2|x| x^2)"),
            Ok((
                "",
                fc_exp!(
                    "sig",
                    [1, 2],
                    lambda!([var!("x")], node!(var!("x"), '^', 2))
                )
            ))
        );
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

        assert_eq!(mul("2 ^ 3"), Ok(("", node!(2, '^', 3))));
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
        assert_eq!(add("2 * pi"), Ok(("", node!(2, '*', cst!("pi")))));

        assert_eq!(
            add("sqrt(4) + abs(4)"),
            Ok(("", node!(fc_exp!("sqrt", 4), '+', fc_exp!("abs", 4))))
        );
    }
}
