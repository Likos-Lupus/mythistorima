use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::Utc;
use serde_json::{json, Value};
use sqlx::{Row, SqlitePool};

use crate::{
    errors::{AppError, AppResult},
    models::export::{ExportDocumentsInput, ExportProjectInput, ExportResultDto},
};

#[derive(Debug, Clone)]
struct ExportDocumentRow {
    id: String,
    parent_id: Option<String>,
    document_type: String,
    title: String,
    sort_order: i64,
    content_text: String,
    content_html: String,
    content_json: String,
    character_count: i64,
    depth: usize,
}

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn sanitize_filename(value: &str) -> String {
    let sanitized: String = value
        .chars()
        .map(|ch| match ch {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            ch if ch.is_control() => '_',
            ch => ch,
        })
        .collect();
    let trimmed = sanitized.trim();
    if trimmed.is_empty() {
        "mythistorima".to_string()
    } else {
        trimmed.to_string()
    }
}

fn ensure_output_dir(database_path: &Path) -> AppResult<PathBuf> {
    let base = database_path.parent().unwrap_or_else(|| Path::new("."));
    let dir = base.join("exports");
    fs::create_dir_all(&dir).map_err(|error| {
        AppError::with_detail("EXPORT_DIR_FAILED", "无法创建导出目录", error.to_string())
    })?;
    Ok(dir)
}

fn default_extension(format: &str) -> &'static str {
    match format {
        "txt" => "txt",
        "markdown" | "md" => "md",
        "html" => "html",
        "project_package" | "package" => "mythistorima.json",
        _ => "txt",
    }
}

fn normalize_format(format: &str) -> AppResult<String> {
    match format.trim() {
        "txt" => Ok("txt".to_string()),
        "markdown" | "md" => Ok("markdown".to_string()),
        "html" => Ok("html".to_string()),
        "project_package" | "package" => Ok("project_package".to_string()),
        _ => Err(AppError::invalid_input(
            "导出格式必须是 txt / markdown / html / project_package",
        )),
    }
}

fn resolve_output_path(
    database_path: &Path,
    output_path: Option<String>,
    project_title: &str,
    format: &str,
) -> AppResult<PathBuf> {
    if let Some(path) = output_path.filter(|value| !value.trim().is_empty()) {
        let target = PathBuf::from(path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                AppError::with_detail("EXPORT_DIR_FAILED", "无法创建导出目录", error.to_string())
            })?;
        }
        return Ok(target);
    }

    let dir = ensure_output_dir(database_path)?;
    let file_name = format!(
        "{}-{}.{}",
        sanitize_filename(project_title),
        now_ms(),
        default_extension(format)
    );
    Ok(dir.join(file_name))
}

pub async fn export_documents(
    pool: &SqlitePool,
    database_path: &Path,
    input: ExportDocumentsInput,
) -> AppResult<ExportResultDto> {
    let format = normalize_format(&input.format)?;
    if format == "project_package" {
        return export_project_package(pool, database_path, input.project_id, input.output_path)
            .await;
    }

    export_documents_as_format(pool, database_path, input, format).await
}

pub async fn export_project(
    pool: &SqlitePool,
    database_path: &Path,
    input: ExportProjectInput,
) -> AppResult<ExportResultDto> {
    let format = normalize_format(input.format.as_deref().unwrap_or("project_package"))?;
    if format == "project_package" {
        return export_project_package(pool, database_path, input.project_id, input.output_path)
            .await;
    }

    export_documents_as_format(
        pool,
        database_path,
        ExportDocumentsInput {
            project_id: input.project_id,
            format: format.clone(),
            range: Some("all".to_string()),
            document_id: None,
            document_ids: None,
            output_path: input.output_path,
        },
        format,
    )
    .await
}

