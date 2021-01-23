// Copyright 2021 Dmitrii Azarenko
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use phonenumber::{Mode, PhoneNumber, format, is_valid};
use pyo3::class::basic::CompareOp;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;


const E164: &str = "E164";
const INTERNATIONAL: &str = "INTERNATIONAL";
const NATIONAL: &str = "NATIONAL";
const RFC3966: &str = "RFC3966";


/// Phone number format.
///
/// INTERNATIONAL and NATIONAL formats are consistent with the definition in
/// ITU-T Recommendation E123. However we follow local conventions such as using
/// '-' instead of whitespace as separators. For example, the number of the
/// Google Switzerland office will be written as "+41 44 668 1800" in
/// INTERNATIONAL format, and as "044 668 1800" in NATIONAL format. E164 format
/// is as per INTERNATIONAL format but with no formatting applied,
/// e.g. "+41446681800". RFC3966 is as per INTERNATIONAL format, but with all
/// spaces and other separating symbols replaced with a hyphen, and with any
/// phone number extension appended with ";ext=". It also will have a prefix of
/// "tel:" added, e.g. "tel:+41-44-668-1800".
///
/// Note: If you are considering storing the number in a neutral format, you
/// are highly advised to use the PhoneNumber class.
#[pyclass(name="PhoneNumberFormat", module="phonenumber")]
pub struct PyPhoneNumberFormat {}

impl PyPhoneNumberFormat {

    pub fn get_mode(code: &str) -> Option<Mode> {
        match code {
            E164 => Some(Mode::E164),
            INTERNATIONAL => Some(Mode::International),
            NATIONAL => Some(Mode::National),
            RFC3966 => Some(Mode::Rfc3966),
            _ => None,
        }
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl PyPhoneNumberFormat {

    #[classattr]
    pub fn E164() -> &'static str { E164 }

    #[classattr]
    pub fn INTERNATIONAL() -> &'static str { INTERNATIONAL }

    #[classattr]
    pub fn NATIONAL() -> &'static str { NATIONAL }

    #[classattr]
    pub fn RFC3966() -> &'static str { RFC3966 }

}


/// Class representing international telephone numbers.
#[pyclass(name="PhoneNumber", module="phonenumber")]
pub struct PyPhoneNumber {
    wrap: PhoneNumber,
}

#[pyproto]
impl PyObjectProtocol for PyPhoneNumber {

    fn __str__(&self) -> PyResult<String> {
        let mut doc = format!(
            "Country Code: {} National Number: {}",
            String::from(self.wrap.code().value().to_string()),
            self.wrap.national().value().to_string(),
        );
        if let Some(ext) = self.wrap.extension() {
            doc = format!("{} Extension: {}", doc, ext);
        }
        if let Some(carr) = self.wrap.carrier() {
            doc = format!("{} Carrier: {}", doc, carr);
        }
        Ok(doc)
    }

    fn __repr__(&self) -> PyResult<String> {
        let mut doc = format!(
            "PhoneNumber(country_code={}, national_number={}",
            String::from(self.wrap.code().value().to_string()),
            self.wrap.national().value().to_string(),
        );
        if let Some(ext) = self.wrap.extension() {
            doc = format!("{}, extension='{}'", doc, ext);
        }
        if let Some(carr) = self.wrap.carrier() {
            doc = format!("{}, carrier='{}'", doc, carr);
        }
        doc = format!("{})", doc);
        Ok(doc)
    }

    fn __richcmp__(&self, other: Py<PyPhoneNumber>, op: CompareOp) -> PyResult<bool> {

        let pyerr = |op| {
            PyTypeError::new_err(format!(
                "'{}' not supported between instances of '{type}' and '{type}'",
                op, type="PhoneNumber",
            ))
        };

        Python::with_gil(|py| match op {
            CompareOp::Eq => Ok(self.wrap == other.borrow(py).wrap),
            CompareOp::Ne => Ok(self.wrap != other.borrow(py).wrap),
            CompareOp::Lt => Err(pyerr("<")),
            CompareOp::Gt => Err(pyerr(">")),
            CompareOp::Le => Err(pyerr("<=")),
            CompareOp::Ge => Err(pyerr(">=")),
        })
    }

}

impl PyPhoneNumber {

    pub fn new(phone_number: PhoneNumber) -> Self {
        PyPhoneNumber { wrap: phone_number }
    }

    pub fn is_valid(&self) -> bool {
        is_valid(&self.wrap)
    }

    pub fn format(&self, num_format: &str) -> String {

        match PyPhoneNumberFormat::get_mode(num_format) {
            None => format!("{}", &self.wrap),
            Some(mode) => {
                format(&self.wrap).mode(mode).to_string()
            }
        }

    }

}
