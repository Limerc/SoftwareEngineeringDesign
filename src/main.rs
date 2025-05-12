use proj::sql::conn;
use proj::llm::query::{LLMAsk, LLMAskStream, Message, QueryTable};
use rocket::{routes, launch};
use proj::llm::askQuestion::processed_stream;
use proj::server::{events};
// fn main() {
//     // let mut con = conn::DBConnection{
//     //     ip:"139.59.103.151".to_string(),
//     //     user:"cooperator".to_string(),
//     //     password:"GH&h%7678ttyukm".to_string(),
//     //     db_name:"learn_rocket".to_string(),
//     //     ..Default::default()
//     // };
//     // println!("{:#?}", con);
//     // con.ge_pool();
//     let v = vec![Message {
//         content:"1+1等于几".to_string(),
//         ..Default::default()
//     }];
//     LLMAskStream(v, Some(QueryTable{
//         api_key:"sk-ce50c3670bc3466baf91d39e43a98ff5".to_string(),
//         stream:true,
//         include_usage:false,
//         ..Default::default()
//     }));
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![processed_stream])
}
