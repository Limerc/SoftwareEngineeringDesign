use crate::llm::query::{Message, PostMessage, ResponseType};
use crate::models::ai_history::insert_ai_history;
use crate::models::tokenlist::get_token;
use crate::routes::jwt::JWTGuard;
use futures::{Stream, StreamExt};
use reqwest::header::HeaderMap;
use reqwest::{Client, header};
use rocket::post;
use rocket::response::stream::TextStream;
use rocket::serde::{Deserialize, Serialize, json::Json};
#[derive(Debug, Deserialize)]
pub struct CodeRequest {
    code: Option<String>,
    question: String,
} // 请求数据

#[derive(Debug)]
pub struct QueryTable {
    pub curl: String,
    pub api_key: String,
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    pub include_usage: bool,
    pub response_type: ResponseType,
}

impl Default for QueryTable {
    fn default() -> Self {
        Self {
            curl: String::from("https://api.deepseek.com/chat/completions"),
            api_key: String::from(""),
            model: String::from("deepseek-chat"),
            messages: vec![],
            stream: false,
            include_usage: true,
            response_type: ResponseType::Text,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PostResponseFormat {
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostData {
    model: String,
    messages: Vec<PostMessage>,
    response_format: PostResponseFormat,
    stream: bool,
    include_usage: bool,
}

impl PostData {
    pub fn from(query_table: &QueryTable) -> Self {
        let mut v: Vec<PostMessage> = vec![];
        for message in &query_table.messages {
            v.push(PostMessage::from(&message));
        }
        Self {
            model: query_table.model.clone(),
            messages: v,
            response_format: match query_table.response_type {
                ResponseType::Text => PostResponseFormat {
                    r#type: "text".to_string(),
                },
                ResponseType::JsonObject => PostResponseFormat {
                    r#type: "json_object".to_string(),
                },
            },
            stream: query_table.stream,
            include_usage: query_table.include_usage,
        }
    }
}
pub fn GetHeader(query_table: &QueryTable) -> HeaderMap {
    // 获取请求头
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/json").unwrap(),
    );
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(format!("Bearer {}", query_table.api_key).as_str()).unwrap(),
    );
    headers
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
