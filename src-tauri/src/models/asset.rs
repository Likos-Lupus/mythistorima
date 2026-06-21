use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AssetDto {
    pub id: String,
    pub project_id: Option<String>,
    #[serde(rename = "type")]
    pub asset_type: String,
    pub filename: String,
    pub mime: Option<String>,
    pub path: String,
    pub hash: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssetInput {
    pub project_id: Option<String>,
    #[serde(rename = "type")]
    pub asset_type: String,
    pub filename: String,
    pub mime: Option<String>,
    pub path: String,
    pub hash: Option<String>,
}
