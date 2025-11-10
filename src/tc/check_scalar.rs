use crate::{
    ast::scalar_value::ScalarValue,
    tc::tc_error::TypeCheckError,
    ty::{ETy, Ty},
};

pub fn check_scalar(val: &ScalarValue, ety: &ETy) -> Result<(), TypeCheckError> {
    match val {
        ScalarValue::Null => {
            if ety.nullable {
                Ok(())
            } else {
                todo!("Invalid nullable")
            }
        }
        ScalarValue::Bool(_) => {
            if matches!(ety.ty, Ty::Boolean) {
                Ok(())
            } else {
                todo!()
            }
        }
        ScalarValue::Int32(_) => todo!(),
        ScalarValue::Int64(_) => todo!(),
        ScalarValue::Float32(_) => todo!(),
        ScalarValue::Float64(_) => todo!(),
        ScalarValue::Text(_) => todo!(),
        ScalarValue::Bytea(items) => todo!(),
        ScalarValue::Array(scalar_values) => todo!(),
        ScalarValue::Map(hash_map) => todo!(),
    }
}
