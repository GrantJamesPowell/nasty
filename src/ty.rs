use std::collections::HashMap;

pub struct ExtendedTy {
    pub ty: Ty,
    pub nullabe: bool
}

pub enum Ty {
    Struct { fields: HashMap<string, string> }
}
