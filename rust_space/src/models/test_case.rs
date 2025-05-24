use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TestCase {
    pub id: i32,
    pub problem_id: i32,
    pub input: String,
    pub expected: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}