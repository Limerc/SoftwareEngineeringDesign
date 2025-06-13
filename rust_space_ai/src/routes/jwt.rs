use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,          // 用户ID
    pub username: String, // 用户名
    exp: usize,           // 过期时间
} // token 格式

const SECRET: &str = "your-secret-key";

// 生成 JWT
pub fn generate_token(username: String, id: i32) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        username,
        id,
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .unwrap()
}

// 验证 JWT
fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub struct JWTGuard {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWTGuard {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // 从请求头获取 Authorization
        let auth_header = request.headers().get_one("Authorization");
        match auth_header {
            Some(auth_header) => {
                let token = auth_header.strip_prefix("Bearer ").unwrap_or(auth_header);
                match validate_token(token) {
                    Ok(claims) => rocket::outcome::Outcome::Success(JWTGuard { claims }),
                    Err(e) => {
                        rocket::outcome::Outcome::Error((Status::Unauthorized, e.to_string()))
                    }
                }
            }
            None => rocket::outcome::Outcome::Error((
                Status::Unauthorized,
                "Missing Authorization header".into(),
            )),
        }
    }
}
