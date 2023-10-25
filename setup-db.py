import psycopg2

CREATE_DATABASE = """DROP DATABASE finance_db; CREATE DATABASE finance_db WITH OWNER finance_user;"""

INSERT_USER_TABLE = """CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    first_name TEXT,
    last_name TEXT,
    sales_tax NUMERIC,
    created_on TIMESTAMPTZ NOT NULL,
    last_modified_on TIMESTAMPTZ NOT NULL
);"""

INSERT_SECURITY_TABLE = """CREATE TABLE IF NOT EXISTS security (
    id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    user_password TEXT NOT NULL,
    user_access_token TEXT NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    last_modified_on TIMESTAMPTZ NOT NULL
);"""

INSERT_ACCOUNTTYPES_ENUM = """CREATE TYPE ACCOUNTTYPES AS ENUM(
    'checking',
    'saving'
);"""

INSERT_INTERESTFREQUENCYUNITS_ENUM = """CREATE TYPE INTERESTFREQUENCYUNITS AS ENUM(
    'day',
    'week',
    'month',
    'year'
);"""

INSERT_ACCOUNTS_TABLE = """CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) NOT NULL,
    account_type ACCOUNTTYPES NOT NULL,
    nickname TEXT NOT NULL,
    interest NUMERIC NOT NULL,
    interest_frequency NUMERIC NOT NULL,
    interest_frequency_unit INTERESTFREQUENCYUNITS NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    last_modified_on TIMESTAMPTZ NOT NULL
);"""

INSERT_TRANSACTIONTYPES_ENUM = """CREATE TYPE TRANSACTIONTYPES AS ENUM (
    'deposit',
    'withdraw',
    'expense',
    'transfer'
);"""

INSERT_TRANSACTIONCATEGORIES_ENUM = """CREATE TYPE TRANSACTIONCATEGORIES AS ENUM (
    'income',
    'dividend',
    'expense',
    'transfer',
    'interest'
);"""

INSERT_TRANSACTIONS_TABLE = """CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY,
    account_id UUID REFERENCES accounts(id) NOT NULL,
    transaction_date TIMESTAMPTZ NOT NULL,
    transaction_type TRANSACTIONTYPES NOT NULL,
    transaction_category TRANSACTIONCATEGORIES NOT NULL,
    amount NUMERIC NOT NULL,
    title TEXT NOT NULL,
    vendor TEXT,
    comment TEXT,
    created_on TIMESTAMPTZ NOT NULL,
    last_modified_on TIMESTAMPTZ NOT NULL
);"""

connection = psycopg2.connect(host="localhost", dbname="finance_db", user="finance_user", password="finance_pwd", port=5432)
cursor = connection.cursor()
cursor.execute(INSERT_USER_TABLE)
cursor.execute(INSERT_SECURITY_TABLE)
cursor.execute(INSERT_ACCOUNTTYPES_ENUM)
cursor.execute(INSERT_INTERESTFREQUENCYUNITS_ENUM)
cursor.execute(INSERT_ACCOUNTS_TABLE)
cursor.execute(INSERT_TRANSACTIONTYPES_ENUM)
cursor.execute(INSERT_TRANSACTIONCATEGORIES_ENUM)
cursor.execute(INSERT_TRANSACTIONS_TABLE)
connection.commit()