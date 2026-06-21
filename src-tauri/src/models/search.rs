use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultDto {
    pub target_type: String,
    pub target_id: String,
    pub project_id: String,
    pub title: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchProjectInput {
    pub project_id: String,
    pub query: String,
    pub scopes: Vec<String>,
    pub limit: Option<i64>,
}
