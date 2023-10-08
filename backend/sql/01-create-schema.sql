-- Users table
CREATE TABLE users (
    id uuid PRIMARY KEY,
    password_hash text NOT NULL,
    user_context text NOT NULL,
    first_name text NOT NULL,
    last_name text NOT NULL,
    username text NOT NULL
);

-- Account Type enum
CREATE TYPE account_types AS ENUM (
    'checking',
    'savings'
);

-- Compound Interest Unit enum
CREATE TYPE interest_frequency_units AS ENUM (
    'day',
    'week',
    'month',
    'year'
);

-- Account table
CREATE TABLE accounts (
    id uuid PRIMARY KEY,
    user_id uuid REFERENCES users(id),
    account_type account_types NOT NULL,
    nickname text NOT NULL,
    interest float NOT NULL,
    interest_frequency INT NOT NULL,
    interest_frequency_unit interest_frequency_units NOT NULL
);

CREATE TYPE transaction_types AS ENUM (
    'deposit',
    'withdraw',
    'expense',
    'transfer'
);

CREATE TYPE transaction_categories AS ENUM (
    'income',
    'dividend',
    'transfer',
    'expense'
);

-- Transaction table
CREATE TABLE transactions (
    id uuid PRIMARY KEY,
    account_id uuid REFERENCES accounts(id),
    transaction_date TIMESTAMPTZ not null,
    transaction_type transaction_types not null,
    category transaction_categories not null,
    amount float not null,
    comment text
);