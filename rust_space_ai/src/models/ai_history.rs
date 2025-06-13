use sea_orm::entity::prelude::*;
use sea_orm::{Database, QueryOrder, QuerySelect, Set};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ai_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub user_id: i32,
    pub question: String,
    pub response: String,
    pub created_at: DateTimeWithTimeZone,
}

// 2. 手动实现 Entity 关联（替代 DeriveEntity）
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {} // 启用 ActiveModel 默认行为
pub async fn insert_ai_history(
    user_id: i32,
    question: String,
    response: String,
) -> Result<(), DbErr> {
    println!("inserting");
    match Database::connect("mysql://cooperator:test12345@8.134.162.177/learn_rocket").await {
        Ok(db) => {
            println!("数据库连接成功！");
            // 后续插入逻辑...
            println!("inserting-1");
            let new_record = ActiveModel {
                user_id: Set(user_id),
                question: Set(question),
                response: Set(response),
                ..Default::default() // 其他字段使用默认值（如自增 id）
            };
            println!("inserting-2");

            // 修改后（传播错误并打印详细信息）
            if let Err(e) = new_record.insert(&db).await {
                eprintln!("插入失败: {:?}", e); // 打印错误详细信息
                return Err(e);
            }
            println!("inserting-3");
            println!("inserted record successfully");
            Ok(())
        }
        Err(e) => {
            // 打印错误详细信息
            eprintln!("数据库连接失败: {:?}", e);
            // 如果是连接错误（如网络问题、认证失败），打印更具体的错误类型
            // if let DbErr::Conn(e) = e {
            //     eprintln!("连接错误详情: {}", e);
            // }
            Err(e)
        }
    }
}

pub async fn get_ai_history(user_id: i32, page: u32, per_page: u32) -> Result<Vec<Model>, DbErr> {
    let offset = ((page - 1) * per_page) as u64;
    let db = Database::connect("mysql://cooperator:test12345@8.134.162.177/learn_rocket").await?;

    Entity::find()
        .filter(Column::UserId.eq(user_id)) // 过滤指定用户
        .order_by_desc(Column::CreatedAt) // 按创建时间倒序（最新在前）
        .offset(offset) // 分页偏移
        .limit(per_page as u64) // 每页数量
        .all(&db) // 执行查询
        .await
}
