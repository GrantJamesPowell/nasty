use crate::ty::Ty;

use super::tc_error::TypeCheckError;

pub fn check_cast(_source_ty: &Ty, _to: &Ty) -> Result<(), TypeCheckError> {
    todo!()
}
