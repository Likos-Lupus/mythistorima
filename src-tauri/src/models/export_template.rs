use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ExportTemplateConfig {
    pub include_title: bool,
    pub include_author: bool,
    pub include_chapter_title: bool,
    pub chapter_title_format: String,
    pub paragraph_separator: String,
    pub first_line_indent: bool,
    pub font_family: String,
    pub font_size: f64,
    pub line_height: f64,
    pub pixiv_page_break: bool,
}

impl Default for ExportTemplateConfig {
    fn default() -> Self {
        Self {
            include_title: true,
            include_author: true,
            include_chapter_title: true,
            chapter_title_format: "第 {index} 章 {title}".to_string(),
            paragraph_separator: "\n\n".to_string(),
            first_line_indent: true,
            font_family: "serif".to_string(),
            font_size: 12.0,
            line_height: 1.6,
            pixiv_page_break: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExportTemplateInput {
    pub project_id: String,
    pub name: String,
    pub format: String,
    pub config_json: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateExportTemplateInput {
    pub project_id: String,
    pub template_id: String,
    pub name: Option<String>,
    pub format: Option<String>,
    pub config_json: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListExportTemplatesInput {
    pub project_id: String,
    pub include_builtin: Option<bool>,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportWithTemplateInput {
    pub project_id: String,
    pub template_id: String,
    pub range: Option<String>,
    pub document_id: Option<String>,
    pub document_ids: Option<Vec<String>>,
    pub output_path: Option<String>,
    pub config_override_json: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportPreviewDto {
    pub format: String,
    pub content: String,
    pub document_count: i64,
    pub character_count: i64,
    pub truncated: bool,
}
