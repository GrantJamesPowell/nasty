use crate::{ast::expr::ExprAst, ast::expr::FunctionIdentifier, tc::check_expr::CheckedExpr};

#[derive(Debug, Clone)]
pub struct Expression<Meta = ()> {
    ast: CheckedExpr<Meta>,
}

impl Expression {
    fn add(&self, rhs: impl Into<Expression>) -> Expression {
        let ast = ExprAst::FunctionCall {
            func: FunctionIdentifier::Add,
            args: (|| todo!())(),
        };

        todo!()
    }

    fn tap(&self, func: impl FnOnce(&Expression)) -> Expression {
        func(self);
        self.clone()
    }
}

impl From<i32> for Expression {
    fn from(_value: i32) -> Self {
        todo!();
    }
}
