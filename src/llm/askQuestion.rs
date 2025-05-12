use rocket::response::Debug;
use reqwest::{Client, Body, Response};
use bytes::Bytes;
use futures::{Stream, StreamExt, TryStreamExt};
use futures::future::ok;
use rocket::{post, response::stream::{TextStream}, serde::json::Json, State};

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize};
use super::query::{GetHeader, Message, PostData, QueryTable};

use std::time::Duration;
use tracing::error;
use crate::server::events;

#[derive(Debug, Deserialize)]
pub struct CodeRequest {
    code: Option<String>,
    question: String,
}

#[derive(Debug)]
pub struct ApiKey(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = req.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Error((Status::Unauthorized, ())),
            1 => {
                let token = keys[0];
                if token.starts_with("Bearer ") {
                    Outcome::Success(ApiKey(token[7..].trim().to_string()))
                } else {
                    Outcome::Error((Status::BadRequest, ()))
                }
            }
            _ => Outcome::Error((Status::BadRequest, ())),
        }
    }
}

impl ApiKey {
    // 消费结构体，返回内部的 String
    pub fn into_inner(self) -> String {
        self.0
    }
}



#[post("/api/ai/ask", format = "json", data = "<data>")]
pub async fn processed_stream(
    _key: ApiKey,
    data: Json<CodeRequest>
) -> Result<TextStream<impl Stream<Item=String> + Sized>, &'static str> {
    // 创建处理管道
        let client = Client::new();
    let query_table = QueryTable {
        api_key: _key.into_inner(),
        messages: match data.code.clone() {
            None => vec![Message{
                content: data.question.clone(),
                ..Message::default()
            }],
            Some(code) => vec![Message{
                content: String::from("这是一段rust代码 :".to_string() + &code),
                ..Message::default()
            },
            Message{
                content: data.question.clone(),
                ..Message::default()
            }]
        },
        stream: true,
        ..QueryTable::default()
    };

    // 发送请求（错误时返回 500）
    let response = match client.post(query_table.curl.as_str())
        .headers(GetHeader(&query_table))
        .json(&PostData::from(&query_table))
        .send()
        .await {
        Ok(resp) => resp,
        Err(e) => {
            return Err("");
        }
    };
    Ok(TextStream! {
    let mut byte_stream = response.bytes_stream();
            while let Some(chunk) = byte_stream.next().await  {
            let chunk = chunk.unwrap();
            let string = std::str::from_utf8(&chunk).unwrap();
            let mut string1 = "".to_string();
            if !string.contains("data: [DONE]"){
                let mut parts: Vec<&str> = string.split("data: ").collect();
                parts.iter().for_each(|part|{string1 =  format!("{}\n",gjson::get(part, "choices.0.delta.content"));});
            }
                yield string1.to_string();
        }
    })
}