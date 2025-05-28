use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("mysql_database")]
pub struct Db(sqlx::MySqlPool);