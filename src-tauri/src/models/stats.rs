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

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectOverviewInput {
    pub project_id: String,
    pub day_start: i64,
    pub day_end: i64,
    pub trend_start: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct OverviewDocumentDto {
    pub id: String,
    pub title: String,
    pub document_type: String,
    pub status: String,
    pub character_count: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct OverviewNoteDto {
    pub id: String,
    pub title: String,
    pub note_type: String,
    pub status: String,
    pub priority: String,
    pub document_id: Option<String>,
    pub document_title: Option<String>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct OverviewForeshadowDto {
    pub id: String,
    pub title: String,
    pub status: String,
    pub priority: String,
    pub setup_document_id: Option<String>,
    pub setup_document_title: Option<String>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct OverviewCharacterDto {
    pub card_id: String,
    pub card_name: String,
    pub document_count: i64,
    pub mention_count: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct WritingTrendPointDto {
    pub day_start: i64,
    pub character_count: i64,
    pub elapsed_ms: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectOverviewDto {
    pub project_id: String,
    pub document_count: i64,
    pub completed_document_count: i64,
    pub character_count: i64,
    pub today_character_count: i64,
    pub today_elapsed_ms: i64,
    pub writing_streak_days: i64,
    pub recent_documents: Vec<OverviewDocumentDto>,
    pub open_notes: Vec<OverviewNoteDto>,
    pub unresolved_foreshadows: Vec<OverviewForeshadowDto>,
    pub top_characters: Vec<OverviewCharacterDto>,
    pub writing_trend: Vec<WritingTrendPointDto>,
}
