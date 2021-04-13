import faker
from faker.config import AVAILABLE_LOCALES


num = 100_000

exclude = {'en_PH', 'fil_PH'}


def gen():
    for locale in AVAILABLE_LOCALES:
        if locale in exclude:
            continue
        print(locale)
        fake = faker.Faker(locale=locale)
        try:
            fake.phone_number()
        except AttributeError:
            print(f'Skipped: ^^^')
            continue
        with open(f'data/{locale}', 'w') as db:
            for _ in range(num):
                db.write(f'{fake.phone_number()}\n')


if __name__ == '__main__':
    gen()
