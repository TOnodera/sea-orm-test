#[cfg(test)]
mod test {
    use entity::post;
    use entity::post::Entity as Post;
    use migration::MigratorTrait;
    use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, EntityTrait, Schema};

    async fn setup(db: &DatabaseConnection) {
        let schema = Schema::new(DbBackend::Sqlite);
        let stmt = schema.create_table_from_entity(entity::post::Entity);
        db.execute(db.get_database_backend().build(&stmt)).await;
    }

    fn down() {}

    #[tokio::test]
    async fn database() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        setup(&db);
        migration::Migrator::up(&db, None).await;
        app::handler(&db);
        migration::Migrator::reset(&db).await;
    }
}
