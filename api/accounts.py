from uuid_extensions import uuid7str
from flask import Blueprint, jsonify, request
from main import connection

GET_ACCOUNT = """SELECT * FROM accounts WHERE id = (%s)"""
LIST_ACCOUNTS = """SELECT * FROM accounts"""
POST_ACCOUNT = """INSERT INTO accounts (id, user_id, account_type, nickname, interest, interest_frequency, interest_frequency_unit, created_on, last_modified_on) VALUES (%s, %s, %s, %s, %s, %s, %s, now(), now()) RETURNING id"""

GET_ACCOUNT_TYPE = """SELECT * from account_types WHERE id = (%s)"""
LIST_ACCOUNT_TYPES = """SELECT * from account_types"""
POST_ACCOUNT_TYPE = """INSERT INTO account_types (label) VALUES (%s) RETURNING id"""

accounts = Blueprint("accounts", __name__)

@accounts.route("/api/account/<account_uuid>", methods=["GET"])
def get_account(account_uuid):
    with connection:
        with connection.cursor() as cursor:
            cursor.execute(GET_ACCOUNT, (account_uuid,))
            raw_account_data = cursor.fetchone()
            if raw_account_data:
                account_data = {
                    "id" : raw_account_data[0],
                    "user_id" : raw_account_data[1],
                    "account_type" : raw_account_data[2],
                    "nickname" : raw_account_data[3],
                    "interest" : float(raw_account_data[4]),
                    "interest_frequency" : int(raw_account_data[5]),
                    "interest_frequency_unit" : raw_account_data[6],
                    "created_on" : raw_account_data[7],
                    "last_modified_on" : raw_account_data[8]
                }
            else:
                return "Account not found", 400

    return jsonify(account_data), 200

@accounts.route("/api/account", methods = ["POST"])
def post_account():
    data = request.get_json()

    with connection:
        with connection.cursor() as cursor:
            account_id = uuid7str()
            cursor.execute(POST_ACCOUNT, (account_id, data['user_id'], data['account_type'], data['nickname'], data['interest'], data['interest_frequency'], data['interest_frequency_unit']))
            raw_account_id = cursor.fetchone()[0]
            account_id = {
                "id" : raw_account_id
            }

    return jsonify(account_id), 201

@accounts.route("/api/account", methods=["GET"])
def list_accounts():
    with connection:
        with connection.cursor() as cursor:
            cursor.execute(LIST_ACCOUNTS)
            raw_accounts_data = cursor.fetchall()
            accounts_data = []
            for raw_account_data in raw_accounts_data:
                account_data = {
                    "id" : raw_account_data[0],
                    "user_id" : raw_account_data[1],
                    "account_type" : raw_account_data[2],
                    "nickname" : raw_account_data[3],
                    "interest" : float(raw_account_data[4]),
                    "interest_frequency" : int(raw_account_data[5]),
                    "interest_frequency_unit" : raw_account_data[6],
                    "created_on" : raw_account_data[7],
                    "last_modified_on" : raw_account_data[8]
                }
                accounts_data.append(account_data)


    return jsonify(accounts_data), 200

@accounts.route("/api/account/type", methods=["POST"])
def post_account_type():
    data = request.get_json()
    
    with connection:
        with connection.cursor() as cursor:
            cursor.execute(POST_ACCOUNT_TYPE, (data['label'], ))
            raw_account_type_id = cursor.fetchone()[0]
            account_type_id = {
                "id" : raw_account_type_id
            }

    return jsonify(account_type_id), 200

@accounts.route("/api/account/type", methods=["GET"])
def list_account_types():
    with connection:
        with connection.cursor() as cursor:
            cursor.execute(LIST_ACCOUNT_TYPES)
            raw_account_types_data = cursor.fetchall()
            account_types_data = []
            for raw_account_type_data in raw_account_types_data:
                account_type_data = {
                    "id" : raw_account_type_data[0],
                    "label" : raw_account_type_data[1]
                }
                account_types_data.append(account_type_data)

    return jsonify(account_types_data), 200