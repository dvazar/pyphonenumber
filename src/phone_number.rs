use pyo3::prelude::*;

use phonenumber::{Mode, PhoneNumber, format, is_valid};


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
    phone_number: PhoneNumber,
}

impl PyPhoneNumber {

    pub fn new(phone_number: PhoneNumber) -> Self {
        PyPhoneNumber { phone_number }
    }

    pub fn is_valid(&self) -> bool {
        is_valid(&self.phone_number)
    }

    pub fn format(&self, num_format: &str) -> String {

        match PyPhoneNumberFormat::get_mode(num_format) {
            None => format!("{}", &self.phone_number),
            Some(mode) => {
                format(&self.phone_number).mode(mode).to_string()
            }
        }

    }

}
