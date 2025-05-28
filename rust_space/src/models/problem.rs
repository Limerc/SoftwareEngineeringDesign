use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Problem {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub examples: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}