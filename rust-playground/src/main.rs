#[macro_use] extern crate rocket;

use rocket::serde::{Deserialize, Serialize, json::Json};
use std::process::Command;

#[derive(Deserialize)]
struct CodeInput {
    code: String,
}

#[derive(Serialize)]
struct CodeOutput {
    stdout: String,
    stderr: String,
}

#[post("/run", format = "json", data = "<input>")]
fn run_code(input: Json<CodeInput>) -> Json<CodeOutput> {
    // 将用户代码写入临时文件
    std::fs::write("/tmp/user_code.rs", &input.code).expect("Failed to write code to file");

    // 使用 Docker 运行用户代码
    let output = Command::new("docker")
        .arg("run")
        .arg("--rm") // 容器运行结束后自动删除
        .arg("-v") // 挂载用户代码到容器
        .arg("/tmp/user_code.rs:/user_code.rs")
        .arg("rust:latest") // 使用官方 Rust 镜像
        .arg("sh")
        .arg("-c")
        .arg("rustc /user_code.rs && ./user_code")
        .output()
        .expect("Failed to execute code in Docker");

    Json(CodeOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![run_code])
        .mount("/", rocket::fs::FileServer::from("static"))
}