use crate::{
    ast::expr::FunctionIdentifier,
    tc::{
        check_expr::{ExprTyCheck, ExprTypeCheckResult},
        tc_error::TypeCheckError,
    },
    ty::Ty,
};

pub fn check_function_call<'a>(
    func: FunctionIdentifier,
    args: impl IntoIterator<Item = &'a ExprTyCheck>,
) -> ExprTypeCheckResult {
    use FunctionIdentifier::*;
    match func {
        // Binary Math
        Add | Sub | Mul | Div => binary_math(args),

        // Text
        Length | Upper | Lower => text_modification(args),
    }
}

fn binary_math<'a>(_args: impl IntoIterator<Item = &'a ExprTyCheck>) -> ExprTypeCheckResult {
    todo!()
    // Ok(ExprTyCheck {
    //     ty: ETy {
    //         ty: numeric_promote(args.iter().map(|arg| &arg.ty.ty))?,
    //         nullable: args.iter().any(|arg| arg.ty.nullable)
    //     },
    //     windowed: args.iter().any(|arg| arg.windowed),
    //     aggregate: args.iter().any(|arg| arg.aggregate),
    //     attr_reqs: merge_attr_reqs(args.iter().map(|arg| &arg.attr_reqs))?,
    // })
}

fn text_modification<'a>(_args: impl IntoIterator<Item = &'a ExprTyCheck>) -> ExprTypeCheckResult {
    todo!();
}

fn numeric_promote<'a>(tys: impl IntoIterator<Item = &'a Ty>) -> Result<Ty, TypeCheckError> {
    tys.into_iter().map(|ty| {
        ty.numeric_rank()
            .ok_or_else(|| -> TypeCheckError { todo!() })
    });
    todo!();
}
