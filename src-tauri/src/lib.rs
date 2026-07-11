use std::sync::LazyLock;
use surrealdb::engine::local::{RocksDb, Db};
use surrealdb::{Surreal, types::{RecordId, SurrealValue}};
use tokio::sync::OnceCell;

pub static SURREAL: LazyLock<Surreal<Db>> = LazyLock::new(Surreal::init);

use serde::{Serialize, Deserialize};

#[derive(Debug, SurrealValue, Serialize, Deserialize)]
struct Profile {
    token: String,
    email: String,
    username: String,
}

#[derive(Debug, SurrealValue, Serialize, Deserialize)]
struct ProfileRecord {
    id: RecordId,
    token: String,
    email: String,
    username: String,
}

#[tauri::command]
async fn profile() -> Result<Vec<ProfileRecord>, String> {
    Ok(
        SURREAL.select("profiles").await.map_err(|e| e.to_string())?
    )
}

#[tauri::command]
async fn add_profile(token: &str) -> Result<ProfileRecord, String> {
    let created: Option<ProfileRecord> = SURREAL
        .create("profiles")
        .content(Profile { token: token.to_string(), email: "".to_string(), username: "Tyson".to_string() })
        .await
        .map_err(|e| e.to_string())?;

    Ok(created.unwrap())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            tauri::async_runtime::spawn(async {
                SURREAL.connect::<RocksDb>("../../croissant.sansel").await?;
                SURREAL.use_ns("test").use_db("test").await?;
                Ok::<_, surrealdb::Error>(())
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_profile,
            profile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}