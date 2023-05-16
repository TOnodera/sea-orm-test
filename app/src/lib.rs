use entity::post;
use entity::post::Entity as Post;
use sea_orm::{entity::Set, DatabaseConnection, EntityTrait};

pub async fn handler(db: &DatabaseConnection) {
    // 取得
    let select: Option<post::Model> = Post::find_by_id(1).one(db).await.unwrap();
    match select {
        Some(row) => {
            assert_eq!(row.title, "title1");
            assert_eq!(row.text, "text1");
        }
        None => panic!("データの取得に失敗しました。"),
    };

    // 挿入
    let new_post = post::ActiveModel {
        title: Set(String::from("title2")),
        text: Set(String::from("text2")),
        ..Default::default()
    };
    post::Entity::insert(new_post).exec(db).await;
    let select: Option<post::Model> = Post::find_by_id(2).one(db).await.unwrap();
    match select {
        Some(row) => {
            assert_eq!(row.title, "title2");
            assert_eq!(row.text, "text2");
        }
        None => panic!("データの取得に失敗しました。"),
    };
}
