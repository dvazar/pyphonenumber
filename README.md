pyphonenumber Python Library
============================

These are bindings to the [phonenumber](https://github.com/rustonaut/rust-phonenumber)
library (Rust version of [libphonenumber](https://github.com/googlei18n/libphonenumber)
by Google).

It supports Python 3.6+.


Example Usage
-------------

The main object that the library deals with is a `PhoneNumber` object.  You can
create this from a string representing a phone number using the `parse` function,
but you also need to specify the country that the phone number is being dialled
from (unless the number is in E.164 format, which is globally unique).

```pycon
>>> import phonenumber
>>> x = phonenumber.parse('+442083661177')
>>> print(x)
Country Code: 44 National Number: 2083661177
>>> type(x)
<class 'phonenumber.PhoneNumber'>
>>> y = phonenumber.parse('020 8366 1177', 'GB')
>>> print(y)
Country Code: 44 National Number: 2083661177
>>> x == y
True
>>> z = phonenumber.parse('00 1 650 253 2222, Ext. 123', 'GB')  # as dialled from GB, not a GB number
>>> print(z)
Country Code: 1 National Number: 6502532222 Extension: 123
```

The `PhoneNumber` object that `parse` produces typically still needs to be validated,
to check whether it's a *valid* number (e.g. it's in an assigned exchange).

```pycon
>>> z = phonenumber.parse('+120012301')
>>> print(z)
Country Code: 1 National Number: 20012301
>>> phonenumber.is_valid_number(z)
False
>>> z = phonenumber.parse('+12001230101')
>>> print(z)
Country Code: 1 National Number: 2001230101
>>> phonenumber.is_valid_number(z)  # NPA 200 not used
False
```

The `parse` function will also fail completely (with a `NumberParseException`)
on inputs that cannot be uniquely parsed, or that  can't possibly be phone numbers.

```pycon
>>> z = phonenumber.parse('02081234567')  # no region, no + => unparseable
Traceback (most recent call last):
  File "<input>", line 1, in <module>
phonenumber.NumberParseException: invalid country code
>>> z = phonenumber.parse('gibberish')
Traceback (most recent call last):
  File "<input>", line 1, in <module>
phonenumber.NumberParseException: not a number
```

Once you've got a phone number, a common task is to format it in a standardized format.
There are a few formats available (under `PhoneNumberFormat`), and the `format_number`
function does the formatting.

```pycon
>>> phonenumber.format_number(x, phonenumber.PhoneNumberFormat.NATIONAL)
'020 8366 1177'
>>> phonenumber.format_number(x, phonenumber.PhoneNumberFormat.INTERNATIONAL)
'+44 20 8366 1177'
>>> phonenumber.format_number(x, phonenumber.PhoneNumberFormat.E164)
'+442083661177'
>>> phonenumber.format_number(x, phonenumber.PhoneNumberFormat.RFC3966)
'tel:+44-20-8366-1177'
```
