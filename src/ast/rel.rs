use std::collections::HashMap;

use smartstring::alias::String;

use crate::ty::ExtendedTy;

use super::expr::Expr;

pub enum Rel {
    GenerateSeries {
        start: Expr,
        stop: Expr,
    },
    Where {
        condition: Expr,
    },
    Table {
        name: String,
        schema: Option<String>,
        attributes: HashMap<String, ExtendedTy>,
    },
}
