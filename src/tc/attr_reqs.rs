use smartstring::alias::String;
use std::{collections::HashMap, sync::Arc};

use crate::{tc::tc_error::TypeCheckError, ty::ETy};

#[derive(Debug, Clone)]
pub struct AttrReqs(Option<Arc<HashMap<String, ETy>>>);

impl AttrReqs {
    pub fn merge<'a>(
        _reqs: impl IntoIterator<Item = &'a AttrReqs>,
    ) -> Result<AttrReqs, TypeCheckError> {
        todo!();
    }

    pub fn new() -> Self {
        AttrReqs(Default::default())
    }

    pub fn from_attr(name: impl Into<String>, ty: ETy) -> Self {
        let inner: HashMap<String, ETy> = HashMap::from([(name.into(), ty)]);
        AttrReqs(Some(Arc::from(inner)))
    }
}
