import psycopg2
import requests
import json
from uuid_extensions import uuid7, uuid7str
from flask import Flask, request, jsonify, render_template
from decimal import *
from operator import itemgetter

connection = psycopg2.connect(host="localhost", dbname="finance_db", user="finance_user", password="finance_pwd", port=5432)
app = Flask(__name__)

@app.route("/")
def index():
    request = requests.get(url = "http://127.0.0.1:5000/api/account/type")
    account_types = request.json()
    negative_net_worth_account_types = ["Credit", "Loan"]
    negative_net_worth_account_type_ids = []
    for account_type in negative_net_worth_account_types:
        id = next((x for x in account_types if x['label'] == account_type), None)
        negative_net_worth_account_type_ids.append(id['id'])

    request = requests.get(url = "http://127.0.0.1:5000/api/account")
    accounts = request.json()

    negative_accounts = [account['id'] for account in accounts if account['account_type'] in negative_net_worth_account_type_ids]
    transactions = []

    for account in accounts:
        request = requests.get(url = "http://127.0.0.1:5000/api/account/" + account['id'] + "/transaction")
        account_transactions = request.json()
        account_total = sum([transaction['amount'] for transaction in account_transactions])
        account['total'] = account_total
        transactions = account_transactions + transactions

    net_worth = sum([transaction['amount'] for transaction in transactions if transaction['account_id'] not in negative_accounts]) - sum([transaction['amount'] for transaction in transactions if transaction['account_id'] in negative_accounts])

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

    request = requests.get("http://localhost:5000/api/account/type")
    account_types = request.json()

    interest_frequency_units_sql = """SELECT unnest(enum_range(NULL::interestfrequencyunits))::text"""

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
    request = requests.get(url = "http://localhost:5000/api/account")
    accounts = request.json()

    request = requests.get(url = "http://localhost:5000/api/transaction/type")
    transaction_types = request.json()
    transaction_types = sorted(transaction_types, key=itemgetter('label'))

    request = requests.get(url = "http://localhost:5000/api/transaction/category")
    transaction_categories = request.json()
    transaction_categories = sorted(transaction_categories, key=itemgetter('label'))

    return render_template("new_transaction.html", accounts=accounts, transaction_types=transaction_types, transaction_categories=transaction_categories)

@app.route("/account/type/new")
def new_account_type():
    return render_template("new_account_type.html")

@app.route("/transaction/type/new")
def new_transaction_type():
    return render_template("new_transaction_type.html")

@app.route("/transaction/category/new")
def new_transaction_category():
    return render_template("new_transaction_category.html")

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