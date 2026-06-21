use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfoDto {
    pub name: String,
    pub version: String,
    pub database_path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStatsDto {
    pub project_id: String,
    pub document_count: i64,
    pub character_count: i64,
    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentStatsDto {
    pub document_id: String,
    pub character_count: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SchemaMigrationDto {
    pub version: i64,
    pub name: String,
    pub applied_at: i64,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordWritingSessionInput {
    pub session_id: String,
    pub project_id: String,
    pub document_id: String,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub characters_before: i64,
    pub characters_after: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct WritingSessionDto {
    pub id: String,
    pub project_id: String,
    pub document_id: String,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub characters_before: i64,
    pub characters_after: i64,
    pub inserted_count: i64,
    pub deleted_count: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TodayWritingStatsDto {
    pub project_id: String,
    pub character_count: i64,
    pub elapsed_ms: i64,
}
