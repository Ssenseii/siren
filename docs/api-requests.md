# Api Requests

**Create User**

curl -X POST -H "Content-Type: application/json" -d '{"username": "user1", "email": "el.user@example.com", "password": "password1*"}' http://localhost:3000/api/user

**Login User**

curl -X POST -H "Content-Type: application/json" -d '{"username": "user1", "password": "password1*"}' http://localhost:3000/api/login

**Get Users List**

curl -X GET -H "Content-Type: application/json" http://localhost:3000/api/users

**Get One User**

curl -X GET -H "Content-Type: application/json" http://localhost:3000/api/user/6ffecd4b-6acd-4636-b689-b8464ba6f155
