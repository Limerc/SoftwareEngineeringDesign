#[macro_use] extern crate rocket;
mod models;
mod routes;
mod db;

use rocket_db_pools::Database;
use routes::{auth, problems, judge, blog, video};
use db::Db;

use rocket::response::Redirect;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/static/index.html"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init()) // 挂载数据库连接池
        .mount("/auth", routes![auth::register, auth::login]) // 用户注册和登录
        .mount("/problems", routes![problems::get_problems, problems::get_problem_details]) // 题目管理
        .mount("/judge", routes![judge::submit_code]) // 判题功能
        .mount("/blog", routes![
            blog::get_blog_list,
            blog::add_blog,
            blog::add_comment,
            blog::get_comments,
        ]) // 博客相关接口
        .mount("/video", routes![
            video::search_videos
        ]) // 视频相关接口
        .mount("/static", rocket::fs::FileServer::from("static")) // 提供静态文件服务
        .mount("/", routes![index]) // 添加根路径路由
}