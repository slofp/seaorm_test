use chrono::Utc;
use entity::test_data;
use sea_orm::{Database, EntityTrait, IntoActiveModel};
use sea_orm::ActiveModelTrait;
use sea_orm::entity::ModelTrait;


fn get_db_url() -> String {
    dotenv::var("DATABASE_URL").expect("DATABASE_URL must be write .env")
}

fn init() {
    dotenv::from_filename(".env").ok();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello CodeSandbox!");

    init();

    let db = Database::connect(get_db_url()).await?;

    let modelData = test_data::Model {
        id: 0,
        username: "testUser".to_owned(),
        memo: Some("kuso".to_owned()),
        created_at: Utc::now().into(),
    };

    println!("modelData: {:?}", modelData);

    let data = modelData.into_active_model();
    let res = data.insert(&db).await?;

    println!("insertedData: {:?}", res);

    let selectedRes = test_data::Entity::find_by_id(0).one(&db).await?;

    if let Some(selectedData) = selectedRes {
        println!("selectedData: {:?}", selectedData);
        let res = selectedData.delete(&db).await?;
        assert_eq!(res.rows_affected, 1);
    }

    db.close().await?;

    Ok(())
}
