from uuid_extensions import uuid7str
from flask import Blueprint, jsonify, request
from main import connection

users = Blueprint("users", __name__)

POST_USER = """INSERT INTO users (id, username, first_name, last_name, sales_tax, created_on, last_modified_on) VALUES (%s, %s, %s, %s, %s, now(), now()) RETURNING id"""
GET_USER = """SELECT * from users WHERE id = (%s)"""
LIST_USERS = """SELECT * from users"""

@users.route("/api/user/<user_uuid>", methods=["GET"])
def get_user(user_uuid):
    with connection:
        with connection.cursor() as cursor:
            cursor.execute(GET_USER, (user_uuid,))
            raw_user_data = cursor.fetchone()
            if raw_user_data:
                user_data = {
                    "uuid" : raw_user_data[0],
                    "username" : raw_user_data[1],
                    "first_name" : raw_user_data[2],
                    "last_name" : raw_user_data[3],
                    "sales_tax" : raw_user_data[4],
                    "created_on" : raw_user_data[5],
                    "last_modified_on" : raw_user_data[6]
                }
            else:
                return "User not found", 400

    return jsonify(user_data), 200

@users.route("/api/user", methods=["POST"])
def post_user():
    data = request.get_json()

    with connection:
        with connection.cursor() as cursor:
            user_id = uuid7str()
            cursor.execute(POST_USER, (user_id, data['username'], data['first_name'], data['last_name'], data['sales_tax']))
            raw_user_id = cursor.fetchone()[0]
            user_id = {
                "id" : raw_user_id
            }

    return jsonify(user_id), 201

@users.route("/api/user", methods=["GET"])
def list_accounts():
    with connection:
        with connection.cursor() as cursor:
            cursor.execute(LIST_USERS)
            raw_users_data = cursor.fetchall()
            users_data = []
            for raw_user_data in raw_users_data:
                user_data = {
                    "id" : raw_user_data[0],
                    "username" : raw_user_data[1],
                    "first_name" : raw_user_data[2],
                    "last_name" : raw_user_data[3],
                    "sales_tax" : raw_user_data[4],
                    "created_on" : raw_user_data[5],
                    "last_modified_on" : raw_user_data[6]
                }
                users_data.append(user_data)

    return jsonify(users_data), 200