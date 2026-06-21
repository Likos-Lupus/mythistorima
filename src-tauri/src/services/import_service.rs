use std::{fs, path::Path};

use chrono::Utc;
use serde_json::{json, Value};
use sqlx::SqlitePool;

use crate::{
    errors::{AppError, AppResult},
    models::{
        document::{CreateDocumentInput, UpdateDocumentContentInput},
        import::{ImportResultDto, ImportTextFileInput},
    },
    services::document_service,
};

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn infer_format(path: &str, explicit: Option<String>) -> String {
    if let Some(format) = explicit.filter(|value| !value.trim().is_empty()) {
        return format.trim().to_lowercase();
    }
    Path::new(path)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("txt")
        .to_lowercase()
}

fn title_from_path(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("导入章节")
        .to_string()
}

fn strip_markdown_heading(text: &str) -> (Option<String>, String) {
    let mut lines = text.lines();
    if let Some(first) = lines.next() {
        let trimmed = first.trim();
        if let Some(title) = trimmed
            .strip_prefix("# ")
            .or_else(|| trimmed.strip_prefix("## "))
            .or_else(|| trimmed.strip_prefix("### "))
        {
            return (
                Some(title.trim().to_string()),
                lines.collect::<Vec<_>>().join("\n"),
            );
        }
    }
    (None, text.to_string())
}

fn strip_html(raw: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for ch in raw.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                out.push('\n');
            }
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out.replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
}

fn document_json_from_text(text: &str) -> Value {
    let paragraphs: Vec<Value> = text
        .split('\n')
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            json!({
                "type": "paragraph",
                "attrs": { "pid": format!("p_{}", uuid::Uuid::new_v4().simple()) },
                "content": [{ "type": "text", "text": line }]
            })
        })
        .collect();

    json!({
        "type": "doc",
        "content": if paragraphs.is_empty() { vec![json!({"type":"paragraph", "attrs": {"pid": format!("p_{}", uuid::Uuid::new_v4().simple())}})] } else { paragraphs }
    })
}

fn html_from_text(text: &str) -> String {
    text.split('\n')
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| format!("<p>{}</p>", escape_html(line)))
        .collect::<Vec<_>>()
        .join("")
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub async fn import_text_file(
    pool: &SqlitePool,
    input: ImportTextFileInput,
) -> AppResult<ImportResultDto> {
    let format = infer_format(&input.path, input.format.clone());
    if !matches!(format.as_str(), "txt" | "md" | "markdown" | "html" | "htm") {
        return Err(AppError::invalid_input(
            "导入格式必须是 txt / markdown / html",
        ));
    }

    let raw = fs::read_to_string(&input.path).map_err(|error| {
        AppError::with_detail("IMPORT_READ_FAILED", "读取导入文件失败", error.to_string())
    })?;

    let (detected_title, content_text, content_html) = match format.as_str() {
        "md" | "markdown" => {
            let (title, text) = strip_markdown_heading(&raw);
            let html = html_from_text(&text);
            (title, text, html)
        }
        "html" | "htm" => {
            let text = strip_html(&raw);
            (None, text, raw)
        }
        _ => (None, raw.clone(), html_from_text(&raw)),
    };

    let title = input
        .title
        .filter(|value| !value.trim().is_empty())
        .or(detected_title)
        .unwrap_or_else(|| title_from_path(&input.path));
    let character_count = content_text
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .count() as i64;

    let document = document_service::create_document(
        pool,
        CreateDocumentInput {
            project_id: input.project_id,
            parent_id: input.parent_id,
            document_type: "chapter".to_string(),
            title: title.clone(),
            sort_order: None,
        },
    )
    .await?;

    let content_json = document_json_from_text(&content_text);
    document_service::update_document_content(
        pool,
        UpdateDocumentContentInput {
            document_id: document.id.clone(),
            content_json,
            content_text,
            content_html,
            character_count,
        },
    )
    .await?;

    Ok(ImportResultDto {
        document_id: document.id,
        title,
        character_count,
        imported_at: now_ms(),
    })
}
