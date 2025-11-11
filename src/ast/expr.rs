use super::scalar_value::ScalarValue;
use crate::ty::ETy;

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

#[derive(Debug, Clone)]
pub enum ExprAst {
    Scalar {
        val: ScalarValue,
        ty: ETy,
    },
    FunctionCall {
        func: FunctionIdentifier,
        args: Box<[usize]>,
    },
}
