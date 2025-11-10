use super::scalar_value::ScalarValue;
use crate::ty::{ETy, Ty};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionIdentifier {
    Add,
    Sub,
    Mul,
    Div,
    Length,
    Upper,
    Lower,
    Coalesce,
}

#[derive(Debug, Clone)]
pub struct Expr<Meta> {
    pub ast: Arc<ExprAst<Meta>>,
    pub meta: Arc<Meta>,
}

#[derive(Debug, Clone)]
pub enum ExprAst<Meta> {
    Scalar {
        val: ScalarValue,
        ty: ETy,
    },
    FunctionCall {
        func: FunctionIdentifier,
        args: Box<[Arc<Expr<Meta>>]>,
    },
    Cast {
        source: Arc<Expr<Meta>>,
        target: Ty,
    },
}
