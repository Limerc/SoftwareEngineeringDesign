#[macro_use]
extern crate rocket;
mod db;
mod models;
mod routes;
mod utils;

use db::Db;
use rocket_db_pools::Database;
use routes::{auth, blog, judge, problems, video};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init()) // 挂载数据库连接池
        .mount("/auth", routes![auth::register, auth::login]) // 用户注册和登录
        .mount(
            "/problems",
            routes![problems::get_problems, problems::get_problem_details],
        ) // 题目管理
        .mount("/judge", routes![judge::submit_code]) // 判题功能
        .mount(
            "/blog",
            routes![
                blog::get_blog_list,
                blog::add_blog,
                blog::add_comment,
                blog::get_comments,
            ],
        ) // 博客相关接口
        .mount("/video", routes![video::search_videos]) // 视频相关接口
        .mount("/dist", rocket::fs::FileServer::from("dist")) // 提供静态文件服务
}
