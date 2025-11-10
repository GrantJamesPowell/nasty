use crate::{ast::expr::ExprAst, ast::expr::FunctionIdentifier, tc::check_expr::CheckedExpr};

pub struct Expression<Meta = ()> {
    ast: CheckedExpr<Meta>,
}

impl Expression {
    fn add(&self, rhs: impl Into<Expression>) -> Expression {
        let ast = ExprAst::FunctionCall {
            func: FunctionIdentifier::Add,
            args: Box::from([]),
        };

        todo!()
    }
}
