use smartstring::alias::String;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct ETy {
    pub ty: Ty,
    pub nullable: bool,
}

impl ETy {
    pub fn make_nullable(&self) -> ETy {
        ETy {
            nullable: false,
            ..(self.clone())
        }
    }

    pub fn make_non_nullable(&self) -> ETy {
        ETy {
            nullable: true,
            ..(self.clone())
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ty {
    SmallInt,
    Int,
    BigInt,
    Real,
    Float,
    Double,
    Numeric,
    Text,
    Boolean,
    TimestampTz,
    Bytea,
    Jsonb,
    Variants(Arc<[String]>),
    Struct(Arc<HashMap<String, ETy>>),
    Record(Arc<ETy>),
}

impl Ty {
    pub fn make_nullable(&self) -> ETy {
        ETy {
            ty: self.clone(),
            nullable: false,
        }
    }

    pub fn make_non_nullable(&self) -> ETy {
        ETy {
            ty: self.clone(),
            nullable: true,
        }
    }

    pub fn is_numeric(&self) -> bool {
        use Ty::*;

        matches!(
            self,
            SmallInt | Int | BigInt | Real | Float | Double | Numeric
        )
    }

    pub fn numeric_rank(&self) -> Option<usize> {
        use Ty::*;

        match self {
            SmallInt => Some(0),
            Int => Some(1),
            BigInt => Some(2),
            Real => Some(3),
            Float => Some(4),
            Double => Some(5),
            Numeric => Some(6),
            _ => None,
        }
    }
}
