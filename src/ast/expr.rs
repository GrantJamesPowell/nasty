use super::scalar_value::ScalarValue;
use crate::ty::{ExtendedTy, Ty};
use std::sync::Arc;

pub enum FunctionIdentifier {
    Add,
    Sub,
    Mul,
    Div,
    Length,
    Upper,
    Lower,
}

pub enum Rel {
    GenerateSeries { start: Expr, stop: Expr },
    Where { condition: Expr },
}

pub enum Expr {
    Attribute {},
    Scalar {
        val: ScalarValue,
        ty: ExtendedTy,
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
