use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ExportTemplateDto {
    pub id: String,
    pub project_id: Option<String>,
    pub name: String,
    pub format: String,
    pub config_json: String,
    pub is_builtin: i64,
    pub created_at: i64,
    pub updated_at: i64,
}
