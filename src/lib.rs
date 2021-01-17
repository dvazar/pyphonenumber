use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
pub fn is_valid(number: &str) -> PyResult<bool> {
    match phonenumber::parse(None, number) {
        Ok(number) => Ok(phonenumber::is_valid(&number)),
        Err(_) => Ok(false),
    }
}


#[pymodule]
fn phonenumber(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_valid, m)?)?;

    Ok(())
}
