# TODO

## To-do List

- [ ] In the schema diagram, change Account type to enum
- [ ] In the schema diagram, add Compound Interest Units enum
- [ ] Transaction Items table

- [ ] Will need to rethink categories and types as enums, otherwise I cannot create categories without rebuilding the whole program. [Source](https://users.rust-lang.org/t/can-rust-build-enums-dynamically-at-runtime/58452/3)
- [ ] Currency should be implemented for transactions table

## Ideas

- Database design document
- Upload images of receipts to the database, so you can click on a transaction, and it'll tell you details about it, including the image.
- Create reoccurring charges (subscriptions, insurance, car payment, etc.) so you can plan future months in advance.
- Have account transfer transactions, this way money isn't doubly counted when being transferred into a saving account
- Have transaction categories (income, expense, gift, interest), make these customizable, so you can create your own categories.
- Be able to select certain categories to count towards taxable income. For example, gifts shouldn't count but income, interest, dividends, should (depending on the account type).
- Should transactions store their cost value? With itemized transactions, even if you don't use itemized receipts, it should create a single item with the price of the transaction as a whole.
- Have a sales tax value in the user table as the "default" sales tax. Every item should use that sales tax unless specified otherwise in the UI when filling out the item, or should that also be stored in the item table.
- Account user types (Admin, User, etc.). Not sure if it's really necessary. Especially for my use case.
- Some kind of enforcement for structure fields on the create methods for Mac
- Instead of having a stock API make it so that you can enter the stock's closing price, and it will store that. Perhaps a UI with selecting the day, and it'll show all currently owned stocks with their corresponding stock price.
- Create a way of importing CSV of yahoo's finance data.
- Move from warp over to poem-openapi, swagger api documents are auto-generated with this crate. [Crate](https://crates.io/crates/poem-openapi)
- Implement rust_decimal, include features db-postgres, db-tokio-postgres, serde-with-*. [Crate](https://crates.io/crates/rust_decimal)
