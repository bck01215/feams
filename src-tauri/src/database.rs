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
    pub token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    pub name: String,
    pub login_date: u64,
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

pub async fn get_last_user() -> Result<Option<User>, Box<dyn std::error::Error>> {
    let mut result = DB
        .query("SELECT * FROM user ORDER BY login_date DESC LIMIT 1")
        .await?;
    Ok(result.take(0)?)
}

pub async fn save_token(
    token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    name: &str,
) -> Result<Option<Record>, Box<dyn std::error::Error>> {
    Ok(DB
        .update(("user", name))
        .content(User {
            token,
            name: name.to_string(),
            login_date: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
        .await?)
}

// pub async fn delete_token(
//     db: Surreal<Db>,
//     name: &str,
// ) -> Result<Option<User>, Box<dyn std::error::Error>> {
//     let resp: Option<User> = db.delete(("name", name)).await?;

//     Ok(resp)
// }
