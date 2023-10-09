# TODO

## To-do List

- [ ] In the schema diagram, change Account type to enum
- [ ] In the schema diagram, add Compound Interest Units enum
- [ ] Transaction Items table

- [ ] Will need to rethink categories and types as enums, otherwise I cannot create categories without rebuilding the whole program. [Source](https://users.rust-lang.org/t/can-rust-build-enums-dynamically-at-runtime/58452/3)

## Ideas

- Create functions for handling numbers in scientific notation, multiply using FOIL
- Database design document
- Store numbers using scientific notation. This way there should be no rounding concerns. (Two signed int, one unsigned int). One signed int as the main integer, one unsigned int as the decimal, the last signed int as the exponent.
- Use auto-incrementing int as primary key but every row has a corresponding UUID which is used to figure out which entity we're looking for.
- Need to implement how often interest is compounded. Daily, weekly, yearly. Preferably this should be fully customizable. Like one field stores how many of X while the other field is the units (days, weeks, months, years)
- Upload images of recipts to the database so you can click on a transation and it'll tell you details about it, including the image.
- Create reoccurring charges (subscriptions, insurance, car payment, etc.) so you can plan future months in advance.
- Have account transfer transactions, this way money isn't double counted when being transferred into a savings account
- Have transaction categories (income, expense, gift, interest), make these customizable so you can create your own categories.
- Be able to select certain categories to count towards taxable income. For example, gifts shouldn't count but income, interest, dividends, should (depending on the account type).
- Should transactions store their cost value. With itemized transactions, even if you don't use itemized receipts it should create a single item with the price of the transaction as a whole.
- Have a sales tax value in the user table as the "default" sales tax. Every item should use that sales tax unless specified otherwise in the UI when filling out the item, or should that also be stored in the item table.
- Account user types (Admin, User, etc.). Not sure if it's really necessary. Especially for my use case.
- Some kind of enforcement for structure fields on the create methods for Mac
- Instead of having a stock API make it so that you can enter the stock's closing price and it will store that. Perhaps a UI with selecting the day and it'll show all currently owned stocks with their corresponding stock price.
- Create a way of importing CSV of yahoo's finance data.
- API url should be /account/{id}/transaction/{id}/item/{id}, do not include user uuid
