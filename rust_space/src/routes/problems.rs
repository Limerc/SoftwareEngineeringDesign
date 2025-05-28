use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::db::Db;
use rocket::http::Status;

#[derive(serde::Serialize)]
pub struct Problem {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub examples: String,
}

#[get("/")]
pub async fn get_problems(mut db: Connection<Db>) -> Json<Vec<Problem>> {
    let problems = sqlx::query_as!(
        Problem,
        "SELECT id, title, description, examples FROM problems"
    )
    .fetch_all(&mut **db)
    .await
    .expect("Failed to fetch problems");

    Json(problems)
}



#[get("/<id>")]
pub async fn get_problem_details(id: i32, mut db: Connection<Db>) -> Result<Json<Problem>, Status> {
    match sqlx::query_as!(
        Problem,
        "SELECT id, title, description, examples FROM problems WHERE id = ?",
        id
    )
    .fetch_one(&mut **db)
    .await
    {
        Ok(problem) => Ok(Json(problem)),
        Err(_) => Err(Status::NotFound),
    }
}
