use entity::post;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::Set;
use sea_orm_migration::sea_orm::ActiveModelTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Post::Table)
                    .add_column(
                        ColumnDef::new(Post::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra(String::from("DEFAULT CURRENT_TIMESTAMP")),
                    )
                    .add_column(
                        ColumnDef::new(Post::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra(String::from("DEFAULT CURRENT_TIMESTAMP")),
                    )
                    .to_owned(),
            )
            .await?;

        // シーディング
        let db = manager.get_connection();
        post::ActiveModel {
            title: Set(String::from("title1")),
            text: Set(String::from("text1")),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Post::Table)
                    .drop_column(Post::CreatedAt)
                    .drop_column(Post::UpdatedAt)
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
    CreatedAt,
    UpdatedAt,
}
