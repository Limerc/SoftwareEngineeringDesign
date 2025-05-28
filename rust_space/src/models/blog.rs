use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BlogPost {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub related_problem: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub comment_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewBlogPost {
    pub title: String,
    pub content: String,
    pub related_problem: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewComment {
    pub content: String,
}



