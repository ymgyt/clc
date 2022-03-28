/// Return constant.
/// cst!("e")
macro_rules! cst {
    ($c:expr) => {
        crate::expression::Constant::try_from($c).unwrap()
    };
}

/// Return constant expression.
/// cst_exp!("e")
macro_rules! cst_exp {
    ($c:expr) => {
        crate::expression::Expression::constant(crate::macros::cst!($c))
    };
}

/// Return Literal expression.
/// lit!(10).
macro_rules! lit {
    ($lit:expr) => {
        crate::expression::Expression::from($lit)
    };
}

/// Return function call expression.
/// fc_exp!("sqrt", 100)
macro_rules! fc_exp {
        ($ident:expr, $( $expr:expr ),+) => {{
            let fc = crate::macros::fc!($ident, $( $expr ),+ );
            crate::expression::Expression::func_call(fc)
        }}
}

/// Return function call.
/// fc!("sqrt", 100)
macro_rules! fc {
        ($ident:expr, $( $expr:expr ),+) => {{
           crate::expression::FuncCall::new($ident, vec![ $( $expr ),+ ])
        }}
}

/// Return node expression.
/// node!(10, '*', 20)
macro_rules! node {
    ($left:expr, $op:expr, $right:expr) => {
        crate::expression::Expression::ast(crate::expression::Node::new(
            crate::expression::Expression::from($left),
            crate::expression::Operator::try_from($op).unwrap(),
            crate::expression::Expression::from($right),
        ))
    };
}

pub(crate) use cst;
pub(crate) use cst_exp;
pub(crate) use fc;
pub(crate) use fc_exp;
pub(crate) use lit;
pub(crate) use node;
