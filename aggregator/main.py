from flask import Flask, jsonify, request
import requests
import os


# External API URLS
MS_VERBS = os.environ.get("VERBS_URL")
MS_NOUNS = os.environ.get("NOUNS_URL")
app = Flask(__name__)

@app.route("/")
def aggregate():
    try:
        response = requests.get(MS_VERBS)
        verb = response.json()
            
        response = requests.get(MS_NOUNS)
        noun = response.json()
        return f"<p>{verb} {noun}</p>"
    except:
        return f"<p>An error occured :c</p>"



if __name__ == '__main__':
    # Run the Flask app
    app.run(debug=False, port=3000)