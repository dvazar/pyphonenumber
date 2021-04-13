import importlib
import os


def emit_fake_phone_number():
    for path, dirs, files in os.walk('data'):
        for file in files:
            with open(os.path.join(path, file)) as f:
                for line in f:
                    line = line.rstrip()
                    if line:
                        yield file.upper()[-2:], line


def test_lib(name):
    lib = importlib.import_module(name)

    for region, number_str in emit_fake_phone_number():
        try:
            number_obj = lib.parse(number_str, region)
        except lib.NumberParseException as exc:
            continue
        lib.is_valid_number(number_obj)
        lib.format_number(
            number_obj, lib.PhoneNumberFormat.INTERNATIONAL,
        )


def main():
    test_lib('phonenumber')


if __name__ == '__main__':
    main()
