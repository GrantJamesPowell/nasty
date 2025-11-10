use smartstring::alias::String;

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
}

pub enum Expr {
    Attribute {
        name: String,
        ty: ETy,
    },
    Scalar {
        val: ScalarValue,
        ty: ETy,
    },
    FunctionCall {
        args: Arc<[Expr]>,
        func: FunctionIdentifier,
    },
    Cast {
        source: Arc<Expr>,
        target: Ty,
    },
}
