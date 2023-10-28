from uuid_extensions import uuid7str
from flask import Blueprint, jsonify, request
from main import connection
from datetime import datetime

GET_TRANSACTION = """SELECT * FROM transactions WHERE account_id = (%s) AND id = (%s)"""
LIST_TRANSACTIONS = """SELECT * FROM transactions WHERE account_id = (%s)"""
POST_TRANSACTION = """INSERT INTO transactions (id, account_id, transaction_date, transaction_type, transaction_category, amount, title, created_on, last_modified_on) VALUES (%s, %s, %s, %s, %s, %s, %s, now(), now()) RETURNING id"""

LIST_TRANSACTION_TYPES = """SELECT * FROM transaction_types"""
POST_TRANSACTION_TYPE = """INSERT INTO transaction_types (label) VALUES (%s) RETURNING id"""

LIST_TRANSACTION_CATEGORIES = """SELECT * FROM transaction_categories"""
POST_TRANSACTION_CATEGORY = """INSERT INTO transaction_categories (label) VALUES (%s) RETURNING id"""

UPDATE_VENDOR = """UPDATE transactions SET vendor = (%s) WHERE id = (%s)"""
UPDATE_COMMENT = """UPDATE transactions SET comment = (%s) WHERE id = (%s)"""

transactions = Blueprint("transactions", __name__)

@transactions.route("/api/account/<account_uuid>/transaction/<transaction_uuid>", methods=["GET"])
def get_transaction(account_uuid, transaction_uuid):
    with connection:
        with connection.cursor() as cursor:
            from accounts import GET_ACCOUNT
            cursor.execute(GET_ACCOUNT, (account_uuid,))
            raw_account_data = cursor.fetchone()
            print(raw_account_data)
            account_id = raw_account_data[0]
            cursor.execute(GET_TRANSACTION, (account_id, transaction_uuid,))
            raw_transaction_data = cursor.fetchone()
            if raw_transaction_data:
                transaction_data = {
                    "id" : raw_transaction_data[0],
                    "account_id" : raw_transaction_data[1],
                    "transaction_date" : datetime.strptime(raw_transaction_data[2], "%y-%m-%d").date(),
                    "transaction_type" : raw_transaction_data[3],
                    "transaction_category" : raw_transaction_data[4],
                    "amount" : float(raw_transaction_data[5]),
                    "title" : raw_transaction_data[6],
                    "vendor" : raw_transaction_data[7],
                    "comment" : raw_transaction_data[8],
                    "created_on" : raw_transaction_data[9],
                    "last_modified_on" : raw_transaction_data[10]
                }
            else:
                return "Transaction not found", 400

    return jsonify(transaction_data), 200

@transactions.route("/api/account/<account_uuid>/transaction", methods=["GET"])
def list_transactions(account_uuid):
    with connection:
        with connection.cursor() as cursor:
            cursor.execute(LIST_TRANSACTIONS, (account_uuid, ))
            raw_transactions_data = cursor.fetchall()
            transactions_data = []
            for raw_transaction_data in raw_transactions_data:
                transaction_data = {
                    "id" : raw_transaction_data[0],
                    "account_id" : raw_transaction_data[1],
                    "transaction_date" : raw_transaction_data[2].strftime("%Y-%m-%d"),
                    "transaction_type" : raw_transaction_data[3],
                    "transaction_category" : raw_transaction_data[4],
                    "amount" : float(raw_transaction_data[5]),
                    "title" : raw_transaction_data[6],
                    "vendor" : raw_transaction_data[7],
                    "comment" : raw_transaction_data[8],
                    "created_on" : raw_transaction_data[9],
                    "last_modified_on" : raw_transaction_data[10]
                }
                transactions_data.append(transaction_data)
                
            
    
    return jsonify(transactions_data), 200

@transactions.route("/api/account/<account_uuid>/transaction", methods=["POST"])
def post_transaction(account_uuid):
    data = request.get_json()

    with connection:
        with connection.cursor() as cursor:
            transaction_id = uuid7str()
            cursor.execute(POST_TRANSACTION, (transaction_id, account_uuid, data['transaction_date'], data['transaction_type'], data['transaction_category'], data['amount'], data['title']))
            raw_transaction_id = cursor.fetchone()[0]
            transaction_id = {
                "id" : raw_transaction_id
            }
            if "vendor" in data:
                cursor.execute(UPDATE_VENDOR, (data['vendor'], raw_transaction_id))

            if "comment" in data:
                cursor.execute(UPDATE_COMMENT, (data['comment'], raw_transaction_id))

    return jsonify(transaction_id), 201

@transactions.route("/api/transaction/type", methods=["POST"])
def post_transaction_type():
    data = request.get_json()

    with connection:
        with connection.cursor() as cursor:
            cursor.execute(POST_TRANSACTION_TYPE, (data['label'], ))
            raw_transaction_type_id = cursor.fetchone()[0]
            transaction_type_id = {
                "id" : raw_transaction_type_id
            }
    
    return jsonify(transaction_type_id), 200

@transactions.route("/api/transaction/type", methods=["GET"])
def list_transaction_types():
    with connection:
        with connection.cursor() as cursor:
            cursor.execute(LIST_TRANSACTION_TYPES)
            raw_transaction_types_data = cursor.fetchall()
            transaction_types_data = []
            for raw_transaction_type_data in raw_transaction_types_data:
                transaction_type_data = {
                    "id" : raw_transaction_type_data[0],
                    "label" : raw_transaction_type_data[1]
                }
                transaction_types_data.append(transaction_type_data)

    return jsonify(transaction_types_data), 200

@transactions.route("/api/transaction/category", methods=["POST"])
def post_transaction_category():
    data = request.get_json()

    with connection:
        with connection.cursor() as cursor:
            cursor.execute(POST_TRANSACTION_CATEGORY, (data['label'], ))
            raw_transaction_category_id = cursor.fetchone()[0]
            transaction_category_id = {
                "id" : raw_transaction_category_id
            }
    
    return jsonify(transaction_category_id), 200

@transactions.route("/api/transaction/category", methods=["GET"])
def list_transaction_categories():
    with connection:
        with connection.cursor() as cursor:
            cursor.execute(LIST_TRANSACTION_CATEGORIES)
            raw_transaction_categories_data = cursor.fetchall()
            transaction_categories_data = []
            for raw_transaction_category_data in raw_transaction_categories_data:
                transaction_category_data = {
                    "id" : raw_transaction_category_data[0],
                    "label" : raw_transaction_category_data[1]
                }
                transaction_categories_data.append(transaction_category_data)

    return jsonify(transaction_categories_data), 200