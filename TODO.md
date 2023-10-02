# Ideas

- Database design document
- Store numbers using scientific notation. This way there should be no rounding concerns. (Two signed int, one unsigned int). One signed int as the main integer, one unsigned int as the decimal, the last signed int as the exponent.
- Use auto-incrementing int as primary key but every row has a corresponding UUID which is used to figure out which entity we're looking for.
