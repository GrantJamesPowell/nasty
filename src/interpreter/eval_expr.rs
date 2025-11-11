use crate::{
    ast::{
        expr::{Expr, ExprAst},
        scalar_value::ScalarValue,
    },
    interpreter::eval_ctx::{EvalCtx, EvalError},
};

pub fn eval_expr<Meta>(expr: &Expr<Meta>, _ctx: EvalCtx) -> Result<ScalarValue, EvalError> {
    let Expr { ast, .. } = expr;

    match ast.as_ref() {
        ExprAst::Scalar { .. } => todo!(),
        ExprAst::FunctionCall { .. } => todo!(),
    }
    // let check = &meta.0;

    // match  {

    //     _ => todo!(),
    // }
}
