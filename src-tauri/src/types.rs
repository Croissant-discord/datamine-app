use surrealdb::{Surreal, types::{RecordId, SurrealValue}};
use serde::{Serialize, Deserialize};

// app types

#[derive(Debug, SurrealValue, Serialize, Deserialize)]
pub struct AppProfile {
    pub token: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, SurrealValue, Serialize, Deserialize)]
pub struct AppProfileRecord {
    pub id: RecordId,
    pub token: String,
    pub email: String,
    pub username: String,
}

// discord http types

#[derive(Deserialize)]
pub struct Me {
    pub id: String,
    pub username: String,
    pub email: String
}