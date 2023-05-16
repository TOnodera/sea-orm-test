use entity::post;
use entity::post::Entity as Post;
use migration::{Migrator, MigratorTrait};
use sea_orm::entity::Set;
use sea_orm::EntityTrait;

#[tokio::main]
async fn main() {
    let database_url = "postgres://postgres:password@db/postgres";
    let db = sea_orm::Database::connect(database_url).await.unwrap();

    app::handler(&db).await;
}
