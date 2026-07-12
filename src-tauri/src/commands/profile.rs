use crate::types::{ AppProfileRecord, AppProfile };
use crate::{ SURREAL, http::HttpClient};
use crate::error::AppError;

#[tauri::command]
pub async fn profile_list() -> Result<Vec<AppProfileRecord>, AppError> {
    Ok(
        SURREAL.select("profiles").await?
    )
}

#[tauri::command]
pub async fn profile_add(token: &str) -> Result<AppProfileRecord, AppError> {
    let r: AppProfile = HttpClient::new(token).await?
        .into();

    let created: Option<AppProfileRecord> = SURREAL
        .create("profiles")
        .content(r)
        .await?;

    Ok(created.unwrap())
}

#[tauri::command]
pub async fn profile_delete(id: &str) -> Result<(), AppError> {
    Ok(())
} 