use std::sync::LazyLock;
use surrealdb::engine::local::{RocksDb, Db};
use surrealdb::{Surreal, types::{RecordId, SurrealValue}};
use tokio::sync::OnceCell;

pub static SURREAL: LazyLock<Surreal<Db>> = LazyLock::new(Surreal::init);

use serde::{Serialize, Deserialize};

#[derive(Debug, SurrealValue, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug, SurrealValue, Serialize, Deserialize)]
struct PersonRecord {
    id: RecordId,
    name: String,
    age: u8,
}

#[tauri::command]
async fn profile() -> Result<Vec<PersonRecord>, String> {
    Ok(
        SURREAL.select("person").await.map_err(|e| e.to_string())?
    )
}

#[tauri::command]
async fn saved_profiles() -> Result<PersonRecord, String> {
    let created: Option<PersonRecord> = SURREAL
        .create("person")
        .content(Person { name: "Tyson".to_string(), age: 32 })
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
            saved_profiles,
            profile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}