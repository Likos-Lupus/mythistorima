use chrono::Utc;
use serde_json::{Map, Value};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::{
        export::ExportResultDto,
        export_template::{
            CreateExportTemplateInput, ExportPreviewDto, ExportTemplateConfig, ExportTemplateDto,
            ExportWithTemplateInput, ListExportTemplatesInput, UpdateExportTemplateInput,
        },
    },
    services::export_service,
};

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn normalize_name(value: &str) -> AppResult<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AppError::invalid_input("导出模板名称不能为空"));
    }
    if trimmed.chars().count() > 120 {
        return Err(AppError::invalid_input("导出模板名称不能超过 120 个字符"));
    }
    Ok(trimmed.to_string())
}

fn normalize_format(value: &str) -> AppResult<String> {
    match value.trim() {
        "txt" => Ok("txt".to_string()),
        "markdown" | "md" => Ok("markdown".to_string()),
        "html" => Ok("html".to_string()),
        "pixiv" => Ok("pixiv".to_string()),
        _ => Err(AppError::invalid_input(
            "模板格式必须是 txt / markdown / html / pixiv",
        )),
    }
}

fn parse_json_object(raw: &str, label: &str) -> AppResult<Map<String, Value>> {
    let value = serde_json::from_str::<Value>(raw).map_err(|error| {
        AppError::with_detail(
            "INVALID_EXPORT_TEMPLATE_CONFIG",
            format!("{}无法解析", label),
            error.to_string(),
        )
    })?;
    value
        .as_object()
        .cloned()
        .ok_or_else(|| AppError::invalid_input(format!("{}必须是 JSON 对象", label)))
}

fn normalize_config(mut config: ExportTemplateConfig) -> AppResult<ExportTemplateConfig> {
    let title_format = config.chapter_title_format.trim();
    if title_format.is_empty() {
        config.chapter_title_format = "{title}".to_string();
    } else if title_format.chars().count() > 160 {
        return Err(AppError::invalid_input("章节标题格式不能超过 160 个字符"));
    } else {
        config.chapter_title_format = title_format.to_string();
    }

    if config.paragraph_separator.chars().count() > 12 {
        return Err(AppError::invalid_input("段落分隔符不能超过 12 个字符"));
    }
    if config.paragraph_separator.is_empty() {
        config.paragraph_separator = "\n\n".to_string();
    }

    config.font_family = match config.font_family.trim() {
        "serif" => "serif",
        "sans-serif" => "sans-serif",
        "monospace" => "monospace",
        "system" => "system",
        _ => {
            return Err(AppError::invalid_input(
                "字体必须是 serif / sans-serif / monospace / system",
            ));
        }
    }
    .to_string();

    if !config.font_size.is_finite() {
        config.font_size = 12.0;
    }
    if !config.line_height.is_finite() {
        config.line_height = 1.6;
    }
    config.font_size = config.font_size.clamp(8.0, 72.0);
    config.line_height = config.line_height.clamp(1.0, 3.0);

    Ok(config)
}

fn canonicalize_config_json(raw: Option<String>) -> AppResult<String> {
    let config = match raw {
        Some(value) if !value.trim().is_empty() => {
            serde_json::from_str::<ExportTemplateConfig>(&value).map_err(|error| {
                AppError::with_detail(
                    "INVALID_EXPORT_TEMPLATE_CONFIG",
                    "导出模板配置无法解析",
                    error.to_string(),
                )
            })?
        }
        _ => ExportTemplateConfig::default(),
    };

    let normalized = normalize_config(config)?;
    serde_json::to_string(&normalized).map_err(|error| {
        AppError::with_detail(
            "INVALID_EXPORT_TEMPLATE_CONFIG",
            "导出模板配置无法序列化",
            error.to_string(),
        )
    })
}

fn merge_config_json(base: &str, override_json: Option<String>) -> AppResult<String> {
    let Some(override_json) = override_json.filter(|value| !value.trim().is_empty()) else {
        return canonicalize_config_json(Some(base.to_string()));
    };

    let mut merged = parse_json_object(base, "模板配置")?;
    for (key, value) in parse_json_object(&override_json, "模板覆盖配置")? {
        merged.insert(key, value);
    }

    let value = Value::Object(merged);
    canonicalize_config_json(Some(value.to_string()))
}

async fn ensure_project_exists(pool: &SqlitePool, project_id: &str) -> AppResult<()> {
    let exists: Option<(String,)> = sqlx::query_as("SELECT id FROM projects WHERE id = ?1 LIMIT 1")
        .bind(project_id)
        .fetch_optional(pool)
        .await?;
    if exists.is_none() {
        return Err(AppError::not_found("project"));
    }
    Ok(())
}

