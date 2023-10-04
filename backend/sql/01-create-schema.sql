-- uuid_generate_v4 function
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name text NOT NULL,
    last_name text NOT NULL,
    username text NOT NULL,
    user_uuid uuid DEFAULT uuid_generate_v4 ()
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
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id),
    account_type account_types NOT NULL,
    account_nickname text NOT NULL,
    account_uuid uuid DEFAULT uuid_generate_v4 (),
    interest_whole INT8 NOT NULL,
    interest_decimal INT8 NOT NULL,
    interest_decimal_exponent INT8 NOT NULL,
    compound_interest_value INT NOT NULL,
    compound_interest_unit compound_interest_units NOT NULL
);