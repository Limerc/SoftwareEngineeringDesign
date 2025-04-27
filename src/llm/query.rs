use reqwest::header;
use reqwest::header::HeaderMap;
use serde::{Serialize, Deserialize};
use serde_json::{json, from_value, from_str};

#[derive(Debug)]
pub enum RoleType{
    User,
    System,
    Assistant
}

#[derive(Debug)]
pub enum ResponseType{
    Text,
    JsonObject
}

#[derive(Debug)]
pub struct Message {
    pub role: RoleType,
    pub content: String,
}

impl Default for Message{
    fn default() -> Self {
        Self{
            role:RoleType::User,
            content: String::from(""),
        }
    }
}
impl Message{
    pub fn from(post_message: PostMessage)-> Self{
        Self{
            role: match post_message.role.as_str() {
                "user" => RoleType::User,
                "system" => RoleType::System,
                "assistant" => RoleType::Assistant,
                &_ => panic!("Unknown role")
            },
            content:post_message.content,
        }
    }
}
#[derive(Debug)]
pub struct QueryTable {
    pub curl: String,
    pub api_key: String,
    pub model: String,
    pub messages: Vec<Message>,
    pub stream:bool,
    pub response_type:ResponseType,
}

impl Default for QueryTable{
    fn default() -> Self {
        Self{
            curl:String::from("https://api.deepseek.com/chat/completions"),
            api_key:String::from(""),
            model:String::from("deepseek-chat"),
            messages:vec![],
            stream:false,
            response_type:ResponseType::Text,
        }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PostMessage{
    role:String,
    content:String,
}

impl PostMessage {
    pub fn from(message: &Message) -> Self {
        Self{
            role: match message.role{
                RoleType::User => "user".to_string(),
                RoleType::System => "system".to_string(),
                RoleType::Assistant => "assistant".to_string(),
            },
            content:(*message).content.clone()
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct PostResponseFormat{
    r#type : String
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct PostData{
    model: String,
    messages: Vec<PostMessage>,
    response_format:PostResponseFormat,
    stream:bool,
}

impl PostData{
    pub fn from(query_table: &QueryTable) -> Self{
        let mut v: Vec<PostMessage> = vec![];
        for message in &query_table.messages{
            v.push(PostMessage::from(&message));
        }
        Self{
            model:query_table.model.clone(),
            messages: v,
            response_format:match query_table.response_type {
                ResponseType::Text => PostResponseFormat{r#type:"text".to_string()},
                ResponseType::JsonObject => PostResponseFormat{r#type:"json_object".to_string()},
            },
            stream:query_table.stream
        }
    }
}

fn GetHeader(query_table: &QueryTable) ->HeaderMap{
    // 获取请求头
    let mut headers = header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_str("application/json").unwrap());
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(format!("Bearer {}", query_table.api_key).as_str()).unwrap());
    headers
}

// 非流式调用LLM
#[tokio::main]
pub async fn LLMAsk(messages: Vec<Message>, query_table: Option<QueryTable>) -> PostMessage{
    // 调用LLM得到回复，非流式调用
    let mut query_table = query_table.unwrap_or_default();
    query_table.messages = messages;
    let client = reqwest::Client::new();
    let body = client.post(query_table.curl.as_str())
        .headers(GetHeader(&query_table))
        .json(&PostData::from(&query_table))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let binding = gjson::get(body.as_str(), "choices.0.message");
    let messages = binding.str();
    let messages:PostMessage = from_str(messages).unwrap();
    messages
}