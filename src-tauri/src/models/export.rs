use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportDocumentsInput {
    pub project_id: String,
    pub format: String,
    pub range: Option<String>,
    pub document_id: Option<String>,
    pub document_ids: Option<Vec<String>>,
    pub output_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportProjectInput {
    pub project_id: String,
    pub format: Option<String>,
    pub output_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResultDto {
    pub path: String,
    pub format: String,
    pub document_count: i64,
    pub character_count: i64,
    pub created_at: i64,
}
