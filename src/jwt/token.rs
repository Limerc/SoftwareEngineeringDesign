// use jsonwebtoken::{EncodingKey, DecodingKey, Header, Validation, encode, decode};
// use chrono::{Utc, Duration};
// use rocket::{get, post};
// use rocket::request::{self, Request, FromRequest};
// use rocket::http::Status;
// use rocket::outcome::Outcome;
// use serde::{Deserialize, Serialize};
// 
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//     sub: i32,  // 用户ID
//     exp: usize,   // 过期时间
// }
// 
// const SECRET: &str = "your-secret-key";
// 
// // 生成 JWT
// fn generate_token(user_id: i32) -> String {
//     let expiration = Utc::now()
//         .checked_add_signed(Duration::hours(24))
//         .expect("Invalid timestamp")
//         .timestamp();
// 
//     let claims = Claims {
//         sub: user_id,
//         exp: expiration as usize,
//     };
// 
//     encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET.as_bytes()))
//         .unwrap()
// }
// 
// // 验证 JWT
// fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
//     let token_data = decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(SECRET.as_bytes()),
//         &Validation::default(),
//     )?;
//     Ok(token_data.claims)
// }
// 
// struct JWTGuard {
//     claims: Claims,
// }
// 
// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for JWTGuard {
//     type Error = String;
// 
//     async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
//         // 从请求头获取 Authorization
//         let auth_header = request.headers().get_one("Authorization");
//         match auth_header {
//             Some(auth_header) => {
//                 let token = auth_header.strip_prefix("Bearer ").unwrap_or(auth_header);
//                 match validate_token(token) {
//                     Ok(claims) => Outcome::Success(JWTGuard { claims }),
//                     Err(e) => Outcome::Error((Status::Unauthorized, e.to_string())),
//                 }
//             }
//             None => Outcome::Error((Status::Unauthorized, "Missing Authorization header".into())),
//         }
//     }
// }
// 
// #[get("/protected")]
// pub fn protected_route(jwt: JWTGuard) -> String {
//     format!("Hello, user {}!", jwt.claims.sub)
// }
// 
// #[post("/login")]
// pub fn login() -> String {
//     // 假设用户验证成功，生成 Token
//     let user_id = 1;
//     generate_token(user_id)
// }