async fn export_documents_as_format(
    pool: &SqlitePool,
    database_path: &Path,
    input: ExportDocumentsInput,
    format: String,
) -> AppResult<ExportResultDto> {
    let project = get_project_row(pool, &input.project_id).await?;
    let documents = get_export_documents(pool, &input.project_id, &input).await?;
    if documents.is_empty() {
        return Err(AppError::invalid_input("没有可导出的文档"));
    }

    let rendered = match format.as_str() {
        "txt" => render_txt(&documents),
        "markdown" => render_markdown(&documents),
        "html" => render_html(&project.title, &documents),
        _ => {
            return Err(AppError::invalid_input(
                "文档导出格式必须是 txt / markdown / html",
            ));
        }
    };
    let path = resolve_output_path(database_path, input.output_path, &project.title, &format)?;
    fs::write(&path, rendered).map_err(|error| {
        AppError::with_detail("EXPORT_WRITE_FAILED", "写入导出文件失败", error.to_string())
    })?;

    Ok(ExportResultDto {
        path: path.to_string_lossy().to_string(),
        format,
        document_count: documents.len() as i64,
        character_count: documents.iter().map(|item| item.character_count).sum(),
        created_at: now_ms(),
    })
}

async fn export_project_package(
    pool: &SqlitePool,
    database_path: &Path,
    project_id: String,
    output_path: Option<String>,
) -> AppResult<ExportResultDto> {
    let project = get_project_row(pool, &project_id).await?;
    let documents = query_all_documents(pool, &project_id).await?;
    let cards = query_json_rows(pool, "cards", &project_id).await?;
    let notes = query_json_rows(pool, "notes", &project_id).await?;
    let package = json!({
        "format": "mythistorima.project_package",
        "version": 1,
        "exportedAt": now_ms(),
        "project": project.raw,
        "documents": documents.iter().map(|item| json!({
            "id": item.id,
            "parentId": item.parent_id,
            "type": item.document_type,
            "title": item.title,
            "sortOrder": item.sort_order,
            "contentJson": serde_json::from_str::<Value>(&item.content_json).unwrap_or_else(|_| json!({"type":"doc","content":[{"type":"paragraph"}]})),
            "contentText": item.content_text,
            "contentHtml": item.content_html,
            "characterCount": item.character_count
        })).collect::<Vec<_>>(),
        "cards": cards,
        "notes": notes,
    });

    let path = resolve_output_path(
        database_path,
        output_path,
        &project.title,
        "project_package",
    )?;
    let data = serde_json::to_string_pretty(&package).map_err(|error| {
        AppError::with_detail(
            "EXPORT_SERIALIZE_FAILED",
            "项目包序列化失败",
            error.to_string(),
        )
    })?;
    fs::write(&path, data).map_err(|error| {
        AppError::with_detail("EXPORT_WRITE_FAILED", "写入项目包失败", error.to_string())
    })?;

    Ok(ExportResultDto {
        path: path.to_string_lossy().to_string(),
        format: "project_package".to_string(),
        document_count: documents.len() as i64,
        character_count: documents.iter().map(|item| item.character_count).sum(),
        created_at: now_ms(),
    })
}

struct ProjectRow {
    title: String,
    raw: Value,
}

async fn get_project_row(pool: &SqlitePool, project_id: &str) -> AppResult<ProjectRow> {
    let row = sqlx::query(
        r#"
        SELECT id, title, author, description, status, language, cover_asset_id,
               target_character_count, daily_target_count, metadata_json, created_at, updated_at
        FROM projects WHERE id = ?1
        "#,
    )
    .bind(project_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("project"))?;

    let title: String = row.try_get("title")?;
    Ok(ProjectRow {
        title: title.clone(),
        raw: json!({
            "id": row.try_get::<String, _>("id")?,
            "title": title,
            "author": row.try_get::<Option<String>, _>("author")?,
            "description": row.try_get::<Option<String>, _>("description")?,
            "status": row.try_get::<String, _>("status")?,
            "language": row.try_get::<String, _>("language")?,
            "coverAssetId": row.try_get::<Option<String>, _>("cover_asset_id")?,
            "targetCharacterCount": row.try_get::<Option<i64>, _>("target_character_count")?,
            "dailyTargetCount": row.try_get::<Option<i64>, _>("daily_target_count")?,
            "metadataJson": row.try_get::<String, _>("metadata_json")?,
            "createdAt": row.try_get::<i64, _>("created_at")?,
            "updatedAt": row.try_get::<i64, _>("updated_at")?,
        }),
    })
}

