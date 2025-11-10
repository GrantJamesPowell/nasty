use smartstring::alias::String;
use std::{clone, collections::HashMap, sync::Arc};

use crate::{
    ast::expr::{Expr, ExprAst},
    tc::{check_function_call::check_function_call, tc_error::TypeCheckError},
    ty::ETy,
};

pub type AttrReqs = HashMap<String, ETy>;

#[derive(Debug, Clone)]
pub struct ExprTyCheck {
    pub ty: ETy,
    pub windowed: bool,
    pub aggregate: bool,
    pub attr_reqs: AttrReqs,
}

pub type ExprTypeCheckResult = Result<ExprTyCheck, TypeCheckError>;

type CheckedExpr<Meta> = Expr<(ExprTypeCheckResult, Arc<Meta>)>;

pub struct TypeCheckedExpr<Meta>(CheckedExpr<Meta>);

pub fn check_expr<Meta>(input: &Expr<Meta>) -> TypeCheckedExpr<Meta> {
    use ExprAst::*;
    let Expr { ast, meta } = input;

    match ast.as_ref() {
        // Attribute { name, ty } => todo!(),
        // Scalar { val, ty } => todo!(),
        // Cast { source, target } => todo!(),
        FunctionCall { func, args } => {
            let arg_results: Vec<TypeCheckedExpr<Meta>> =
                args.iter().map(|arg| check_expr(arg)).collect();

            let res: ExprTypeCheckResult = {
                let mut arg_tys: Vec<&ExprTyCheck> = Vec::with_capacity(args.len());
                let mut arg_errs: Vec<usize> = Vec::new();

                for (i, res) in arg_results.iter().enumerate() {
                    match res.0.meta.0.as_ref() {
                        Ok(ty) => arg_tys.push(ty),
                        Err(_) => arg_errs.push(i),
                    }
                }

                if arg_errs.is_empty() {
                    check_function_call(*func, arg_tys.iter().map(|&ty| ty))
                } else {
                    todo!("Propogate errors from children")
                }
            };

            let output: CheckedExpr<Meta> = Expr {
                ast: Arc::from(FunctionCall {
                    func: *func,
                    args: arg_results
                        .into_iter()
                        .map(|arg| Arc::from(arg.0))
                        .collect(),
                }),
                meta: Arc::from((res, meta.clone())),
            };

            TypeCheckedExpr(output)
        }
        _ => todo!(),
    }

    // match ast {
    //     Expr::Scalar { val, ty } => {
    //         todo!()
    //         // check_scalar(val, ty)?;

    //         // Ok(ExprTyCheck {
    //         //     ty: ty.clone(),
    //         //     windowed: false,
    //         //     aggregate: false,
    //         //     attr_reqs: Default::default(),
    //         // })
    //     }
    //     Expr::Cast { source, target } => {
    //         todo!()
    //         //let child = check_expr(source)?;
    //         //check_cast(&child.ty.ty, target)?;

    //         //let new_ty = ETy {
    //         //    ty: target.clone(),
    //         //    nullable: child.ty.nullable,
    //         //};

    //         //Ok(ExprTyCheck {
    //         //    ty: new_ty,
    //         //    ..child
    //         //})
    //     }
    //     Expr::Attribute { name, ty } => {
    //         to
    //     //     Ok(ExprTyCheck {
    //     //     ty: ty.clone(),
    //     //     windowed: false,
    //     //     aggregate: false,
    //     //     attr_reqs: HashMap::from([(name.clone(), ty.clone())]),
    //     // })
    //     },
    //     Expr::FunctionCall { func, args } => {
    //         let arg_tys = args
    //             .iter()
    //             .map(|arg| check_expr(arg))
    //             .collect::<Result<Vec<ExprTyCheck>, TypeCheckError>>()?;

    //         check_function_call(*func, arg_tys.iter().as_slice())
    //     }
    // }
}
