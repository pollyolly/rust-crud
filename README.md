### Notes URL
```
http://localhost:3000/user_list/{offset}/{limit}
GET: /user_list/0/10
http://localhost:3000/user_by_id/{id}
GET: /user_by_id/1
http://localhost:3000/user_create
POST:
{
    "name": "jake",
    "occupation":"ceo",
    "email":"jake@gmail.com",
    "phone":"7878"
}
http://localhost:3000/user_update
PUT:
{
    "name": "jake demo",
    "occupation":"ceo demo",
    "email":"jake-demo@gmail.com",
    "phone":"7878-demo",
    "id":1
} 
http://localhost:3000/user_delete/1
GET: user_delete/1
http://localhost:3000/upload
GET: file
http://localhost:3000/download/{filename}
GET: donwload/filename.txt
http://localhost:3000/user_info
GET: /user_info
http://localhost:3000/login
GET: /login
```
### Cargo
```
[package]
name = "rust_crud"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = { version = "0.8.1", features = ["multipart", "macros"] }
serde = {version="1.0.196", features=["derive"]}
serde_json = "1.0.113"
tokio = {version="1.36.0", features = ["full"]}
sqlx = {version="0.8.6", features=["postgres", "macros", "runtime-tokio-native-tls"]}
tower-http = { version = "0.5.2", features = ["cors"] }
mime_guess = "2.0.5"
tokio-util = {version="0.7.15", features = ["io"] }
chrono = "0.4.41"
jsonwebtoken = "9.3.1"
axum-extra = { version ="0.10.1", features=["typed-header"] }
axum-jwt-auth = "0.5.1"
```
### JWT
Generate RSA (public and key) for JWT
```
$openssl genrsa -out private_key.pem 2048
$openssl rsa -in private_key.pem -pubout -out public_key.pem
```
### Reference
Tutorial Link

https://www.youtube.com/watch?v=VHNdLXCyOPI&list=PLDi2liHqCnVqjPWcdAP-Qyvb3pKyYlZ0Y&index=2
