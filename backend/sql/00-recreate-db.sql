-- DEV ONLY - Comment out for persisting db
DROP DATABASE IF EXISTS finance_db;
DROP USER IF EXISTS finance_user;

-- DEV ONLY - For quick iteration
CREATE USER finance_user PASSWORD 'finance_pwd';
CREATE DATABASE finance_db owner finance_user ENCODING = 'UTF-8';