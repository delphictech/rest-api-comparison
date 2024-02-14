from flask import Flask, jsonify

app = Flask(__name__)


@app.route("/")
def hello_world():
    return "HELLO WORLD"


@app.get("/test")
def return_json():
    json = {"message": "testing route", "code": 200}

    return jsonify(json)


@app.route("/coins/<userID>", methods=["GET", "POST"])
def tokens():
    pass
