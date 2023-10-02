# Personal Finance

This is an application I'm writing to handle my personal finances. This project will be separated into multiple "phases".
The backend is written in Rust with a PostgreSQL database.

## Phase 1

Implement basic accounts like checking, savings. This will handle the tracking of everyday transactions. Eventually paving the way to do itemized transactions to see exactly how much money you are spending.

There should be a UI associated with this step where you can select time periods to see how much money was received vs spent. YTD time period, last month, last year, last week, preferably completely customizable.

## Phase 2

Loans and debt. Credit cards fall in this phase.

## Phase 3

Stock Portfolio, this will be a challenge espesically if I want to include API usage to get a stocks current price. Probably will put this under it's own dashboard, separate from the rest of the program. Will need to incluce these in the total networth calculation however.

Different kinds of accounts like Roth IRA, 401(k), Roth 401(k), etc.

## Other

I want estimate capabilities for things like taxes and growth charts. Kind of like how ProjectionLabs works but more specific to my data and use cases.

This program will have statistical analysis tools. I want to be able to understand all aspects of my financial situation.

## Issues

If there are any mathematical calucation discrepencies please submit an issue because I want this to work 100% of the time.
