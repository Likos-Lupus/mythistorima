use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportTextFileInput {
    pub project_id: String,
    pub path: String,
    pub format: Option<String>,
    pub parent_id: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResultDto {
    pub document_id: String,
    pub title: String,
    pub character_count: i64,
    pub imported_at: i64,
}
