use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ProofreadingRuleDto {
    pub id: String,
    pub project_id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub rule_type: String,
    pub pattern: Option<String>,
    pub config_json: String,
    pub severity: String,
    pub enabled: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProofreadingIssueDto {
    pub id: String,
    pub rule_id: Option<String>,
    pub document_id: String,
    pub paragraph_id: Option<String>,
    pub severity: String,
    pub message: String,
    pub start_offset: Option<i64>,
    pub end_offset: Option<i64>,
    pub suggestion: Option<String>,
}
