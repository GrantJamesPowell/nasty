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
        match self {
            ExprTypeCheckResult::Success(ty) => Some(ty),
            ExprTypeCheckResult::SourceError { output_ty, .. } => output_ty.as_ref(),
            ExprTypeCheckResult::PropogatedError { output_ty, .. } => output_ty.as_ref(),
        }
    }
}

pub fn check_expr<Meta>(expr: &[ExprAst]) -> Vec<ExprTypeCheckResult> {
    let mut tc: Vec<ExprTypeCheckResult> = Vec::with_capacity(expr.len());

    for (i, node) in expr.iter().enumerate() {
        let res: ExprTypeCheckResult = match node {
            ExprAst::Scalar { val, ty } => check_scalar(val, ty),
            ExprAst::FunctionCall { func, args } => {
                let arg_tys: Result<Vec<&ExprTypeCheckResult>, usize> = args
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(arg_num, pos)| tc.get(i - pos).ok_or(arg_num))
                    .collect();

                match arg_tys {
                    Ok(tys) => tys
                        .into_iter()
                        .map(|x| x.ty_ref())
                        .map(|x| x.ok_or(ExprTypeCheckResult::PropogatedError { output_ty: None }))
                        .collect::<Result<Vec<&ExprTyCheck>, ExprTypeCheckResult>>()
                        .map_or_else(|x| x, |tys| check_function_call(*func, tys)),
                    Err(_) => todo!(),
                }
            }
        };

        tc[i] = res;
    }

    tc
}
