use reqwest::Client;

use futures::{Stream, StreamExt};

use rocket::{get, post, put, response::stream::TextStream, serde::json::Json};

use super::query::{GetHeader, Message, PostData, QueryTable};
use rocket::http::Status;
use rocket::request::{FromRequest, Request};
use rocket::serde::Deserialize;

use super::token::*;

#[derive(Debug, Deserialize)]
pub struct CodeRequest {
    code: Option<String>,
    question: String,
} // 请求数据

use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rocket::request::{self};

use crate::llm::AIHistory::{get_ai_history, insert_ai_history};
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: i32, // 用户ID
    exp: usize,   // 过期时间
} // token 格式

const SECRET: &str = "your-secret-key";

// 生成 JWT
fn generate_token(user_id: i32) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        user_id: user_id.to_owned(),
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

struct JWTGuard {
    claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWTGuard {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
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

#[post("/api/ai/ask", format = "json", data = "<data>")]
pub async fn processed_stream(
    _key: JWTGuard,
    data: Json<CodeRequest>,
) -> Result<TextStream<impl Stream<Item = String> + Sized>, &'static str> {
    // 创建处理管道
    let client = Client::new();
    let query_table = QueryTable {
        api_key: get_token(_key.claims.user_id).await.unwrap().unwrap(),
        messages: match data.code.clone() {
            None => vec![Message {
                content: data.question.clone(),
                ..Message::default()
            }],
            Some(code) => vec![
                Message {
                    content: String::from("这是一段rust代码 :".to_string() + &code),
                    ..Message::default()
                },
                Message {
                    content: data.question.clone(),
                    ..Message::default()
                },
            ],
        },
        stream: true,
        ..QueryTable::default()
    };

    let mut question = String::from("");
    for mess in query_table.messages.iter() {
        question = question + &mess.content;
    }
    // 发送请求（错误时返回 500）
    let response = match client
        .post(query_table.curl.as_str())
        .headers(GetHeader(&query_table))
        .json(&PostData::from(&query_table))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            return Err("");
        }
    };
    let mut ans = String::from("");
    Ok(TextStream! {
    let mut byte_stream = response.bytes_stream();
    while let Some(chunk) = byte_stream.next().await  {
            let chunk = chunk.unwrap();
            let string = std::str::from_utf8(&chunk).unwrap();
            let mut string1 = "".to_string();
            if !string.contains("data: [DONE]"){
                let parts: Vec<&str> = string.split("data: ").collect();
                parts.iter().for_each(|part|{string1 =  format!("{}\n",gjson::get(part, "choices.0.delta.content"));});
            }else{
                let _ = insert_ai_history(_key.claims.user_id, question.clone(), ans.clone()).await;
            }
            ans = ans + &string1;
            yield string1.to_string();
        }
    })
}

#[derive(Debug, Deserialize)]
struct AskHistory {
    page: u32,
    per_page: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Histoty {
    pub question: String,
    pub response: String,
    pub created_at: DateTimeWithTimeZone,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseHistory {
    histories: Vec<Histoty>,
}
#[get("/api/ai/history", format = "json", data = "<data>")]
pub async fn get_history(
    _key: JWTGuard,
    data: Json<AskHistory>,
) -> Result<Json<ResponseHistory>, String> {
    let histories = get_ai_history(_key.claims.user_id, data.page, data.per_page)
        .await
        .map_err(|e| format!("数据库错误: {}", e))?;

    Ok(Json(ResponseHistory {
        histories: histories
            .into_iter()
            .map(|h| Histoty {
                // 确保字段类型匹配
                question: h.question,
                response: h.response,
                created_at: h.created_at, // 如果需字符串格式，改为 h.created_at.to_rfc3339()
            })
            .collect(), // 添加 collect() 转换为 Vec<Histoty>
    }))
}

#[derive(Debug, Deserialize)]
struct Token {
    pub token: String,
}

#[put("/api/ai/token", format = "json", data = "<data>")]
pub async fn set_token_api(_key: JWTGuard, data: Json<Token>) -> Result<(), String> {
    set_token(_key.claims.user_id, data.token.clone())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
#[get("/protected")]
pub fn protected_route(jwt: JWTGuard) -> String {
    format!("Hello, user {}!", jwt.claims.user_id)
}

#[post("/login")]
pub fn login() -> String {
    // 假设用户验证成功，生成 Token
    let user_id = 1;
    generate_token(user_id)
}
