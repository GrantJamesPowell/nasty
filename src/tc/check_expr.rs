use std::sync::Arc;

use crate::{
    ast::expr::{Expr, ExprAst},
    tc::{attr_reqs::AttrReqs, tc_error::TypeCheckError},
    ty::ETy,
};

#[derive(Debug, Clone)]
pub struct ExprTyCheck {
    pub ty: ETy,
    pub attr_reqs: AttrReqs,
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

pub type CheckedExpr<Meta> = Expr<(ExprTypeCheckResult, Arc<Meta>)>;

pub struct TypeCheckedExpr<Meta>(CheckedExpr<Meta>);

pub fn check_expr<Meta>(input: &Expr<Meta>) -> TypeCheckedExpr<Meta> {
    use ExprAst::*;
    let Expr { ast, meta } = input;

    let (new_ast, check): (
        ExprAst<(ExprTypeCheckResult, Arc<Meta>)>,
        ExprTypeCheckResult,
    ) = match ast.as_ref() {
        Attribute { name, ty } => {
            let res = ExprTypeCheckResult::Success(ExprTyCheck {
                ty: ty.clone(),
                attr_reqs: AttrReqs::new(),
            });

            let new_ast = Attribute {
                name: name.clone(),
                ty: ty.clone(),
            };

            (new_ast, res)
        }
        Cast { source, target } => {
            let source_res = check_expr(source).0;

            // let res: ExprTypeCheckResult = {
            //     match source_res.meta.0.as_ref() {
            //         Ok(source_ty) => check_cast(&source_ty.ty.ty, target)
            //             .map(|_| ExprTyCheck {
            //                 ty: ETy {
            //                     ty: target.clone(),
            //                     nullable: source_ty.ty.nullable,
            //                 },
            //                 windowed: false,
            //                 aggregate: false,
            //                 attr_reqs: AttrReqs::new(),
            //             })
            //             .map_err(|err| CheckFailure { err, partial: () }),
            //         Err(_) => todo!("Invalid cast?"),
            //     }
            // };

            todo!();

            // let output: CheckedExpr<Meta> = Expr {
            //     ast: Arc::from(Cast {
            //         source: Arc::from(source_res),
            //         target: target.clone(),
            //     }),
            //     meta: Arc::from((res, Arc::clone(meta))),
            // };

            // output
        }
        Scalar { val, ty } => {
            todo!();
            // let res = check_scalar(val, ty).map(|_| ExprTyCheck {
            //     ty: ty.clone(),
            //     windowed: false,
            //     aggregate: false,
            //     attr_reqs: AttrReqs::new(),
            // });

            // let output: CheckedExpr<Meta> = Expr {
            //     ast: Arc::from(Scalar {
            //         val: val.clone(),
            //         ty: ty.clone(),
            //     }),
            //     meta: Arc::from((res, Arc::clone(meta))),
            // };

            // output
        }
        FunctionCall { func, args } => {
            todo!();
            // let arg_results: Vec<TypeCheckedExpr<Meta>> =
            //     args.iter().map(|arg| check_expr(arg)).collect();

            // let res: ExprTypeCheckResult = {
            //     let mut arg_tys: Vec<&ExprTyCheck> = Vec::with_capacity(args.len());
            //     let mut arg_errs: Vec<usize> = Vec::new();

            //     for (i, res) in arg_results.iter().enumerate() {
            //         match res.0.meta.0.as_ref() {
            //             Ok(ty) => arg_tys.push(ty),
            //             Err(_) => arg_errs.push(i),
            //         }
            //     }

            //     if arg_errs.is_empty() {
            //         check_function_call(*func, arg_tys.iter().map(|&ty| ty))
            //     } else {
            //         todo!("Propogate errors from children")
            //     }
            // };

            // let output: CheckedExpr<Meta> = Expr {
            //     ast: Arc::from(FunctionCall {
            //         func: *func,
            //         args: arg_results
            //             .into_iter()
            //             .map(|arg| Arc::from(arg.0))
            //             .collect(),
            //     }),
            //     meta: Arc::from((res, Arc::clone(meta))),
            // };

            // output
        }
    };
    let output: CheckedExpr<Meta> = Expr {
        ast: Arc::from(new_ast),
        meta: Arc::from((check, Arc::clone(meta))),
    };

    TypeCheckedExpr(output)
}
