use crate::{
    ast::expr::FunctionIdentifier,
    tc::{
        check_expr::{ExprCheckRes, ExprTyCheck},
        merge_attr_reqs::merge_attr_reqs,
        tc_error::TypeCheckError,
    },
    ty::{ETy, Ty},
};

pub fn check_function_call(
    func: FunctionIdentifier,
    args: &[ExprTyCheck],
) -> Result<ExprTyCheck, TypeCheckError> {
    match func {
        // Binary Math
        FunctionIdentifier::Add
        | FunctionIdentifier::Sub
        | FunctionIdentifier::Mul
        | FunctionIdentifier::Div => with_arity(2, args, binary_math),

        // Text
        FunctionIdentifier::Length | FunctionIdentifier::Upper | FunctionIdentifier::Lower => {
            with_arity(1, args, text_modification)
        }
    }
}

type FuncTyCheck = fn(&[ExprTyCheck]) -> ExprTyCheck;

fn binary_math(args: &[ExprTyCheck]) -> ExprCheckRes {
    Ok(ExprTyCheck {
        ty: ETy {
            ty: numeric_promote(args.iter().map(|arg| &arg.ty.ty))?,
            nullable: args.iter().any(|arg| arg.ty.nullable)
        },
        windowed: args.iter().any(|arg| arg.windowed),
        aggregate: args.iter().any(|arg| arg.aggregate),
        attr_reqs: merge_attr_reqs(args.iter().map(|arg| &arg.attr_reqs))?,
    })
}

fn text_modification(_args: &[ExprTyCheck]) -> ExprCheckRes {
    todo!();
}

fn numeric_promote<'a>(tys: impl IntoIterator<Item = &'a Ty>) -> Result<Ty, TypeCheckError> {
    tys.into_iter().map(|ty| ty.numeric_rank().ok_or_else(|| -> TypeCheckError { todo!() }));
    todo!();
}

fn with_arity(
    n: usize,
    args: &[ExprTyCheck],
    func: impl Fn(&[ExprTyCheck]) -> ExprCheckRes,
) -> ExprCheckRes {
    if args.len() == n {
        func(args)
    } else {
        todo!("Invalid arity")
    }
}
