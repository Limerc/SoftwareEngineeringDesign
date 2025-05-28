use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket_db_pools::Connection;
use crate::db::Db;
use std::process::Command;

#[derive(Deserialize)]
pub struct Submission {
    pub problem_id: i32,
    pub code: String,
}

#[derive(Serialize)]
pub struct JudgeResponse {
    success: bool,
    message: String,
    stdout: Option<String>,
    stderr: Option<String>,
}

#[post("/", format = "json", data = "<submission>")]
pub async fn submit_code(submission: Json<Submission>, mut db: Connection<Db>) -> Json<JudgeResponse> {
    // 获取测试用例
    let test_cases = match sqlx::query!(
        "SELECT input, expected FROM test_cases WHERE problem_id = ?",
        submission.problem_id
    )
    .fetch_all(&mut **db)
    .await
    {
        Ok(cases) => cases,
        Err(_) => {
            return Json(JudgeResponse {
                success: false,
                message: "Failed to fetch test cases".to_string(),
                stdout: None,
                stderr: None,
            });
        }
    };

    // 将用户代码写入临时文件
    let code_path = "/tmp/user_code.rs";
    if let Err(err) = std::fs::write(code_path, &submission.code) {
        return Json(JudgeResponse {
            success: false,
            message: "Failed to write code to file".to_string(),
            stdout: None,
            stderr: Some(err.to_string()),
        });
    }
//判题这个模块代码最麻烦了，输入输出问题排查了好久才解决
    // 使用 Docker 编译并运行用户代码
    for test_case in test_cases {
        let output = Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("-v")
            .arg(format!("{}:/user_code.rs", code_path))
            .arg("rust:latest")
            .arg("sh")
            .arg("-c")
            .arg(format!(
                "rustc /user_code.rs -o /user_code && echo '{}' | ./user_code",
                test_case.input
            ))
            .output();
    
        match output {
            Ok(output) => {
                if !output.status.success() {
                    return Json(JudgeResponse {
                        success: false,
                        message: "Runtime error in user code".to_string(),
                        stdout: Some(String::from_utf8_lossy(&output.stdout).trim().to_string()),
                        stderr: Some(String::from_utf8_lossy(&output.stderr).trim().to_string()),
                    });
                }
    
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if stdout != test_case.expected.trim() {
                    return Json(JudgeResponse {
                        success: false,
                        message: format!(
                            "Test case failed on input: {}. Expected: {}, Got: {}",
                            test_case.input,
                            test_case.expected.trim(),
                            stdout
                        ),
                        stdout: Some(stdout),
                        stderr: Some(String::from_utf8_lossy(&output.stderr).trim().to_string()),
                    });
                }
            }
            Err(err) => {
                return Json(JudgeResponse {
                    success: false,
                    message: format!(
                        "Failed to execute code in Docker for input: {}. Error: {}",
                        test_case.input,
                        err.to_string()
                    ),
                    stdout: None,
                    stderr: None,
                });
            }
        }
    }
    
    // 所有测试用例通过
    Json(JudgeResponse {
        success: true,
        message: "All test cases passed".to_string(),
        stdout: None,
        stderr: None,
    })
}