# TODO

## To-do List

- [ ] In the schema diagram, change Account type to enum
- [ ] In the schema diagram, add Compound Interest Units enum

## Ideas

- Database design document
- Store numbers using scientific notation. This way there should be no rounding concerns. (Two signed int, one unsigned int). One signed int as the main integer, one unsigned int as the decimal, the last signed int as the exponent.
- Use auto-incrementing int as primary key but every row has a corresponding UUID which is used to figure out which entity we're looking for.
- Need to implement how often interest is compounded. Daily, weekly, yearly. Preferably this should be fully customizable. Like one field stores how many of X while the other field is the units (days, weeks, months, years)
- Upload images of recipts to the database so you can click on a transation and it'll tell you details about it, including the image.
