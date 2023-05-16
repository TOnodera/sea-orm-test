use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() {
    let database_url = "postgres://postgres:password@db/postgres";
    let connection = sea_orm::Database::connect(database_url).await.unwrap();
    Migrator::up(&connection, None);
}
