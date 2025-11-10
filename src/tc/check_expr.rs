use smartstring::alias::String;
use std::collections::HashMap;

use crate::{
    ast::expr::Expr,
    tc::{
        check_cast::check_cast, check_function_call::check_function_call,
        check_scalar::check_scalar, tc_error::TypeCheckError,
    },
    ty::ETy,
};

#[derive(Debug, Clone)]
pub struct ExprTyCheck {
    ty: ETy,
    windowed: bool,
    aggregate: bool,
    attr_reqs: HashMap<String, ETy>,
}

pub fn check_expr(node: &Expr) -> Result<ExprTyCheck, TypeCheckError> {
    match node {
        Expr::Scalar { val, ty } => {
            check_scalar(val, ty)?;

            Ok(ExprTyCheck {
                ty: ty.clone(),
                windowed: false,
                aggregate: false,
                attr_reqs: Default::default(),
            })
        }
        Expr::Cast { source, target } => {
            let child = check_expr(source)?;
            check_cast(&child.ty.ty, target)?;

            let new_ty = ETy {
                ty: target.clone(),
                nullable: child.ty.nullable,
            };

            Ok(ExprTyCheck {
                ty: new_ty,
                ..child
            })
        }
        Expr::Attribute { name, ty } => Ok(ExprTyCheck {
            ty: ty.clone(),
            windowed: false,
            aggregate: false,
            attr_reqs: HashMap::from([(name.clone(), ty.clone())]),
        }),
        Expr::FunctionCall { func, args } => {
            let arg_tys = args
                .iter()
                .map(|arg| check_expr(arg))
                .collect::<Result<Vec<ExprTyCheck>, TypeCheckError>>()?;

            check_function_call(*func, arg_tys.iter())
        }
    }
}
