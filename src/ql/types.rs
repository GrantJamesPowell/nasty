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
