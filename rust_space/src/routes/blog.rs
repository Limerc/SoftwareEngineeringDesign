use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::db::Db;
use crate::models::blog::{BlogPost, NewBlogPost, NewComment};
use serde::Serialize;

#[derive(Serialize)]
pub struct BlogListResponse {
    pub code: u8,
    pub message: Option<String>,
    pub posts: Vec<BlogPost>,
}

#[derive(Serialize)]
pub struct AddBlogResponse {
    pub code: u8,
    pub message: Option<String>,
    pub post_id: i64,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize)]
pub struct AddCommentResponse {
    pub code: u8,
    pub message: Option<String>,
    pub comment_id: i64,
}

// 获取博客列表
#[get("/get_blog_list?<message>&<page>&<per_page>")]
pub async fn get_blog_list(
    mut db: Connection<Db>,
    message: Option<&str>,
    page: Option<u32>,
    per_page: Option<u32>
) -> Json<BlogListResponse> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    let pattern = format!("%{}%", message.unwrap_or(""));

    let posts = sqlx::query_as!(
        BlogPost,
        r#"
        SELECT id, title, content, related_problem, created_at,
            (SELECT COUNT(*) FROM comment WHERE blog_id = blog.id) as comment_count
        FROM blog
        WHERE title LIKE ? OR content LIKE ?
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
        pattern, pattern, per_page as i64, offset as i64
    )
    .fetch_all(&mut **db)
    .await
    .unwrap_or_else(|_| vec![]);

    Json(BlogListResponse {
        code: 0,
        message: Some("获取成功".to_string()),
        posts,
    })
}

// 发布博客
#[post("/add_blog", format = "json", data = "<blog>")]
pub async fn add_blog(
    mut db: Connection<Db>,
    blog: Json<NewBlogPost>
) -> Json<AddBlogResponse> {
    let res = sqlx::query!(
        r#"
        INSERT INTO blog (title, content, related_problem)
        VALUES (?, ?, ?)
        "#,
        blog.title,
        blog.content,
        blog.related_problem
    )
    .execute(&mut **db)
    .await;

    match res {
        Ok(result) => {
            let post_id = result.last_insert_id() as i64;
            Json(AddBlogResponse {
                code: 0,
                message: Some("发布成功".to_string()),
                post_id,
                created_at: Some(chrono::Utc::now().naive_utc()),
            })
        }
        Err(_) => Json(AddBlogResponse {
            code: 1,
            message: Some("数据库错误".to_string()),
            post_id: 0,
            created_at: None,
        }),
    }
}

// 添加评论
#[post("/add_comment/<id>", format = "json", data = "<comment>")]
pub async fn add_comment(
    mut db: Connection<Db>,
    id: i64,
    comment: Json<NewComment>
) -> Json<AddCommentResponse> {
    let res = sqlx::query!(
        r#"
        INSERT INTO comment (blog_id, content)
        VALUES (?, ?)
        "#,
        id,
        comment.content
    )
    .execute(&mut **db)
    .await;

    match res {
        Ok(result) => {
            let comment_id = result.last_insert_id() as i64;
            Json(AddCommentResponse {
                code: 0,
                message: Some("评论成功".to_string()),
                comment_id,
            })
        }
        Err(_) => Json(AddCommentResponse {
            code: 1,
            message: Some("数据库错误".to_string()),
            comment_id: 0,
        }),
    }
}

#[derive(Serialize)]
pub struct Comment {
    pub id: i64,
    pub blog_id: i64,
    pub content: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize)]
pub struct CommentListResponse {
    pub comments: Vec<Comment>,
}

// 获取某博客的评论列表
#[get("/get_comments/<blog_id>")]
pub async fn get_comments(
    mut db: Connection<Db>,
    blog_id: i64
) -> Json<CommentListResponse> {
    let comments = sqlx::query_as!(
        Comment,
        r#"
        SELECT id, blog_id, content, created_at
        FROM comment
        WHERE blog_id = ?
        ORDER BY created_at ASC
        "#,
        blog_id
    )
    .fetch_all(&mut **db)
    .await
    .unwrap_or_else(|_| vec![]);

    Json(CommentListResponse { comments })
}

