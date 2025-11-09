use smartstring::alias::String;
use std::{collections::HashMap, sync::Arc};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ExtendedTy {
    pub ty: Ty,
    pub nullabe: bool
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Ty {
    Int,
    Real,
    Double,
    Text,
    Boolean,
    Float,
    TimestampTz,
    Bytea,
    Jsonb,
    Struct(HashMap<String, ExtendedTy>),
    Record(Arc<ExtendedTy>)
}
