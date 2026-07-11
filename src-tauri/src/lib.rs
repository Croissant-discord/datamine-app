use surrealdb::{Surreal, engine::local::{RocksDb, Db}};
use std::sync::LazyLock;

pub mod commands;

pub mod http;
pub mod types;

use crate::commands::*;

pub static SURREAL: LazyLock<Surreal<Db>> = LazyLock::new(Surreal::init);

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
            profile::profile_add,
            profile::profile_list,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}