use proj::sql::conn;
use proj::llm::askQuestion::*;
use rocket::{routes, launch};
use sea_orm::Iden;
use tokio::main;
use proj::llm::askQuestion::processed_stream;
use proj::server::{events};
use proj::llm::token;
// #[main]
// async fn main() {
//     // let mut con = conn::DBConnection{
//     //     ip:"139.59.103.151".to_string(),
//     //     user:"cooperator".to_string(),
//     //     password:"GH&h%7678ttyukm".to_string(),
//     //     db_name:"learn_rocket".to_string(),
//     //     ..Default::default()
//     // };
//     // println!("{:#?}", con);
//     // con.ge_pool();
//     // let v = vec![Message {
//     //     content:"1+1等于几".to_string(),
//     //     ..Default::default()
//     // }];
//     // LLMAskStream(v, Some(QueryTable{
//     //     api_key:"sk-ce50c3670bc3466baf91d39e43a98ff5".to_string(),
//     //     stream:true,
//     //     include_usage:false,
//     //     ..Default::default()
//     // }));
//     AIHistory::insert_ai_history(1, "测试".to_string(), "测试".to_string());
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![processed_stream])
// }
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // token::set_token(2, "1233829916".to_string()).await?;
//     println!("{}", token::get_token(2).await.unwrap().unwrap());
//     Ok(())
// }


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![protected_route, login, processed_stream, get_history])
}