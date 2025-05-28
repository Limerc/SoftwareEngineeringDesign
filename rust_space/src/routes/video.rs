use rocket::serde::json::Json;
use crate::models::video::Video;
use rocket_db_pools::Connection;
use crate::db::Db;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: u8,
    pub message: Option<String>,
    pub data: T,
}

#[get("/search_videos?<keyword>")]
pub async fn search_videos(
    mut db: Connection<Db>,
    keyword: Option<&str>
) -> Json<ApiResponse<Vec<Video>>> {
    let pattern = keyword.map(|k| format!("%{}%", k)).unwrap_or("%".to_string());

    let videos = sqlx::query_as!(
        Video,
        r#"SELECT id, title, url FROM video WHERE title LIKE ?"#,
        pattern
    )
    .fetch_all(&mut **db)
    .await
    .unwrap_or_else(|_| vec![]);

    Json(ApiResponse {
        code: 0,
        message: Some("操作成功".to_string()),
        data: videos,
    })
}