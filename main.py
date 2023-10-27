import psycopg2
import requests
from uuid_extensions import uuid7, uuid7str
from flask import Flask, request, jsonify, render_template
from decimal import *

connection = psycopg2.connect(host="localhost", dbname="finance_db", user="finance_user", password="finance_pwd", port=5432)
app = Flask(__name__)

@app.route("/")
def index():
    request = requests.get(url = "http://127.0.0.1:5000/api/account")
    accounts = request.json()

    transactions = []

    for account in accounts:
        request = requests.get(url = "http://127.0.0.1:5000/api/account/" + account['id'] + "/transaction")
        account_transactions = request.json()
        account_total = sum([transaction['amount'] for transaction in account_transactions])
        account['total'] = account_total
        transactions = account_transactions + transactions

    net_worth = sum([transaction['amount'] for transaction in transactions])

    return render_template('index.html', net_worth=net_worth, accounts=accounts)

@app.route("/account/<account_uuid>")
def account_overview(account_uuid):
    request = requests.get(url = "http://127.0.0.1:5000/api/account/" + account_uuid)
    account = request.json()
    request = requests.get(url = "http://127.0.0.1:5000/api/account/" + account_uuid + "/transaction")
    transactions = request.json()

    account_worth = sum([transaction['amount'] for transaction in transactions])

    return render_template('account_overview.html', account=account, account_worth=account_worth, transactions=transactions)

@app.route("/account/new")
def new_account():
    request = requests.get("http://127.0.0.1:5000/api/user")
    users = request.json()

    account_types_sql = """SELECT unnest(enum_range(NULL::accounttypes))::text"""
    interest_frequency_units_sql = """SELECT unnest(enum_range(NULL::interestfrequencyunits))::text"""

    with connection:
        with connection.cursor() as cursor:
            cursor.execute(account_types_sql)
            raw_account_types = cursor.fetchall()

    account_types = []

    for account_type in raw_account_types:
        account_types.append(account_type[0].capitalize())

    account_types.sort()

    with connection:
        with connection.cursor() as cursor:
            cursor.execute(interest_frequency_units_sql)
            raw_interest_frequency_units = cursor.fetchall()

    interest_frequency_units = []

    for interest_frequency_unit in raw_interest_frequency_units:
        interest_frequency_units.append(interest_frequency_unit[0].capitalize())

    return render_template('new_account.html', users=users, account_types=account_types, interest_frequency_units=interest_frequency_units)

@app.route("/user/new")
def new_user():

    return render_template('new_user.html')

@app.route("/transaction/new")
def new_transaction():
    request = requests.get(url = "http://127.0.0.1:5000/api/account")
    accounts = request.json()

    transaction_types_sql = """SELECT unnest(enum_range(NULL::transactiontypes))::text"""
    transaction_categories_sql = """SELECT unnest(enum_range(NULL::transactioncategories))::text"""

    with connection:
        with connection.cursor() as cursor:
            cursor.execute(transaction_types_sql)
            raw_transaction_types = cursor.fetchall()

    transaction_types = []

    for transaction_type in raw_transaction_types:
        transaction_types.append(transaction_type[0].capitalize())

    transaction_types.sort()

    with connection:
            with connection.cursor() as cursor:
                cursor.execute(transaction_categories_sql)
                raw_transaction_categories = cursor.fetchall()

    transaction_categories = []

    for transaction_category in raw_transaction_categories:
        transaction_categories.append(transaction_category[0].capitalize())

    transaction_categories.sort()

    return render_template("new_transaction.html", accounts=accounts, transaction_types=transaction_types, transaction_categories=transaction_categories)

def create_app():
    from api.users import users as users_blueprint
    app.register_blueprint(users_blueprint)

    from api.accounts import accounts as accounts_blueprint
    app.register_blueprint(accounts_blueprint)

    from api.transactions import transactions as transactions_blueprint
    app.register_blueprint(transactions_blueprint)

    return app

def main():
    app = create_app()
    app.run()
    connection.close()

if __name__ == "__main__":
    main()