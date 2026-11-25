# Api Requests

**Create User**

curl -X POST -H "Content-Type: application/json" -d '{"username": "user1", "email": "el.user@example.com", "password": "password1*"}' http://localhost:3000/api/user

**Login User**

curl -X POST -H "Content-Type: application/json" -d '{"username": "user1", "password": "password1*"}' http://localhost:3000/api/login
