use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::Utc;
use serde_json::{json, Value};
use sqlx::{Row, SqlitePool};

use crate::{
    errors::{AppError, AppResult},
    models::{
        export::{ExportDocumentsInput, ExportProjectInput, ExportResultDto},
        export_template::{
            ExportPreviewDto, ExportTemplateConfig, ExportTemplateDto, ExportWithTemplateInput,
        },
    },
    services::publication_export_service::{
        self, PublicationAsset, PublicationDocumentSource, PublicationProject,
    },
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

struct ProjectRow {
    id: String,
    title: String,
    author: Option<String>,
    language: String,
    cover_asset_id: Option<String>,
    raw: Value,
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
        "txt" | "pixiv" => "txt",
        "markdown" | "md" => "md",
        "html" => "html",
        "docx" => "docx",
        "epub" => "epub",
        "project_package" | "package" => "mythistorima.json",
        _ => "txt",
    }
}

fn normalize_format(format: &str) -> AppResult<String> {
    match format.trim() {
        "txt" => Ok("txt".to_string()),
        "markdown" | "md" => Ok("markdown".to_string()),
        "html" => Ok("html".to_string()),
        "docx" => Ok("docx".to_string()),
        "epub" => Ok("epub".to_string()),
        "pixiv" => Ok("pixiv".to_string()),
        "project_package" | "package" => Ok("project_package".to_string()),
        _ => Err(AppError::invalid_input(
            "导出格式必须是 txt / markdown / html / docx / epub / pixiv / project_package",
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

    let config = legacy_config(&format);
    let path = resolve_output_path(database_path, input.output_path, &project.title, &format)?;
    write_export_format(
        pool,
        database_path,
        &path,
        &project,
        &documents,
        &format,
        &config,
    )
    .await?;

    Ok(export_result(&path, &format, &documents))
}

pub async fn export_with_template_config(
    pool: &SqlitePool,
    database_path: &Path,
    input: ExportWithTemplateInput,
    template: &ExportTemplateDto,
    config: &ExportTemplateConfig,
) -> AppResult<ExportResultDto> {
    let format = normalize_format(&template.format)?;
    if format == "project_package" {
        return Err(AppError::invalid_input(
            "项目包不使用导出模板，请使用项目包导出功能",
        ));
    }

    let project = get_project_row(pool, &input.project_id).await?;
    let documents = get_export_documents(
        pool,
        &input.project_id,
        &ExportDocumentsInput {
            project_id: input.project_id.clone(),
            format: format.clone(),
            range: input.range.clone(),
            document_id: input.document_id.clone(),
            document_ids: input.document_ids.clone(),
            output_path: input.output_path.clone(),
        },
    )
    .await?;

    if documents.is_empty() {
        return Err(AppError::invalid_input("没有可导出的文档"));
    }

    let path = resolve_output_path(database_path, input.output_path, &project.title, &format)?;
    write_export_format(
        pool,
        database_path,
        &path,
        &project,
        &documents,
        &format,
        config,
    )
    .await?;

    Ok(export_result(&path, &format, &documents))
}

pub async fn preview_with_template_config(
    pool: &SqlitePool,
    input: ExportWithTemplateInput,
    template: &ExportTemplateDto,
    config: &ExportTemplateConfig,
) -> AppResult<ExportPreviewDto> {
    let format = normalize_format(&template.format)?;
    let project = get_project_row(pool, &input.project_id).await?;
    let documents = get_export_documents(
        pool,
        &input.project_id,
        &ExportDocumentsInput {
            project_id: input.project_id.clone(),
            format: format.clone(),
            range: input.range,
            document_id: input.document_id,
            document_ids: input.document_ids,
            output_path: None,
        },
    )
    .await?;

    if documents.is_empty() {
        return Err(AppError::invalid_input("没有可预览的文档"));
    }

    let preview_document_count = documents.len().min(8);
    let preview_documents = &documents[..preview_document_count];
    let publication_project = publication_project(&project);
    let publication_documents = publication_documents(preview_documents);
    let rendered = match format.as_str() {
        "docx" | "epub" => publication_export_service::render_publication_preview_html(
            &publication_project,
            &publication_documents,
            config,
            &format,
        ),
        "pixiv" => publication_export_service::render_pixiv(
            &publication_project,
            &publication_documents,
            config,
        ),
        _ => render_with_config(&project, preview_documents, &format, config)?,
    };
    let (content, content_truncated) = truncate_chars(rendered, 30_000);

    Ok(ExportPreviewDto {
        format,
        content,
        document_count: documents.len() as i64,
        character_count: documents.iter().map(|item| item.character_count).sum(),
        truncated: content_truncated || documents.len() > preview_document_count,
    })
}

fn export_result(path: &Path, format: &str, documents: &[ExportDocumentRow]) -> ExportResultDto {
    ExportResultDto {
        path: path.to_string_lossy().to_string(),
        format: format.to_string(),
        document_count: documents.len() as i64,
        character_count: documents.iter().map(|item| item.character_count).sum(),
        created_at: now_ms(),
    }
}

fn legacy_config(format: &str) -> ExportTemplateConfig {
    let mut config = ExportTemplateConfig::default();
    config.include_author = false;
    config.chapter_title_format = "{title}".to_string();
    config.first_line_indent = false;
    config.pixiv_page_break = true;

    match format {
        "txt" | "markdown" => config.include_title = false,
        "html" | "pixiv" => config.include_title = true,
        _ => {}
    }
    config
}

fn truncate_chars(value: String, max_chars: usize) -> (String, bool) {
    if value.chars().count() <= max_chars {
        return (value, false);
    }
    let mut truncated: String = value.chars().take(max_chars).collect();
    truncated.push_str("\n\n……预览内容已截断……");
    (truncated, true)
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

    Ok(export_result(&path, "project_package", &documents))
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

    let id: String = row.try_get("id")?;
    let title: String = row.try_get("title")?;
    let author: Option<String> = row.try_get("author")?;
    let language: String = row.try_get("language")?;
    let cover_asset_id: Option<String> = row.try_get("cover_asset_id")?;
    Ok(ProjectRow {
        id: id.clone(),
        title: title.clone(),
        author: author.clone(),
        language: language.clone(),
        cover_asset_id: cover_asset_id.clone(),
        raw: json!({
            "id": id,
            "title": title,
            "author": author,
            "description": row.try_get::<Option<String>, _>("description")?,
            "status": row.try_get::<String, _>("status")?,
            "language": language,
            "coverAssetId": cover_asset_id,
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
        "all" => all,
        _ => {
            return Err(AppError::invalid_input(
                "导出范围必须是 all / current / selected",
            ));
        }
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

async fn write_export_format(
    pool: &SqlitePool,
    database_path: &Path,
    path: &Path,
    project: &ProjectRow,
    documents: &[ExportDocumentRow],
    format: &str,
    config: &ExportTemplateConfig,
) -> AppResult<()> {
    let publication_project = publication_project(project);
    let publication_documents = publication_documents(documents);

    match format {
        "docx" => publication_export_service::write_docx(
            path,
            &publication_project,
            &publication_documents,
            config,
        ),
        "epub" => {
            let assets = query_publication_assets(pool, &project.id).await?;
            publication_export_service::write_epub(
                path,
                database_path,
                &publication_project,
                &publication_documents,
                &assets,
                config,
            )
        }
        "pixiv" => {
            let rendered = publication_export_service::render_pixiv(
                &publication_project,
                &publication_documents,
                config,
            );
            fs::write(path, rendered).map_err(|error| {
                AppError::with_detail(
                    "EXPORT_WRITE_FAILED",
                    "写入 Pixiv 导出文件失败",
                    error.to_string(),
                )
            })
        }
        _ => {
            let rendered = render_with_config(project, documents, format, config)?;
            fs::write(path, rendered).map_err(|error| {
                AppError::with_detail("EXPORT_WRITE_FAILED", "写入导出文件失败", error.to_string())
            })
        }
    }
}

fn publication_project(project: &ProjectRow) -> PublicationProject {
    PublicationProject {
        id: project.id.clone(),
        title: project.title.clone(),
        author: project.author.clone(),
        language: project.language.clone(),
        cover_asset_id: project.cover_asset_id.clone(),
    }
}

fn publication_documents(documents: &[ExportDocumentRow]) -> Vec<PublicationDocumentSource> {
    documents
        .iter()
        .map(|document| PublicationDocumentSource {
            id: document.id.clone(),
            document_type: document.document_type.clone(),
            title: document.title.clone(),
            depth: document.depth,
            content_json: document.content_json.clone(),
            content_text: document.content_text.clone(),
        })
        .collect()
}

async fn query_publication_assets(
    pool: &SqlitePool,
    project_id: &str,
) -> AppResult<Vec<PublicationAsset>> {
    let rows = sqlx::query(
        r#"
        SELECT id, filename, mime, path
        FROM assets
        WHERE project_id = ?1
        ORDER BY created_at ASC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    let mut assets = Vec::with_capacity(rows.len());
    for row in rows {
        assets.push(PublicationAsset {
            id: row.try_get("id")?,
            filename: row.try_get("filename")?,
            mime: row.try_get("mime")?,
            path: row.try_get("path")?,
        });
    }
    Ok(assets)
}

fn render_with_config(
    project: &ProjectRow,
    documents: &[ExportDocumentRow],
    format: &str,
    config: &ExportTemplateConfig,
) -> AppResult<String> {
    match format {
        "txt" => Ok(render_plain_text(project, documents, config, false)),
        "markdown" => Ok(render_markdown(project, documents, config)),
        "html" => Ok(render_html(project, documents, config)),
        _ => Err(AppError::invalid_input(
            "文本渲染格式必须是 txt / markdown / html",
        )),
    }
}

fn render_plain_text(
    project: &ProjectRow,
    documents: &[ExportDocumentRow],
    config: &ExportTemplateConfig,
    pixiv: bool,
) -> String {
    let mut header = Vec::new();
    if config.include_title {
        header.push(project.title.clone());
    }
    if config.include_author {
        if let Some(author) = project
            .author
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            header.push(format!("作者：{}", author.trim()));
        }
    }

    let mut sections = Vec::new();
    for (index, document) in documents.iter().enumerate() {
        let mut parts = Vec::new();
        if config.include_chapter_title {
            parts.push(format_document_title(
                document,
                index,
                &config.chapter_title_format,
            ));
        }
        let body = format_plain_body(
            &document.content_text,
            &config.paragraph_separator,
            config.first_line_indent,
        );
        if !body.is_empty() {
            parts.push(body);
        }
        if !parts.is_empty() {
            sections.push(parts.join("\n\n"));
        }
    }

    let mut output = String::new();
    if !header.is_empty() {
        output.push_str(&header.join("\n"));
        if !sections.is_empty() {
            output.push_str("\n\n");
        }
    }

    let document_separator = if pixiv && config.pixiv_page_break {
        "\n\n[newpage]\n\n"
    } else {
        "\n\n"
    };
    output.push_str(&sections.join(document_separator));
    if !output.ends_with('\n') {
        output.push('\n');
    }
    output
}

fn render_markdown(
    project: &ProjectRow,
    documents: &[ExportDocumentRow],
    config: &ExportTemplateConfig,
) -> String {
    let mut out = String::new();
    if config.include_title {
        out.push_str("# ");
        out.push_str(&project.title);
        out.push_str("\n\n");
    }
    if config.include_author {
        if let Some(author) = project
            .author
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            out.push_str("*作者：");
            out.push_str(author.trim());
            out.push_str("*\n\n");
        }
    }

    for (index, document) in documents.iter().enumerate() {
        if config.include_chapter_title {
            let base_level = if config.include_title { 2 } else { 1 };
            let level = (base_level + document.depth).clamp(1, 6);
            out.push_str(&"#".repeat(level));
            out.push(' ');
            out.push_str(&format_document_title(
                document,
                index,
                &config.chapter_title_format,
            ));
            out.push_str("\n\n");
        }
        let body = format_plain_body(
            &document.content_text,
            &config.paragraph_separator,
            config.first_line_indent,
        );
        if !body.is_empty() {
            out.push_str(&body);
            out.push_str("\n\n");
        }
    }
    out
}

fn render_html(
    project: &ProjectRow,
    documents: &[ExportDocumentRow],
    config: &ExportTemplateConfig,
) -> String {
    let font_family = css_font_family(&config.font_family);
    let text_indent = if config.first_line_indent { "2em" } else { "0" };
    let mut out = String::new();
    out.push_str("<!doctype html><html lang=\"zh-CN\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>");
    out.push_str(&escape_html(&project.title));
    out.push_str("</title><style>");
    out.push_str("body{box-sizing:border-box;max-width:820px;margin:48px auto;padding:0 28px;color:#2f2a24;background:#fffdf8;");
    out.push_str("font-family:");
    out.push_str(font_family);
    out.push_str(";font-size:");
    out.push_str(&format!("{:.1}pt", config.font_size));
    out.push_str(";line-height:");
    out.push_str(&format!("{:.2}", config.line_height));
    out.push_str("}h1{margin-bottom:.25em}header.book-header{margin-bottom:3rem}.author{color:#6f665e}section{margin-bottom:2.8rem}section .content p{text-indent:");
    out.push_str(text_indent);
    out.push_str(";margin:.75em 0}blockquote{border-left:3px solid #b7a88f;margin:1em 0;padding:.2em 1em;color:#665f57}ul,ol{padding-left:2em}</style></head><body>");

    if config.include_title || config.include_author {
        out.push_str("<header class=\"book-header\">");
        if config.include_title {
            out.push_str("<h1>");
            out.push_str(&escape_html(&project.title));
            out.push_str("</h1>");
        }
        if config.include_author {
            if let Some(author) = project
                .author
                .as_deref()
                .filter(|value| !value.trim().is_empty())
            {
                out.push_str("<p class=\"author\">作者：");
                out.push_str(&escape_html(author.trim()));
                out.push_str("</p>");
            }
        }
        out.push_str("</header>");
    }

    for (index, document) in documents.iter().enumerate() {
        out.push_str("<section data-document-id=\"");
        out.push_str(&escape_html(&document.id));
        out.push_str("\">");
        if config.include_chapter_title {
            let base_level = if config.include_title { 2 } else { 1 };
            let level = (base_level + document.depth).clamp(1, 6);
            out.push_str("<h");
            out.push_str(&level.to_string());
            out.push('>');
            out.push_str(&escape_html(&format_document_title(
                document,
                index,
                &config.chapter_title_format,
            )));
            out.push_str("</h");
            out.push_str(&level.to_string());
            out.push('>');
        }
        out.push_str("<div class=\"content\">");
        if document.content_html.trim().is_empty() {
            for paragraph in document
                .content_text
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
            {
                out.push_str("<p>");
                out.push_str(&escape_html(paragraph));
                out.push_str("</p>");
            }
        } else {
            out.push_str(&document.content_html);
        }
        out.push_str("</div></section>");
    }
    out.push_str("</body></html>");
    out
}

fn format_plain_body(
    content_text: &str,
    paragraph_separator: &str,
    first_line_indent: bool,
) -> String {
    content_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|paragraph| {
            if first_line_indent {
                format!("　　{}", paragraph)
            } else {
                paragraph.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(paragraph_separator)
}

fn format_document_title(document: &ExportDocumentRow, index: usize, pattern: &str) -> String {
    pattern
        .replace("{index}", &(index + 1).to_string())
        .replace("{title}", &document.title)
        .replace("{type}", document_type_label(&document.document_type))
        .replace("{depth}", &document.depth.to_string())
}

fn document_type_label(document_type: &str) -> &'static str {
    match document_type {
        "volume" => "卷",
        "chapter" => "章",
        "scene" => "场景",
        "note" => "文档",
        _ => "文档",
    }
}

fn css_font_family(font_family: &str) -> &'static str {
    match font_family {
        "sans-serif" => "Inter,'Noto Sans SC','Microsoft YaHei',sans-serif",
        "monospace" => "'JetBrains Mono','Noto Sans Mono CJK SC',monospace",
        "system" => "system-ui,-apple-system,'Segoe UI','Microsoft YaHei',sans-serif",
        _ => "Georgia,'Times New Roman','Noto Serif SC','Songti SC',serif",
    }
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