async fn query_json_rows(
    pool: &SqlitePool,
    table: &'static str,
    project_id: &str,
) -> AppResult<Vec<Value>> {
    if table == "cards" {
        let rows = sqlx::query(
            r#"
            SELECT id, project_id, type, name, aliases_json, description, fields_json, avatar_asset_id, created_at, updated_at
            FROM cards WHERE project_id = ?1 ORDER BY type ASC, updated_at DESC
            "#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await?;

        let mut values = Vec::new();
        for row in rows {
            values.push(json!({
                "id": row.try_get::<String, _>("id")?,
                "projectId": row.try_get::<String, _>("project_id")?,
                "type": row.try_get::<String, _>("type")?,
                "name": row.try_get::<String, _>("name")?,
                "aliasesJson": row.try_get::<String, _>("aliases_json")?,
                "description": row.try_get::<String, _>("description")?,
                "fieldsJson": row.try_get::<String, _>("fields_json")?,
                "avatarAssetId": row.try_get::<Option<String>, _>("avatar_asset_id")?,
                "createdAt": row.try_get::<i64, _>("created_at")?,
                "updatedAt": row.try_get::<i64, _>("updated_at")?,
            }));
        }
        return Ok(values);
    }

    if table == "notes" {
        let rows = sqlx::query(
            r#"
            SELECT id, project_id, document_id, paragraph_id, type, title, body, status, priority, due_at, created_at, updated_at
            FROM notes WHERE project_id = ?1 ORDER BY updated_at DESC
            "#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await?;

        let mut values = Vec::new();
        for row in rows {
            values.push(json!({
                "id": row.try_get::<String, _>("id")?,
                "projectId": row.try_get::<String, _>("project_id")?,
                "documentId": row.try_get::<Option<String>, _>("document_id")?,
                "paragraphId": row.try_get::<Option<String>, _>("paragraph_id")?,
                "type": row.try_get::<String, _>("type")?,
                "title": row.try_get::<String, _>("title")?,
                "body": row.try_get::<String, _>("body")?,
                "status": row.try_get::<String, _>("status")?,
                "priority": row.try_get::<String, _>("priority")?,
                "dueAt": row.try_get::<Option<i64>, _>("due_at")?,
                "createdAt": row.try_get::<i64, _>("created_at")?,
                "updatedAt": row.try_get::<i64, _>("updated_at")?,
            }));
        }
        return Ok(values);
    }

    Ok(vec![])
}

async fn get_export_documents(
    pool: &SqlitePool,
    project_id: &str,
    input: &ExportDocumentsInput,
) -> AppResult<Vec<ExportDocumentRow>> {
    let all = query_all_documents(pool, project_id).await?;
    let selected = match input.range.as_deref().unwrap_or("all") {
        "current" => input
            .document_id
            .as_ref()
            .map(|id| collect_subtree(&all, id))
            .unwrap_or_default(),
        "selected" => {
            let ids = input.document_ids.clone().unwrap_or_default();
            all.into_iter()
                .filter(|item| ids.contains(&item.id))
                .collect()
        }
        _ => all,
    };
    Ok(selected)
}

async fn query_all_documents(
    pool: &SqlitePool,
    project_id: &str,
) -> AppResult<Vec<ExportDocumentRow>> {
    let rows = sqlx::query(
        r#"
        SELECT d.id, d.parent_id, d.type, d.title, d.sort_order,
               COALESCE(c.content_text, '') AS content_text,
               COALESCE(c.content_html, '') AS content_html,
               COALESCE(c.content_json, '{"type":"doc","content":[{"type":"paragraph"}]}') AS content_json,
               d.character_count
        FROM documents d
        LEFT JOIN document_contents c ON c.document_id = d.id
        WHERE d.project_id = ?1
        ORDER BY d.sort_order ASC, d.created_at ASC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    let mut items = Vec::new();
    for row in rows {
        items.push(ExportDocumentRow {
            id: row.try_get("id")?,
            parent_id: row.try_get("parent_id")?,
            document_type: row.try_get("type")?,
            title: row.try_get("title")?,
            sort_order: row.try_get("sort_order")?,
            content_text: row.try_get("content_text")?,
            content_html: row.try_get("content_html")?,
            content_json: row.try_get("content_json")?,
            character_count: row.try_get("character_count")?,
            depth: 0,
        });
    }
    Ok(order_documents(items))
}

fn order_documents(items: Vec<ExportDocumentRow>) -> Vec<ExportDocumentRow> {
    fn walk(
        parent_id: Option<&str>,
        depth: usize,
        source: &[ExportDocumentRow],
        out: &mut Vec<ExportDocumentRow>,
    ) {
        let mut children: Vec<ExportDocumentRow> = source
            .iter()
            .filter(|item| item.parent_id.as_deref() == parent_id)
            .cloned()
            .collect();
        children.sort_by(|a, b| a.sort_order.cmp(&b.sort_order).then(a.title.cmp(&b.title)));
        for mut child in children {
            child.depth = depth;
            let id = child.id.clone();
            out.push(child);
            walk(Some(&id), depth + 1, source, out);
        }
    }
    let mut out = Vec::new();
    walk(None, 0, &items, &mut out);
    out
}

fn collect_subtree(items: &[ExportDocumentRow], document_id: &str) -> Vec<ExportDocumentRow> {
    fn walk(items: &[ExportDocumentRow], id: &str, out: &mut Vec<ExportDocumentRow>) {
        if let Some(item) = items.iter().find(|item| item.id == id) {
            out.push(item.clone());
            let mut children: Vec<_> = items
                .iter()
                .filter(|child| child.parent_id.as_deref() == Some(id))
                .cloned()
                .collect();
            children.sort_by(|a, b| a.sort_order.cmp(&b.sort_order));
            for child in children {
                walk(items, &child.id, out);
            }
        }
    }
    let mut out = Vec::new();
    walk(items, document_id, &mut out);
    out
}

fn render_txt(documents: &[ExportDocumentRow]) -> String {
    let mut out = String::new();
    for doc in documents {
        out.push_str(&doc.title);
        out.push_str("\n\n");
        if !doc.content_text.trim().is_empty() {
            out.push_str(doc.content_text.trim());
            out.push_str("\n\n");
        }
    }
    out
}

fn render_markdown(documents: &[ExportDocumentRow]) -> String {
    let mut out = String::new();
    for doc in documents {
        let level = (doc.depth + 1).clamp(1, 6);
        out.push_str(&"#".repeat(level));
        out.push(' ');
        out.push_str(&doc.title);
        out.push_str("\n\n");
        if !doc.content_text.trim().is_empty() {
            out.push_str(doc.content_text.trim());
            out.push_str("\n\n");
        }
    }
    out
}

fn render_html(project_title: &str, documents: &[ExportDocumentRow]) -> String {
    let mut out = String::new();
    out.push_str("<!doctype html><html lang=\"zh-CN\"><head><meta charset=\"utf-8\"><title>");
    out.push_str(&escape_html(project_title));
    out.push_str("</title><style>body{font-family:serif;line-height:1.8;max-width:760px;margin:48px auto;padding:0 24px;color:#2f2a24;background:#f7f1e5}section{margin-bottom:2.5rem}</style></head><body>");
    out.push_str("<h1>");
    out.push_str(&escape_html(project_title));
    out.push_str("</h1>");
    for doc in documents {
        let level = (doc.depth + 2).clamp(2, 6);
        out.push_str("<section><h");
        out.push_str(&level.to_string());
        out.push('>');
        out.push_str(&escape_html(&doc.title));
        out.push_str("</h");
        out.push_str(&level.to_string());
        out.push('>');
        if doc.content_html.trim().is_empty() {
            for paragraph in doc
                .content_text
                .split('\n')
                .filter(|line| !line.trim().is_empty())
            {
                out.push_str("<p>");
                out.push_str(&escape_html(paragraph.trim()));
                out.push_str("</p>");
            }
        } else {
            out.push_str(&doc.content_html);
        }
        out.push_str("</section>");
    }
    out.push_str("</body></html>");
    out
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
