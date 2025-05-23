use sea_orm::entity::prelude::*;
use chrono::Utc;
use sea_orm::{Database, QueryOrder, QuerySelect, Set};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ai_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub user_id:i32,
    pub question: String,
    pub response:String,
    pub created_at: DateTimeWithTimeZone,
}

// 2. 手动实现 Entity 关联（替代 DeriveEntity）
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {} // 启用 ActiveModel 默认行为
pub async fn insert_ai_history(user_id: i32, question: String, response: String)-> Result<(), DbErr>{
    let db = Database::connect("mysql://root:123456@localhost:3306/program").await?;
    let new_record = ActiveModel {
        user_id: Set(user_id),
        question: Set(question),
        response: Set(response),
        ..Default::default() // 其他字段使用默认值（如自增 id）
    };

    // 执行插入操作
    new_record.insert(&db).await.expect("TODO: panic message");
    println!("inserted record successfully");
    Ok(())
}

pub async fn get_ai_history(user_id: i32, page:u32, per_page:u32)->Result<Vec<Model>, DbErr> {
    let offset = ((page - 1) * per_page) as u64;
    let db = Database::connect("mysql://root:123456@localhost:3306/program").await?;

    Entity::find()
        .filter(Column::UserId.eq(user_id))  // 过滤指定用户
        .order_by_desc(Column::CreatedAt)   // 按创建时间倒序（最新在前）
        .offset(offset)                      // 分页偏移
        .limit(per_page as u64)              // 每页数量
        .all(&db)                             // 执行查询
        .await
}

