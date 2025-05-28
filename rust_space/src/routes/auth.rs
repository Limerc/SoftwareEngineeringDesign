use rocket::serde::{Deserialize, json::Json};
use rocket_db_pools::Connection;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::db::Db;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[post("/register", format = "json", data = "<register_request>")]
pub async fn register(register_request: Json<RegisterRequest>, mut db: Connection<Db>) -> Result<&'static str, &'static str> {
    match hash(&register_request.password, DEFAULT_COST) {
        Ok(hashed_password) => {
            sqlx::query!(
                "INSERT INTO users (username, password) VALUES (?, ?)",
                register_request.username,
                hashed_password
            )
            .execute(&mut **db)
            .await
            .map_err(|_| "Failed to insert user")?;
            Ok("User registered successfully")
        },
        Err(_) => Err("Failed to hash password"),
    }
}

#[post("/login", format = "json", data = "<login_request>")]
pub async fn login(login_request: Json<LoginRequest>, mut db: Connection<Db>) -> Result<&'static str, &'static str> {
    match sqlx::query!(
        "SELECT password FROM users WHERE username = ?",
        login_request.username
    )
    .fetch_one(&mut **db)
    .await {
        Ok(user) => {
            // 验证密码是否正确
            if verify(&login_request.password, &user.password).unwrap_or(false) {
                Ok("Login successful")
            } else {
                Err("Invalid username or password")
            }
        },
        Err(_) => Err("User not found"),
    }
}
