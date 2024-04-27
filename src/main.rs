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
    println!("Hello!");

    init();

    let db = Database::connect(get_db_url()).await?;

    let model_data = test_data::Model {
        id: 0,
        username: "testUser".to_owned(),
        memo: Some("kuso".to_owned()),
        created_at: Utc::now().into(),
    };

    println!("model_data: {:?}", model_data);

    let data = model_data.into_active_model();
    let res = data.insert(&db).await?;

    println!("insertedData: {:?}", res);

    let selected_res = test_data::Entity::find_by_id(0).one(&db).await?;

    if let Some(selected_data) = selected_res {
        println!("selectedData: {:?}", selected_data);
        let res = selected_data.delete(&db).await?;
        assert_eq!(res.rows_affected, 1);
    }

    db.close().await?;

    Ok(())
}
