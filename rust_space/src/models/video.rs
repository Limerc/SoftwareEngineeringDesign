use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Video {
    pub id: i32, 
    pub title: String,
    pub url: String,
}