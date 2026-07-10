use std::sync::LazyLock;
use surrealdb::engine::local::{RocksDb, Db};
use surrealdb::{Surreal, types::{RecordId, SurrealValue}};
use tokio::sync::OnceCell;

pub static SURREAL: LazyLock<Surreal<Db>> = LazyLock::new(Surreal::init);
static DB_READY: OnceCell<()> = OnceCell::const_new();

pub async fn ensure_connected() -> surrealdb::Result<()> {
    DB_READY
        .get_or_try_init(|| async {
            SURREAL.connect::<RocksDb>("croissant.sansel").await?;
            SURREAL.use_ns("test").use_db("test").await?;
            Ok::<_, surrealdb::Error>(())
        })
        .await?;
    Ok(())
}

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
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn profile() -> Result<Vec<PersonRecord>, String> {
    ensure_connected().await.map_err(|e| e.to_string())?;

    Ok(SURREAL.select("person").await.unwrap())
}

#[tauri::command]
async fn saved_profiles() -> Result<Vec<String>, String> {
    ensure_connected().await.map_err(|e| e.to_string())?;

    let _created: Option<PersonRecord> = SURREAL
        .create("person")
        .content(Person { name: "Tyson".to_string(), age: 32 })
        .await
        .map_err(|e| e.to_string())?;

    Ok(vec![String::from("test")])
}

#[tauri::command]
fn run_datamaning(_token: &str) {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            saved_profiles,
            profile,
            run_datamaning
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}