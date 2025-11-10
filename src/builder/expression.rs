use crate::{tc::check_expr::CheckedExpr, ty::ETy};

pub struct Expression<Meta = ()> {
    ast: CheckedExpr<Meta>,
}

impl Expression {
    fn add(&self, rhs: impl Into<Expression>) -> Expression {
        todo!()
    }

    fn sub(&self, lhs: impl Into<Expression>) -> Expression {
        todo!()
    }
}
