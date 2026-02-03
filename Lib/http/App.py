import requests

# A Session object keeps cookies across multiple requests
session = requests.Session()

# 1. Ask the server to set a cookie
print("Requesting to set cookie...")
response1 = session.get("http://127.0.0.1")
print(f"Server says: {response1.text}")
print(f"Cookies currently in session: {session.cookies.get_dict()}\n")

# 2. Visit the 'get' route to see if the server recognizes the session
print("Requesting to read cookie back...")
response2 = session.get("http://127.0.0.1")
print(f"Server response: {response2.text}")
