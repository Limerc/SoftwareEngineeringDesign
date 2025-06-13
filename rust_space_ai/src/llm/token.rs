use sea_orm::entity::prelude::*;
use sea_orm::{Database, DbErr, EntityTrait, Set, sea_query::OnConflict};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tokenlist")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub token: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn set_token(user_id: i32, token: String) -> Result<(), DbErr> {
    let db = Database::connect("mysql://root:123456@localhost:3306/program").await?;

    // 正确调用方式：通过Entity执行插入
    let result = Entity::insert(ActiveModel {
        user_id: Set(user_id),
        token: Set(token.clone()),
    })
    .on_conflict(
        OnConflict::column(Column::UserId)
            .update_column(Column::Token)
            .to_owned(),
    )
    .exec(&db)
    .await;
    match result {
        Ok(_) => Ok(()),
        Err(DbErr::RecordNotInserted) => Ok(()), // 忽略此错误
        Err(e) => Err(e),
    }
}

pub async fn get_token(user_id: i32) -> Result<Option<String>, DbErr> {
    let db = Database::connect("mysql://root:123456@localhost:3306/program").await?;

    // 通过主键查询记录
    let token_record = Entity::find_by_id(user_id).one(&db).await?;

    // 提取 token 字段
    Ok(token_record.map(|record| record.token))
}
