from flask import Flask, jsonify, request, Response

app = Flask(__name__)

loginDetails = {
    "alex": {
        "authToken": "123",
        "userName": "alex",
    },
    "jason": {
        "authToken": "234",
        "userName": "jason",
    },
    "marie": {
        "authToken": "345",
        "userName": "marie",
    },
}
coinDetails = {
    "alex": {
        "balance": 100,
    },
    "jason": {
        "balance": 200,
    },
    "marie": {
        "balance": 300,
    },
}


@app.before_request
def validateUser():

    path = request.path
    headers = request.headers
    route = path.split("/")
    if route[1] and route[1] == "coins":
        name = route[-1]
        if not headers.get("Authtoken") or not loginDetails[name]:
            return Response({"response": {"status": 403}}, status=403, mimetype='application/json')

@app.route("/")
def hello_world():
    return "HELLO WORLD"


@app.get("/test")
def return_json():
    json = {"message": "testing route", "code": 200}

    return jsonify(json)


@app.route("/coins/<userID>", methods=["GET", "POST"])
def tokens(userID):
    return jsonify({"data": {"balance": coinDetails[userID]["balance"]}})


if __name__ == "__main__":
    app.run(debug=True)