async fn touch_project(pool: &SqlitePool, project_id: &str, now: i64) -> AppResult<()> {
    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(project_id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn ensure_builtin_templates(pool: &SqlitePool) -> AppResult<()> {
    let now = now_ms();
    let templates = [
        (
            "builtin_txt_submission",
            "投稿 TXT 模板",
            "txt",
            r#"{"includeTitle":true,"includeAuthor":true,"includeChapterTitle":true,"chapterTitleFormat":"第 {index} 章 {title}","paragraphSeparator":"\n\n","firstLineIndent":true,"fontFamily":"serif","fontSize":12.0,"lineHeight":1.6,"pixivPageBreak":false}"#,
        ),
        (
            "builtin_markdown_archive",
            "归档 Markdown 模板",
            "markdown",
            r#"{"includeTitle":true,"includeAuthor":true,"includeChapterTitle":true,"chapterTitleFormat":"{title}","paragraphSeparator":"\n\n","firstLineIndent":false,"fontFamily":"serif","fontSize":12.0,"lineHeight":1.6,"pixivPageBreak":false}"#,
        ),
        (
            "builtin_html_review",
            "审稿 HTML 模板",
            "html",
            r#"{"includeTitle":true,"includeAuthor":true,"includeChapterTitle":true,"chapterTitleFormat":"第 {index} 章 {title}","paragraphSeparator":"\n\n","firstLineIndent":true,"fontFamily":"serif","fontSize":12.0,"lineHeight":1.8,"pixivPageBreak":false}"#,
        ),
        (
            "builtin_pixiv_basic",
            "Pixiv 基础模板",
            "pixiv",
            r#"{"includeTitle":true,"includeAuthor":false,"includeChapterTitle":true,"chapterTitleFormat":"{title}","paragraphSeparator":"\n\n","firstLineIndent":false,"fontFamily":"serif","fontSize":12.0,"lineHeight":1.6,"pixivPageBreak":true}"#,
        ),
    ];

    for (id, name, format, config_json) in templates {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO export_templates
              (id, project_id, name, format, config_json, is_builtin, created_at, updated_at)
            VALUES (?1, NULL, ?2, ?3, ?4, 1, ?5, ?6)
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(format)
        .bind(config_json)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;

        sqlx::query(
            r#"
            UPDATE export_templates
            SET name = ?1,
                format = ?2,
                config_json = ?3,
                is_builtin = 1,
                updated_at = ?4
            WHERE id = ?5 AND is_builtin = 1
            "#,
        )
        .bind(name)
        .bind(format)
        .bind(config_json)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn get_template_for_project(
    pool: &SqlitePool,
    project_id: &str,
    template_id: &str,
) -> AppResult<ExportTemplateDto> {
    sqlx::query_as::<_, ExportTemplateDto>(
        r#"
        SELECT id, project_id, name, format, config_json, is_builtin, created_at, updated_at
        FROM export_templates
        WHERE id = ?1
          AND (project_id = ?2 OR project_id IS NULL)
        LIMIT 1
        "#,
    )
    .bind(template_id)
    .bind(project_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("export_template"))
}

pub async fn create_export_template(
    pool: &SqlitePool,
    input: CreateExportTemplateInput,
) -> AppResult<ExportTemplateDto> {
    ensure_builtin_templates(pool).await?;
    ensure_project_exists(pool, &input.project_id).await?;

    let id = Uuid::new_v4().to_string();
    let name = normalize_name(&input.name)?;
    let format = normalize_format(&input.format)?;
    let config_json = canonicalize_config_json(input.config_json)?;
    let now = now_ms();

    sqlx::query(
        r#"
        INSERT INTO export_templates
          (id, project_id, name, format, config_json, is_builtin, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, 0, ?6, ?7)
        "#,
    )
    .bind(&id)
    .bind(&input.project_id)
    .bind(name)
    .bind(format)
    .bind(config_json)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    touch_project(pool, &input.project_id, now).await?;
    get_template_for_project(pool, &input.project_id, &id).await
}

pub async fn update_export_template(
    pool: &SqlitePool,
    input: UpdateExportTemplateInput,
) -> AppResult<ExportTemplateDto> {
    ensure_builtin_templates(pool).await?;
    ensure_project_exists(pool, &input.project_id).await?;

    let current = get_template_for_project(pool, &input.project_id, &input.template_id).await?;
    if current.is_builtin != 0 || current.project_id.is_none() {
        return Err(AppError::new(
            "BUILTIN_EXPORT_TEMPLATE_READ_ONLY",
            "内置导出模板不能直接修改，请先复制为项目模板",
        ));
    }

    let name = match input.name {
        Some(value) => normalize_name(&value)?,
        None => current.name,
    };
    let format = match input.format {
        Some(value) => normalize_format(&value)?,
        None => current.format,
    };
    let config_json = match input.config_json {
        Some(value) => canonicalize_config_json(Some(value))?,
        None => canonicalize_config_json(Some(current.config_json))?,
    };
    let now = now_ms();

    sqlx::query(
        r#"
        UPDATE export_templates
        SET name = ?1,
            format = ?2,
            config_json = ?3,
            updated_at = ?4
        WHERE id = ?5 AND project_id = ?6 AND is_builtin = 0
        "#,
    )
    .bind(name)
    .bind(format)
    .bind(config_json)
    .bind(now)
    .bind(&input.template_id)
    .bind(&input.project_id)
    .execute(pool)
    .await?;

    touch_project(pool, &input.project_id, now).await?;
    get_template_for_project(pool, &input.project_id, &input.template_id).await
}

pub async fn delete_export_template(
    pool: &SqlitePool,
    project_id: String,
    template_id: String,
) -> AppResult<bool> {
    ensure_builtin_templates(pool).await?;
    ensure_project_exists(pool, &project_id).await?;
    let current = get_template_for_project(pool, &project_id, &template_id).await?;

    if current.is_builtin != 0 || current.project_id.is_none() {
        return Err(AppError::new(
            "BUILTIN_EXPORT_TEMPLATE_READ_ONLY",
            "内置导出模板不能删除",
        ));
    }

    let result = sqlx::query(
        "DELETE FROM export_templates WHERE id = ?1 AND project_id = ?2 AND is_builtin = 0",
    )
    .bind(template_id)
    .bind(&project_id)
    .execute(pool)
    .await?;

    if result.rows_affected() > 0 {
        touch_project(pool, &project_id, now_ms()).await?;
    }
    Ok(result.rows_affected() > 0)
}

pub async fn list_export_templates(
    pool: &SqlitePool,
    input: ListExportTemplatesInput,
) -> AppResult<Vec<ExportTemplateDto>> {
    ensure_builtin_templates(pool).await?;
    ensure_project_exists(pool, &input.project_id).await?;

    let include_builtin = input.include_builtin.unwrap_or(true);
    let format = match input.format {
        Some(value) if !value.trim().is_empty() => Some(normalize_format(&value)?),
        _ => None,
    };

    let templates = sqlx::query_as::<_, ExportTemplateDto>(
        r#"
        SELECT id, project_id, name, format, config_json, is_builtin, created_at, updated_at
        FROM export_templates
        WHERE (project_id = ?1 OR (?2 = 1 AND project_id IS NULL))
          AND (?3 IS NULL OR format = ?3)
        ORDER BY is_builtin DESC, format ASC, name COLLATE NOCASE ASC, updated_at DESC
        "#,
    )
    .bind(&input.project_id)
    .bind(if include_builtin { 1_i64 } else { 0_i64 })
    .bind(format)
    .fetch_all(pool)
    .await?;

    Ok(templates)
}

pub async fn export_with_template(
    pool: &SqlitePool,
    database_path: &std::path::Path,
    input: ExportWithTemplateInput,
) -> AppResult<ExportResultDto> {
    ensure_builtin_templates(pool).await?;
    ensure_project_exists(pool, &input.project_id).await?;
    let template = get_template_for_project(pool, &input.project_id, &input.template_id).await?;
    let config_json = merge_config_json(&template.config_json, input.config_override_json.clone())?;
    let config = serde_json::from_str::<ExportTemplateConfig>(&config_json).map_err(|error| {
        AppError::with_detail(
            "INVALID_EXPORT_TEMPLATE_CONFIG",
            "导出模板配置无法解析",
            error.to_string(),
        )
    })?;

    export_service::export_with_template_config(pool, database_path, input, &template, &config)
        .await
}

pub async fn preview_export_with_template(
    pool: &SqlitePool,
    input: ExportWithTemplateInput,
) -> AppResult<ExportPreviewDto> {
    ensure_builtin_templates(pool).await?;
    ensure_project_exists(pool, &input.project_id).await?;
    let template = get_template_for_project(pool, &input.project_id, &input.template_id).await?;
    let config_json = merge_config_json(&template.config_json, input.config_override_json.clone())?;
    let config = serde_json::from_str::<ExportTemplateConfig>(&config_json).map_err(|error| {
        AppError::with_detail(
            "INVALID_EXPORT_TEMPLATE_CONFIG",
            "导出模板配置无法解析",
            error.to_string(),
        )
    })?;

    export_service::preview_with_template_config(pool, input, &template, &config).await
}
