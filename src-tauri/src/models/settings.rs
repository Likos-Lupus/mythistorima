use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AppSettingDto {
    pub key: String,
    pub value_json: String,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSettingDto {
    pub project_id: String,
    pub key: String,
    pub value_json: String,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAppSettingInput {
    pub key: String,
    pub value_json: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetProjectSettingInput {
    pub project_id: String,
    pub key: String,
    pub value_json: String,
}
