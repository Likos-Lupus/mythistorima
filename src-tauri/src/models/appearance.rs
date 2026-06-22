use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AppearanceStatDto {
    pub id: String,
    pub project_id: String,
    pub document_id: String,
    pub card_id: String,
    pub mention_count: i64,
    pub first_seen_position: Option<i64>,
    pub updated_at: i64,
}
