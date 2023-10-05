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
    interest_integer INT8 NOT NULL,
    interest_decimal INT8 NOT NULL,
    interest_exponent INT NOT NULL,
    interest_frequency INT NOT NULL,
    interest_frequency_unit interest_frequency_units NOT NULL
);