-- Dev seed
INSERT INTO users (id, first_name, last_name, username) VALUES ('00000000-0000-0000-0000-000000000000', 'Justin', 'Leahy', 'justinleahy');
INSERT INTO accounts (id, user_id, account_type, nickname, interest_integer, interest_decimal, interest_exponent, interest_frequency, interest_frequency_unit)
    VALUES ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000000', 'checking', 'Main Checking', 1, 0, -3, 1, 'day');
INSERT INTO transactions (id, account_id, transaction_date, transaction_type, category, transaction_integer, transaction_decimal, transaction_exponent, comment)
    VALUES ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001', '2023-10-06', 'transfer', 'income', 8, 9966, 2, 'Webull Deposit')