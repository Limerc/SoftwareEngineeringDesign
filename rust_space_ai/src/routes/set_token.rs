use crate::models::tokenlist::set_token;
use crate::routes::jwt::JWTGuard;
use rocket::put;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
#[derive(Debug, Deserialize)]
pub struct Token {
    pub token: String,
}
#[put("/api/ai/token", format = "json", data = "<data>")]
pub async fn set_token_api(_key: JWTGuard, data: Json<Token>) -> Result<(), String> {
    set_token(_key.claims.id, data.token.clone())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
