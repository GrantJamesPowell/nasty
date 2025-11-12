use crate::ast::scalar_value::ScalarValue;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct TokenMeta {
    pub start: usize,
    pub length: usize,
}

pub type WithMeta<T> = (T, TokenMeta);

#[derive(Debug, Clone, Copy)]
pub enum FunctionCallDisplay {
    Call,
    Prefix,
    Infix,
}

#[derive(Debug, Clone)]
pub enum QlAst {
    Symbol(Arc<str>),
    Literal(ScalarValue),
    Call {
        op: WithMeta<Arc<QlAst>>,
        display: FunctionCallDisplay,
        args: Box<[Arc<WithMeta<QlAst>>]>,
    },
}

impl Into<QlAst> for ScalarValue {
    fn into(self) -> QlAst {
        QlAst::Literal(self)
    }
}

impl Into<QlAst> for bool {
    fn into(self) -> QlAst {
        QlAst::Literal(ScalarValue::Bool(self))
    }
}

impl Into<QlAst> for i32 {
    fn into(self) -> QlAst {
        QlAst::Literal(ScalarValue::Int32(self))
    }
}

impl Into<QlAst> for i64 {
    fn into(self) -> QlAst {
        QlAst::Literal(ScalarValue::Int64(self))
    }
}

impl Into<QlAst> for f32 {
    fn into(self) -> QlAst {
        QlAst::Literal(ScalarValue::Float32(self))
    }
}

impl Into<QlAst> for f64 {
    fn into(self) -> QlAst {
        QlAst::Literal(ScalarValue::Float64(self))
    }
}
