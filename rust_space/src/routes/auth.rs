use crate::{db::Db, routes::video::ApiResponse, utils::jwt::generate_token};
use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::serde::{json::Json, Deserialize};
use rocket_db_pools::Connection;

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
pub async fn register(
    register_request: Json<RegisterRequest>,
    mut db: Connection<Db>,
) -> Result<Json<ApiResponse<&'static str>>, &'static str> {
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
            Ok(Json(ApiResponse {
                code: 0,
                message: Some("操作成功".to_string()),
                data: "",
            }))
        }
        Err(_) => Err("Failed to hash password"),
    }
}

#[post("/login", format = "json", data = "<login_request>")]
pub async fn login(
    login_request: Json<LoginRequest>,
    mut db: Connection<Db>,
) -> Result<Json<ApiResponse<String>>, &'static str> {
    match sqlx::query!(
        "SELECT username, id, password FROM users WHERE username = ?",
        login_request.username
    )
    .fetch_one(&mut **db)
    .await
    {
        Ok(user) => {
            // 验证密码是否正确
            if verify(&login_request.password, &user.password).unwrap_or(false) {
                let token = generate_token(user.username, user.id);
                Ok(Json(ApiResponse {
                    code: 0,
                    message: Some("操作成功".to_string()),
                    data: token,
                }))
            } else {
                Err("Invalid username or password")
            }
        }
        Err(_) => Err("User not found"),
    }
}
