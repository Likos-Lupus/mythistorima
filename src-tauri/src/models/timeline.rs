use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEventDto {
    pub id: String,
    pub project_id: String,
    pub linked_card_id: Option<String>,
    pub linked_document_id: Option<String>,
    pub title: String,
    pub description: String,
    pub starts_at_label: Option<String>,
    pub ends_at_label: Option<String>,
    pub sort_key: i64,
    pub location_card_id: Option<String>,
    pub metadata_json: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEventCardDto {
    pub id: String,
    pub project_id: String,
    pub timeline_event_id: String,
    pub card_id: String,
    pub role: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEventInput {
    pub project_id: String,
    pub linked_card_id: Option<String>,
    pub linked_document_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub starts_at_label: Option<String>,
    pub ends_at_label: Option<String>,
    pub sort_key: Option<i64>,
    pub location_card_id: Option<String>,
    pub metadata_json: Option<String>,
}
