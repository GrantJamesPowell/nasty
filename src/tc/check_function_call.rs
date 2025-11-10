use crate::{
    ast::expr::FunctionIdentifier,
    tc::{check_expr::ExprTyCheck, tc_error::TypeCheckError},
};

pub fn check_function_call<'a>(
    _func: FunctionIdentifier,
    _args: impl IntoIterator<Item = &'a ExprTyCheck>,
) -> Result<ExprTyCheck, TypeCheckError> {
    todo!()
}
