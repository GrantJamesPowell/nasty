use crate::{ast::expr::FunctionIdentifier, ty::ETy};

pub fn estimate_output_type_of_function(func: FunctionIdentifier) -> Option<ETy> {
    use FunctionIdentifier::*;

    match func {
        Add => None,
        Sub => None,
        Mul => None,
        Div => None,
        Length => None,
        Upper => None,
        Lower => None,
        Coalesce => None,
    }
}
