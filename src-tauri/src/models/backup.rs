use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupDto {
    pub id: String,
    pub project_id: String,
    pub path: String,
    pub size_bytes: i64,
    pub created_at: i64,
}
