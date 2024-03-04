use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, StandardTokenResponse};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use surrealdb::engine::local::{Db, SpeeDb};
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}
pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

pub async fn initialize_database(app_handle: &AppHandle) -> surrealdb::Result<()> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let speedb_path = app_dir.join("surreal_speedb");
    println!("Database path: {:?}", speedb_path);
    DB.connect::<SpeeDb>(speedb_path).await?;
    DB.use_ns("feams").use_db("feams").await?;
    // Create database connection
    Ok(())
}

pub async fn save_token(
    token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    name: String,
) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
    Ok(DB.create("user").content(User { token, name }).await?)
}

// pub async fn delete_token(
//     db: Surreal<Db>,
//     name: &str,
// ) -> Result<Option<User>, Box<dyn std::error::Error>> {
//     let resp: Option<User> = db.delete(("name", name)).await?;

//     Ok(resp)
// }
