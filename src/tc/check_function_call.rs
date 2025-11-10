use crate::{
    ast::expr::FunctionIdentifier,
    tc::{
        check_expr::{ExprTyCheck, ExprTypeCheckResult},
        tc_error::TypeCheckError,
    },
    ty::Ty,
};

pub fn check_function_call<'a>(
    _func: FunctionIdentifier,
    _args: impl IntoIterator<Item = &'a ExprTyCheck>,
) -> Result<ExprTyCheck, TypeCheckError> {
    todo!()
    // match func {
    //     // Binary Math
    //     FunctionIdentifier::Add
    //     | FunctionIdentifier::Sub
    //     | FunctionIdentifier::Mul
    //     | FunctionIdentifier::Div => with_arity(2, args, binary_math),

    //     // Text
    //     FunctionIdentifier::Length | FunctionIdentifier::Upper | FunctionIdentifier::Lower => {
    //         with_arity(1, args, text_modification)
    //     }
    // }
}

fn binary_math(args: &[ExprTyCheck]) -> ExprTypeCheckResult {
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

fn text_modification(_args: &[ExprTyCheck]) -> ExprTypeCheckResult {
    todo!();
}

fn numeric_promote<'a>(tys: impl IntoIterator<Item = &'a Ty>) -> Result<Ty, TypeCheckError> {
    tys.into_iter().map(|ty| {
        ty.numeric_rank()
            .ok_or_else(|| -> TypeCheckError { todo!() })
    });
    todo!();
}

// fn with_arity(
//     n: usize,
//     args: &[ExprTyCheck],
//     func: impl Fn(&[ExprTyCheck]) -> ExprCheckRes,
// ) -> ExprCheckRes {
//     if args.len() == n {
//         func(args)
//     } else {
//         todo!("Invalid arity")
//     }
// }
