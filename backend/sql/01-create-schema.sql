-- User table
CREATE TABLE user {
    id SERIAL PRIMARY KEY,
    first_name text NOT NULL,
    last_name text NOT NULL,
    user_name text NOT NULL,
    user_uuid uuid DEFAULT uuid_generate_v4 ()
}

-- Account Type enum
CREATE TYPE account_type AS ENUM {
    'checking',
    'savings'
}

-- Compound Interest Unit enum
CREATE TYPE compound_interest_unit AS ENUM {
    'day',
    'week',
    'month',
    'year'
}

-- Account table
CREATE TABLE account {
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES user(id),
    account_type account_type NOT NULL,
    account_nickname text NOT NULL,
    account_uuid uuid DEFAULT uuid_generate_v4 (),
    interest_whole INT8 NOT NULL,
    interest_decimal INT8 NOT NULL,
    interest_decimal_exponent INT8 NOT NULL,
    compound_interest_value INT NOT NULL,
    compound_interest_unit compound_interest_unit NOT NULL
}