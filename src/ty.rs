use smartstring::alias::String;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct ETy {
    pub ty: Ty,
    pub nullable: bool,
}

impl ETy {}

#[derive(Debug, Clone)]
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
    Variants(Arc<[String]>),
    Struct(HashMap<String, ETy>),
    Record(Arc<ETy>),
}
