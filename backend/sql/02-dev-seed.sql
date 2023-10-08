-- Dev seed
INSERT INTO users (id, first_name, last_name, username) VALUES ('00000000-0000-0000-0000-000000000000', 'Justin', 'Leahy', 'justinleahy');
INSERT INTO accounts (id, user_id, account_type, nickname, interest, interest_frequency, interest_frequency_unit)
    VALUES ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000000', 'checking', 'Main Checking', 0.0009995, 1, 'day');
INSERT INTO transactions (id, account_id, transaction_date, transaction_type, category, amount, comment)
    VALUES ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001', '2023-10-06', 'transfer', 'transfer', 899.66, 'Webull Transfer');
INSERT INTO transactions (id, account_id, transaction_date, transaction_type, category, amount, comment)
    VALUES ('00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000001', '2023-10-07', 'expense', 'expense', 100.43, 'Tops');