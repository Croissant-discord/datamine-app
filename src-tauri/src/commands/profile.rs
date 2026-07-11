use crate::types::{ AppProfileRecord, AppProfile };
use crate::{ SURREAL, http::HttpClient};

#[tauri::command]
pub async fn profile_list() -> Result<Vec<AppProfileRecord>, String> {
    Ok(
        SURREAL.select("profiles").await.map_err(|e| e.to_string())?
    )
}

#[tauri::command]
pub async fn profile_add(token: &str) -> Result<AppProfileRecord, String> {
    let r: AppProfile = HttpClient::new(token).await.unwrap().into();

    let created: Option<AppProfileRecord> = SURREAL
        .create("profiles")
        .content(r)
        .await
        .map_err(|e| e.to_string())?;

    Ok(created.unwrap())
}

// #[tauri::command]
// pub async fn delete(id: &str) {
// } 