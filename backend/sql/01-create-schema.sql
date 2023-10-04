-- Users table
CREATE TABLE users (
    id uuid PRIMARY KEY,
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
CREATE TYPE compound_interest_units AS ENUM (
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
    account_nickname text NOT NULL,
    interest_whole INT8 NOT NULL,
    interest_decimal INT8 NOT NULL,
    interest_decimal_exponent INT8 NOT NULL,
    compound_interest_value INT NOT NULL,
    compound_interest_unit compound_interest_units NOT NULL
);