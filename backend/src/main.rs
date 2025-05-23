#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};

// 视频学习接口
#[derive(Serialize)]
struct Video {
    title: String,
    url: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    code: u8,
    message: String,
    data: T,
}

// 视频搜索接口
#[get("/api/videos?<keyword>")]
fn search_videos(keyword: Option<&str>) -> Json<ApiResponse<Vec<Video>>> {
    let results = match keyword {
        Some("动态规划") => vec![
            Video {
                title: "动态规划".to_string(),
                url: "https://billbill/dp_learning".to_string(),
            },
            Video {
                title: "递归".to_string(),
                url: "https://billbill/digui_learning".to_string(),
            },
        ],
        _ => vec![],
    };
    Json(ApiResponse {
        code: 0,
        message: "操作成功".to_string(),
        data: results,
    })
}
// 博客相关接口
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct BlogPost {
    id: u32,
    title: String,
    author: String,
    content: String,
    created_at: String,
    comment_count: u32,
    related_problem: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct BlogListResponse {
    code: u8,
    message: Option<String>,
    posts: Vec<BlogPost>,
}

#[get("/api/problems/<message>/posts?<page>&<per_page>")]
fn get_blog_list(message: &str, page: u32, per_page: u32) -> Json<BlogListResponse> {
    let posts = vec![
        BlogPost {
            id: 101,
            title: "所有权问题详解".to_string(),
            author: "user123".to_string(),
            content: "题目的解析是......".to_string(),
            created_at: "2024-03-20T10:00:00Z".to_string(),
            comment_count: 5,
            related_problem: Some(message.to_string()),
        }
    ];
    Json(BlogListResponse {
        code: 0,
        message: Some("操作成功".to_string()),
        posts,
    })
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewBlogPost {
    title: String,
    content: String,
    related_problem: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct AddBlogResponse {
    code: u8,
    message: Option<String>,
    post_id: u32,
    created_at: String,
}

#[post("/api/posts", format = "json", data = "<post>")]
fn add_blog(post: Json<NewBlogPost>) -> Json<AddBlogResponse> {
    Json(AddBlogResponse {
        code: 0,
        message: Some("操作成功".to_string()),
        post_id: 101,
        created_at: "2024-03-20T10:00:00Z".to_string(),
    })
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewComment {
    content: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct AddCommentResponse {
    code: u8,
    message: Option<String>,
    comment_id: String,
    created_at: String,
}

#[post("/api/posts/<id>/comments", format = "json", data = "<comment>")]
fn add_comment(id: u32, comment: Json<NewComment>) -> Json<AddCommentResponse> {
    Json(AddCommentResponse {
        code: 0,
        message: Some("操作成功".to_string()),
        comment_id: format!("{}-1", id),
        created_at: "2024-03-20T10:05:00Z".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![search_videos, get_blog_list, add_blog, add_comment],
    )
}