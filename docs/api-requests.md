# Api Requests

**Hello**

curl -X GET -H "Content-Type: application/json" http://localhost:3000/api/hello

**Create User**

curl -X POST -H "Content-Type: application/json" -d '{"username": "user1", "email": "el.user@example.com", "password": "password1*"}' http://localhost:3000/api/user

**Update User**

curl -X PATCH -H "Content-Type: application/json" -d '{"username": "user1", "old_password": "password1*", "new_password": "password2*"}' http://localhost:3000/api/user/37b18f0b-7ee8-49b4-8368-9d03264a40e9

**Delete User**

curl -X DELETE -H "Content-Type: application/json" http://localhost:3000/api/user/37b18f0b-7ee8-49b4-8368-9d03264a40e9

**Get Users List**

curl -X GET -H "Content-Type: application/json" http://localhost:3000/api/users

**Get One User**

curl -X GET -H "Content-Type: application/json" http://localhost:3000/api/user/37b18f0b-7ee8-49b4-8368-9d03264a40e9

**Login User**

curl -X POST -H "Content-Type: application/json" -d '{"username": "user1", "password": "password2*"}' http://localhost:3000/api/login