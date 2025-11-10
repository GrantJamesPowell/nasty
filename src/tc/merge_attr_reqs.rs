use crate::tc::tc_error::TypeCheckError;

use super::check_expr::AttrReqs;

pub fn merge_attr_reqs<'a>(
    _reqs: impl Iterator<Item = &'a AttrReqs>,
) -> Result<AttrReqs, TypeCheckError> {
    todo!()
}
