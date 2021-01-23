mod phone_number;
mod utils;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::utils::{
    __pyo3_get_function_parse,
    __pyo3_get_function_is_valid_number,
    __pyo3_get_function_format_number,
};


/// Library for parsing, formatting and validating international phone numbers.
///
/// This library is a bindings to the phonenumber library written in Rust
/// https://crates.io/crates/phonenumber
#[pymodule]
fn phonenumber(py: Python, m: &PyModule) -> PyResult<()> {

    m.add("NumberParseException", py.get_type::<utils::NumberParseException>())?;

    m.add_class::<phone_number::PyPhoneNumberFormat>()?;
    m.add_class::<phone_number::PyPhoneNumber>()?;

    m.add_function(wrap_pyfunction!(is_valid_number, m)?)?;
    m.add_function(wrap_pyfunction!(format_number, m)?)?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;

    Ok(())
}
