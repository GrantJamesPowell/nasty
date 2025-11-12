use crate::{
    ast::expr::ExprAst,
    tc::{
        check_function_call::check_function_call, check_scalar::check_scalar,
        tc_error::TypeCheckError,
    },
    ty::ETy,
};

#[derive(Debug, Clone)]
pub struct ExprTyCheck {
    pub ty: ETy,
}

#[derive(Debug, Clone)]
pub enum ExprTypeCheckResult {
    Success(ExprTyCheck),
    SourceError {
        err: TypeCheckError,
        output_ty: Option<ExprTyCheck>,
    },
    PropogatedError {
        output_ty: Option<ExprTyCheck>,
    },
}

impl ExprTypeCheckResult {
    fn err_ref(&self) -> Option<&TypeCheckError> {
        match self {
            ExprTypeCheckResult::SourceError { err, .. } => Some(err),
            _ => None,
        }
    }

    fn ty_ref(&self) -> Option<&ExprTyCheck> {
        use ExprTypeCheckResult::*;

        match self {
            Success(ty) => Some(ty),
            SourceError { output_ty, .. } => output_ty.as_ref(),
            PropogatedError { output_ty, .. } => output_ty.as_ref(),
        }
    }
}

pub fn check_expr<Meta>(expr: &[ExprAst]) -> Vec<ExprTypeCheckResult> {
    let mut tcs: Vec<ExprTypeCheckResult> = Vec::with_capacity(expr.len());
    check_expr_from_existing_checks(expr, &mut tcs);
    tcs
}

pub fn check_expr_from_existing_checks(expr: &[ExprAst], tcs: &mut [ExprTypeCheckResult]) {
    use ExprAst::*;

    for (i, node) in expr.iter().enumerate().skip(expr.len()) {
        let res: ExprTypeCheckResult = match node {
            Scalar { val, ty } => check_scalar(val, ty),
            FunctionCall { func, args } => {
                let arg_tys = lookup_expr_type_checks(&tcs, args.iter().map(|&pos| i - pos));

                match arg_tys {
                    Ok(tys) => check_function_call(*func, tys),
                    Err(err) => err,
                }
            }
        };

        tcs[i] = res;
    }
}

fn lookup_expr_type_checks<'a>(
    tcs: &[ExprTypeCheckResult],
    idxs: impl IntoIterator<Item = usize>,
) -> Result<Vec<&ExprTyCheck>, ExprTypeCheckResult> {
    idxs.into_iter()
        .map(|idx| tcs.get(idx).and_then(|check| check.ty_ref()))
        .map(|x| x.ok_or_else(|| -> ExprTypeCheckResult { todo!("handle invalid indexes") }))
        .collect()
}
