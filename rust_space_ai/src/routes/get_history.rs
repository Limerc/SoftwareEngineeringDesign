use crate::models::ai_history::get_ai_history;
use crate::routes::jwt::JWTGuard;
use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::prelude::DateTimeWithTimeZone;
#[derive(Debug, Deserialize)]
pub struct AskHistory {
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
#[get("/api/ai/history?<page>&<per_page>")]
pub async fn get_history(
    _key: JWTGuard,
    page: u32,
    per_page: u32,
) -> Result<Json<ResponseHistory>, String> {
    let histories = get_ai_history(_key.claims.id, page, per_page)
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
