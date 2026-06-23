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

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CardAppearanceDto {
    pub id: String,
    pub project_id: String,
    pub document_id: String,
    pub document_title: String,
    #[serde(rename = "documentType")]
    pub document_type: String,
    pub card_id: String,
    pub card_name: String,
    #[serde(rename = "cardType")]
    pub card_type: String,
    pub mention_count: i64,
    pub first_seen_position: Option<i64>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CardAppearanceSummaryDto {
    pub card_id: String,
    pub card_name: String,
    #[serde(rename = "cardType")]
    pub card_type: String,
    pub document_count: i64,
    pub total_mentions: i64,
    pub first_document_id: Option<String>,
    pub first_document_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DocumentAppearanceDocumentDto {
    pub document_id: String,
    pub document_title: String,
    #[serde(rename = "documentType")]
    pub document_type: String,
    pub sort_order: i64,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DocumentAppearanceCellDto {
    pub document_id: String,
    pub card_id: String,
    pub mention_count: i64,
    pub first_seen_position: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectAppearanceSummaryDto {
    pub project_id: String,
    pub total_cards: i64,
    pub appeared_cards: i64,
    pub total_documents: i64,
    pub documents_with_appearances: i64,
    pub total_mentions: i64,
    pub cards: Vec<CardAppearanceSummaryDto>,
    pub documents: Vec<DocumentAppearanceDocumentDto>,
    pub appearances: Vec<DocumentAppearanceCellDto>,
}
