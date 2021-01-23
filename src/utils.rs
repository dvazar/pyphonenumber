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

use phonenumber::country;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use crate::phone_number::PyPhoneNumber;


create_exception!(phonenumber, NumberParseException, PyException);


fn parse_region(region: &str) -> PyResult<country::Id> {
    match region.to_uppercase().parse::<country::Id>() {
        Ok(reg) => Ok(reg),
        Err(_) => {
            Err(PyErr::new::<NumberParseException, _>("invalid region"))
        }
    }
}


/// Parse a string and return a corresponding PhoneNumber object.
///
/// Arguments:
/// number -- The number that we are attempting to parse. This can
///           contain formatting such as +, ( and -, as well as a phone
///           number extension. It can also be provided in RFC3966 format.
/// region -- The region that we are expecting the number to be from. This
///           is only used if the number being parsed is not written in
///           international format. The country_code for the number in
///           this case would be stored as that of the default region
///           supplied. If the number is guaranteed to start with a '+'
///           followed by the country calling code, then None or
///           UNKNOWN_REGION can be supplied.
///
/// Raises:
/// NumberParseException if the string is not considered to be a viable
/// phone number (e.g.  too few or too many digits) or if no default
/// region was supplied and the number is not in international format
/// (does not start with +).
#[pyfunction(region="None")]
#[text_signature = "(number, region=None, /)"]
pub fn parse(number: &str, region: Option<&str>) -> PyResult<PyPhoneNumber> {

    let region_id = match region {
        Some(reg) => Some(parse_region(reg)?),
        None => None,
    };

    match phonenumber::parse(region_id, number) {
        Ok(phone_number) => Ok(
            PyPhoneNumber::new(phone_number)
        ),
        Err(err) => Err(
            PyErr::new::<NumberParseException, _>(format!("{}", err))
        ),
    }
}

/// Tests whether a phone number matches a valid pattern.
///
/// Note this doesn't verify the number is actually in use, which is
/// impossible to tell by just looking at a number itself.  It only verifies
/// whether the parsed, canonicalised number is valid: not whether a
/// particular series of digits entered by the user is diallable from the
/// region provided when parsing. For example, the number +41 (0) 78 927 2696
/// can be parsed into a number with country code "41" and national
/// significant number "789272696". This is valid, while the original string
/// is not diallable.
///
/// Arguments:
/// numobj -- The phone number object that we want to validate
///
/// Returns a boolean that indicates whether the number is of a valid pattern.
#[pyfunction]
#[text_signature = "(numobj, /)"]
pub fn is_valid_number(numobj: &PyPhoneNumber) -> bool {
    numobj.is_valid()
}


/// Formats a phone number in the specified format using default rules.
///
/// Note that this does not promise to produce a phone number that the user
/// can dial from where they are - although we do format in either 'national'
/// or 'international' format depending on what the client asks for, we do not
/// currently support a more abbreviated format, such as for users in the same
/// "area" who could potentially dial the number without area code. Note that
/// if the phone number has a country calling code of 0 or an otherwise
/// invalid country calling code, we cannot work out which formatting rules to
/// apply so we return the national significant number with no formatting
/// applied.
///
/// Arguments:
/// numobj -- The phone number to be formatted.
/// num_format -- The format the phone number should be formatted into
///
/// Returns the formatted phone number.
#[pyfunction]
#[text_signature = "(numobj, num_format, /)"]
pub fn format_number(numobj: &PyPhoneNumber, num_format: &str) -> String {

    numobj.format(num_format)
}
