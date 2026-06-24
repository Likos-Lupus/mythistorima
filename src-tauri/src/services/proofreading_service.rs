use chrono::Utc;
use regex::{Regex, RegexBuilder};
use serde_json::Value;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::proofreading::{
        CreateProofreadingRuleInput, ListProofreadingRulesInput, ProofreadingIssueDto,
        ProofreadingRuleDto, RunProofreadingOnDocumentInput, RunProofreadingOnProjectInput,
        UpdateProofreadingRuleInput,
    },
};

#[derive(Debug, Clone)]
struct DocumentText {
    document_id: String,
    project_id: String,
    title: String,
    content_text: String,
    content_json: String,
}

#[derive(Debug, Clone)]
struct ParagraphSlice {
    paragraph_id: String,
    text: String,
    start_offset: usize,
}

#[derive(Debug, Clone)]
struct CharacterName {
    name: String,
    card_name: String,
}

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn normalize_non_empty(value: &str, label: &str, max_len: usize) -> AppResult<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AppError::invalid_input(format!("{}不能为空", label)));
    }
    if trimmed.chars().count() > max_len {
        return Err(AppError::invalid_input(format!(
            "{}不能超过 {} 个字符",
            label, max_len
        )));
    }
    Ok(trimmed.to_string())
}

fn normalize_optional_id(value: Option<String>) -> Option<String> {
    value.and_then(|raw| {
        let trimmed = raw.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

fn normalize_optional_text(
    value: Option<String>,
    label: &str,
    max_len: usize,
) -> AppResult<Option<String>> {
    match value {
        Some(raw) => {
            let trimmed = raw.trim().to_string();
            if trimmed.chars().count() > max_len {
                return Err(AppError::invalid_input(format!(
                    "{}不能超过 {} 个字符",
                    label, max_len
                )));
            }
            if trimmed.is_empty() {
                Ok(None)
            } else {
                Ok(Some(trimmed))
            }
        }
        None => Ok(None),
    }
}

fn normalize_rule_type(value: &str) -> AppResult<String> {
    let normalized = value.trim();
    match normalized {
        "duplicate_word"
        | "continuous_punctuation"
        | "mixed_punctuation"
        | "long_sentence"
        | "long_paragraph"
        | "sensitive_word"
        | "name_consistency"
        | "custom_regex" => Ok(normalized.to_string()),
        _ => Err(AppError::invalid_input(
            "校对规则类型必须是 duplicate_word / continuous_punctuation / mixed_punctuation / long_sentence / long_paragraph / sensitive_word / name_consistency / custom_regex",
        )),
    }
}

fn normalize_severity(value: &str) -> AppResult<String> {
    let normalized = value.trim();
    match normalized {
        "info" | "warning" | "error" => Ok(normalized.to_string()),
        _ => Err(AppError::invalid_input(
            "校对级别必须是 info / warning / error",
        )),
    }
}

fn normalize_config_json(raw: Option<String>) -> AppResult<String> {
    match raw {
        Some(value) if !value.trim().is_empty() => {
            let parsed = serde_json::from_str::<Value>(&value).map_err(|error| {
                AppError::with_detail(
                    "INVALID_PROOFREADING_CONFIG_JSON",
                    "校对规则 config_json 无法解析",
                    error.to_string(),
                )
            })?;
            if !parsed.is_object() {
                return Err(AppError::invalid_input(
                    "校对规则 config_json 必须是 JSON 对象",
                ));
            }
            serde_json::to_string(&parsed).map_err(|error| {
                AppError::with_detail(
                    "INVALID_PROOFREADING_CONFIG_JSON",
                    "校对规则 config_json 无法序列化",
                    error.to_string(),
                )
            })
        }
        _ => Ok("{}".to_string()),
    }
}

fn parse_config(config_json: &str) -> Value {
    serde_json::from_str::<Value>(config_json).unwrap_or_else(|_| serde_json::json!({}))
}

fn config_i64(config: &Value, key: &str, fallback: i64) -> i64 {
    config
        .get(key)
        .and_then(|value| value.as_i64())
        .unwrap_or(fallback)
        .max(1)
}

fn config_string_list(config: &Value, key: &str) -> Vec<String> {
    config
        .get(key)
        .and_then(|value| value.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str())
                .map(|item| item.trim().to_string())
                .filter(|item| !item.is_empty())
                .collect()
        })
        .unwrap_or_default()
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

async fn get_rule(pool: &SqlitePool, rule_id: &str) -> AppResult<ProofreadingRuleDto> {
    sqlx::query_as::<_, ProofreadingRuleDto>(
        r#"
        SELECT
          id,
          project_id,
          name,
          type AS rule_type,
          pattern,
          config_json,
          severity,
          enabled,
          created_at,
          updated_at
        FROM proofreading_rules
        WHERE id = ?1
        LIMIT 1
        "#,
    )
    .bind(rule_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("proofreading_rule"))
}

async fn get_document_text(pool: &SqlitePool, document_id: &str) -> AppResult<DocumentText> {
    let row = sqlx::query(
        r#"
        SELECT
          d.id AS document_id,
          d.project_id,
          d.title,
          COALESCE(c.content_text, '') AS content_text,
          COALESCE(c.content_json, '') AS content_json
        FROM documents d
        LEFT JOIN document_contents c ON c.document_id = d.id
        WHERE d.id = ?1
        LIMIT 1
        "#,
    )
    .bind(document_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("document"))?;

    Ok(DocumentText {
        document_id: row.try_get("document_id")?,
        project_id: row.try_get("project_id")?,
        title: row.try_get("title")?,
        content_text: row.try_get("content_text")?,
        content_json: row.try_get("content_json")?,
    })
}

async fn list_project_documents(
    pool: &SqlitePool,
    project_id: &str,
) -> AppResult<Vec<DocumentText>> {
    ensure_project_exists(pool, project_id).await?;

    let rows = sqlx::query(
        r#"
        SELECT
          d.id AS document_id,
          d.project_id,
          d.title,
          COALESCE(c.content_text, '') AS content_text,
          COALESCE(c.content_json, '') AS content_json
        FROM documents d
        LEFT JOIN document_contents c ON c.document_id = d.id
        WHERE d.project_id = ?1
          AND d.type IN ('chapter', 'scene')
        ORDER BY
          CASE WHEN d.parent_id IS NULL THEN 0 ELSE 1 END ASC,
          d.parent_id ASC,
          d.sort_order ASC,
          d.created_at ASC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    let documents = rows
        .into_iter()
        .map(|row| {
            Ok(DocumentText {
                document_id: row.try_get("document_id")?,
                project_id: row.try_get("project_id")?,
                title: row.try_get("title")?,
                content_text: row.try_get("content_text")?,
                content_json: row.try_get("content_json")?,
            })
        })
        .collect::<Result<Vec<_>, sqlx::Error>>()?;

    Ok(documents)
}

async fn ensure_builtin_rules(pool: &SqlitePool) -> AppResult<()> {
    let now = now_ms();
    let rules = [
        (
            "builtin_duplicate_word",
            "重复词检查",
            "duplicate_word",
            None::<&str>,
            r#"{"maxUnitCharacters":4}"#,
            "warning",
        ),
        (
            "builtin_continuous_punctuation",
            "连续标点检查",
            "continuous_punctuation",
            Some(r#"[!！?？。…~、，,；;：:]{3,}"#),
            r#"{"minCount":3}"#,
            "warning",
        ),
        (
            "builtin_mixed_punctuation",
            "中英文标点混用检查",
            "mixed_punctuation",
            None::<&str>,
            r#"{"pairs":["，,","，.","。,","。.","！!","!！","？?","?？","：:",":：","；;",";；"]}"#,
            "info",
        ),
        (
            "builtin_long_sentence",
            "超长句检查",
            "long_sentence",
            None::<&str>,
            r#"{"maxCharacters":120}"#,
            "info",
        ),
        (
            "builtin_long_paragraph",
            "超长段落检查",
            "long_paragraph",
            None::<&str>,
            r#"{"maxCharacters":500}"#,
            "info",
        ),
        (
            "builtin_sensitive_word",
            "敏感词检查",
            "sensitive_word",
            Some("敏感词,违禁词"),
            r#"{"words":["敏感词","违禁词"]}"#,
            "warning",
        ),
        (
            "builtin_name_consistency",
            "设定名称一致性检查",
            "name_consistency",
            None::<&str>,
            r#"{"maxDistance":1,"minNameCharacters":2}"#,
            "warning",
        ),
    ];

    for (id, name, rule_type, pattern, config_json, severity) in rules {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO proofreading_rules
              (id, project_id, name, type, pattern, config_json, severity, enabled, created_at, updated_at)
            VALUES (?1, NULL, ?2, ?3, ?4, ?5, ?6, 1, ?7, ?8)
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(rule_type)
        .bind(pattern)
        .bind(config_json)
        .bind(severity)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn create_proofreading_rule(
    pool: &SqlitePool,
    input: CreateProofreadingRuleInput,
) -> AppResult<ProofreadingRuleDto> {
    ensure_builtin_rules(pool).await?;

    let project_id = normalize_optional_id(input.project_id);
    if let Some(project_id) = project_id.as_deref() {
        ensure_project_exists(pool, project_id).await?;
    }

    let name = normalize_non_empty(&input.name, "规则名称", 120)?;
    let rule_type = normalize_rule_type(&input.rule_type)?;
    let pattern = normalize_optional_text(input.pattern, "规则 pattern", 4000)?;
    let config_json = normalize_config_json(input.config_json)?;
    let severity = normalize_severity(input.severity.as_deref().unwrap_or("warning"))?;
    let enabled = input.enabled.unwrap_or(1).clamp(0, 1);
    let rule_id = Uuid::new_v4().to_string();
    let now = now_ms();

    if rule_type == "custom_regex" {
        let pattern_ref = pattern
            .as_deref()
            .ok_or_else(|| AppError::invalid_input("自定义正则规则必须填写 pattern"))?;
        Regex::new(pattern_ref).map_err(|error| {
            AppError::with_detail("INVALID_REGEX", "自定义正则无法编译", error.to_string())
        })?;
    }

    sqlx::query(
        r#"
        INSERT INTO proofreading_rules
          (id, project_id, name, type, pattern, config_json, severity, enabled, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        "#,
    )
    .bind(&rule_id)
    .bind(project_id.as_deref())
    .bind(&name)
    .bind(&rule_type)
    .bind(pattern.as_deref())
    .bind(&config_json)
    .bind(&severity)
    .bind(enabled)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    if let Some(project_id) = project_id.as_deref() {
        touch_project(pool, project_id, now).await?;
    }

    get_rule(pool, &rule_id).await
}

pub async fn update_proofreading_rule(
    pool: &SqlitePool,
    input: UpdateProofreadingRuleInput,
) -> AppResult<ProofreadingRuleDto> {
    ensure_builtin_rules(pool).await?;

    let existing = get_rule(pool, &input.rule_id).await?;
    if existing.project_id.is_none() {
        return Err(AppError::invalid_input(
            "内置校对规则不能直接修改，请新建项目规则覆盖使用",
        ));
    }

    let name = match input.name {
        Some(value) => normalize_non_empty(&value, "规则名称", 120)?,
        None => existing.name.clone(),
    };
    let rule_type = match input.rule_type {
        Some(value) => normalize_rule_type(&value)?,
        None => existing.rule_type.clone(),
    };
    let pattern = match input.pattern {
        Some(value) => normalize_optional_text(Some(value), "规则 pattern", 4000)?,
        None => existing.pattern.clone(),
    };
    let config_json = match input.config_json {
        Some(value) => normalize_config_json(Some(value))?,
        None => existing.config_json.clone(),
    };
    let severity = match input.severity {
        Some(value) => normalize_severity(&value)?,
        None => existing.severity.clone(),
    };
    let enabled = input.enabled.unwrap_or(existing.enabled).clamp(0, 1);
    let now = now_ms();

    if rule_type == "custom_regex" {
        let pattern_ref = pattern
            .as_deref()
            .ok_or_else(|| AppError::invalid_input("自定义正则规则必须填写 pattern"))?;
        Regex::new(pattern_ref).map_err(|error| {
            AppError::with_detail("INVALID_REGEX", "自定义正则无法编译", error.to_string())
        })?;
    }

    sqlx::query(
        r#"
        UPDATE proofreading_rules
        SET name = ?1,
            type = ?2,
            pattern = ?3,
            config_json = ?4,
            severity = ?5,
            enabled = ?6,
            updated_at = ?7
        WHERE id = ?8
        "#,
    )
    .bind(&name)
    .bind(&rule_type)
    .bind(pattern.as_deref())
    .bind(&config_json)
    .bind(&severity)
    .bind(enabled)
    .bind(now)
    .bind(&input.rule_id)
    .execute(pool)
    .await?;

    if let Some(project_id) = existing.project_id.as_deref() {
        touch_project(pool, project_id, now).await?;
    }

    get_rule(pool, &input.rule_id).await
}

pub async fn delete_proofreading_rule(pool: &SqlitePool, rule_id: String) -> AppResult<bool> {
    ensure_builtin_rules(pool).await?;

    let existing = get_rule(pool, &rule_id).await?;
    if existing.project_id.is_none() {
        return Err(AppError::invalid_input("内置校对规则不能删除"));
    }

    let result = sqlx::query("DELETE FROM proofreading_rules WHERE id = ?1")
        .bind(&rule_id)
        .execute(pool)
        .await?;

    if let Some(project_id) = existing.project_id.as_deref() {
        touch_project(pool, project_id, now_ms()).await?;
    }

    Ok(result.rows_affected() > 0)
}

pub async fn list_proofreading_rules(
    pool: &SqlitePool,
    input: ListProofreadingRulesInput,
) -> AppResult<Vec<ProofreadingRuleDto>> {
    ensure_builtin_rules(pool).await?;

    let project_id = normalize_non_empty(&input.project_id, "项目 ID", 120)?;
    let include_builtin = if input.include_builtin.unwrap_or(true) {
        1_i64
    } else {
        0_i64
    };
    ensure_project_exists(pool, &project_id).await?;

    let rules = sqlx::query_as::<_, ProofreadingRuleDto>(
        r#"
        SELECT
          id,
          project_id,
          name,
          type AS rule_type,
          pattern,
          config_json,
          severity,
          enabled,
          created_at,
          updated_at
        FROM proofreading_rules
        WHERE project_id = ?1
           OR (?2 = 1 AND project_id IS NULL)
        ORDER BY
          CASE WHEN project_id IS NULL THEN 0 ELSE 1 END ASC,
          CASE type
            WHEN 'duplicate_word' THEN 0
            WHEN 'continuous_punctuation' THEN 1
            WHEN 'mixed_punctuation' THEN 2
            WHEN 'long_sentence' THEN 3
            WHEN 'long_paragraph' THEN 4
            WHEN 'sensitive_word' THEN 5
            WHEN 'name_consistency' THEN 6
            WHEN 'custom_regex' THEN 7
            ELSE 99
          END ASC,
          created_at ASC
        "#,
    )
    .bind(project_id)
    .bind(include_builtin)
    .fetch_all(pool)
    .await?;

    Ok(rules)
}

async fn load_rules_for_project(
    pool: &SqlitePool,
    project_id: &str,
    enabled_only: bool,
    rule_ids: &[String],
) -> AppResult<Vec<ProofreadingRuleDto>> {
    let enabled_flag = if enabled_only { 1_i64 } else { 0_i64 };
    let all_rules = list_proofreading_rules(
        pool,
        ListProofreadingRulesInput {
            project_id: project_id.to_string(),
            include_builtin: Some(true),
        },
    )
    .await?;

    let filtered = all_rules
        .into_iter()
        .filter(|rule| enabled_flag == 0 || rule.enabled != 0)
        .filter(|rule| rule_ids.is_empty() || rule_ids.iter().any(|id| id == &rule.id))
        .collect();

    Ok(filtered)
}

pub async fn run_proofreading_on_document(
    pool: &SqlitePool,
    input: RunProofreadingOnDocumentInput,
) -> AppResult<Vec<ProofreadingIssueDto>> {
    ensure_builtin_rules(pool).await?;

    let document_id = normalize_non_empty(&input.document_id, "文档 ID", 120)?;
    let document = get_document_text(pool, &document_id).await?;
    let rule_ids = input.rule_ids.unwrap_or_default();
    let rules = load_rules_for_project(
        pool,
        &document.project_id,
        input.enabled_only.unwrap_or(true),
        &rule_ids,
    )
    .await?;

    let character_names = load_character_names(pool, &document.project_id).await?;
    Ok(run_rules_on_document(&document, &rules, &character_names))
}

pub async fn run_proofreading_on_project(
    pool: &SqlitePool,
    input: RunProofreadingOnProjectInput,
) -> AppResult<Vec<ProofreadingIssueDto>> {
    ensure_builtin_rules(pool).await?;

    let project_id = normalize_non_empty(&input.project_id, "项目 ID", 120)?;
    let rule_ids = input.rule_ids.unwrap_or_default();
    let rules = load_rules_for_project(
        pool,
        &project_id,
        input.enabled_only.unwrap_or(true),
        &rule_ids,
    )
    .await?;
    let character_names = load_character_names(pool, &project_id).await?;
    let documents = list_project_documents(pool, &project_id).await?;

    let mut issues = Vec::new();
    for document in documents {
        issues.extend(run_rules_on_document(&document, &rules, &character_names));
    }

    Ok(issues)
}

async fn load_character_names(
    pool: &SqlitePool,
    project_id: &str,
) -> AppResult<Vec<CharacterName>> {
    let rows = sqlx::query(
        r#"
        SELECT name, aliases_json
        FROM cards
        WHERE project_id = ?1
          AND type = 'character'
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    let mut names = Vec::new();
    for row in rows {
        let card_name: String = row.try_get("name")?;
        push_unique_character_name(&mut names, &card_name, &card_name);

        let aliases_json: String = row.try_get("aliases_json")?;
        if let Ok(aliases) = serde_json::from_str::<Vec<String>>(&aliases_json) {
            for alias in aliases {
                push_unique_character_name(&mut names, &alias, &card_name);
            }
        }
    }

    Ok(names)
}

fn push_unique_character_name(names: &mut Vec<CharacterName>, raw_name: &str, card_name: &str) {
    let name = raw_name.trim();
    if name.chars().count() < 2 {
        return;
    }
    if names.iter().any(|item| item.name == name) {
        return;
    }
    names.push(CharacterName {
        name: name.to_string(),
        card_name: card_name.to_string(),
    });
}

fn split_paragraphs(text: &str, content_json: &str) -> Vec<ParagraphSlice> {
    let mut paragraphs = Vec::new();
    let mut offset = 0_usize;

    if let Ok(document) = serde_json::from_str::<Value>(content_json) {
        collect_tiptap_paragraphs(&document, &mut paragraphs, &mut offset);
    }

    if !paragraphs.is_empty() {
        return paragraphs;
    }

    for (index, line) in text.split('\n').enumerate() {
        let line_len = line.chars().count();
        if !line.trim().is_empty() {
            paragraphs.push(ParagraphSlice {
                paragraph_id: format!("paragraph-{}", index + 1),
                text: line.to_string(),
                start_offset: offset,
            });
        }
        offset += line_len + 1;
    }

    if paragraphs.is_empty() && !text.trim().is_empty() {
        paragraphs.push(ParagraphSlice {
            paragraph_id: "paragraph-1".to_string(),
            text: text.to_string(),
            start_offset: 0,
        });
    }

    paragraphs
}

fn collect_tiptap_paragraphs(
    node: &Value,
    paragraphs: &mut Vec<ParagraphSlice>,
    offset: &mut usize,
) {
    let node_type = node.get("type").and_then(Value::as_str).unwrap_or("");
    if node_type == "paragraph" || node_type == "heading" {
        let text = tiptap_node_text(node);
        let paragraph_id = node
            .get("attrs")
            .and_then(|attrs| attrs.get("pid"))
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
            .unwrap_or_else(|| format!("paragraph-{}", paragraphs.len() + 1));

        if !text.trim().is_empty() {
            paragraphs.push(ParagraphSlice {
                paragraph_id,
                text: text.clone(),
                start_offset: *offset,
            });
        }
        *offset += text.chars().count() + 1;
        return;
    }

    if let Some(children) = node.get("content").and_then(Value::as_array) {
        for child in children {
            collect_tiptap_paragraphs(child, paragraphs, offset);
        }
    }
}

fn tiptap_node_text(node: &Value) -> String {
    if let Some(text) = node.get("text").and_then(Value::as_str) {
        return text.to_string();
    }

    let mut output = String::new();
    if let Some(children) = node.get("content").and_then(Value::as_array) {
        for child in children {
            if child.get("type").and_then(Value::as_str) == Some("hardBreak") {
                output.push('\n');
            } else {
                output.push_str(&tiptap_node_text(child));
            }
        }
    }
    output
}

fn run_rules_on_document(
    document: &DocumentText,
    rules: &[ProofreadingRuleDto],
    character_names: &[CharacterName],
) -> Vec<ProofreadingIssueDto> {
    let mut issues = Vec::new();
    let paragraphs = split_paragraphs(&document.content_text, &document.content_json);

    for rule in rules {
        match rule.rule_type.as_str() {
            "duplicate_word" => {
                for paragraph in &paragraphs {
                    collect_duplicate_word_issues(document, paragraph, rule, &mut issues);
                }
            }
            "continuous_punctuation" => {
                for paragraph in &paragraphs {
                    collect_continuous_punctuation_issues(document, paragraph, rule, &mut issues);
                }
            }
            "mixed_punctuation" => {
                for paragraph in &paragraphs {
                    collect_mixed_punctuation_issues(document, paragraph, rule, &mut issues);
                }
            }
            "long_sentence" => {
                for paragraph in &paragraphs {
                    collect_long_sentence_issues(document, paragraph, rule, &mut issues);
                }
            }
            "long_paragraph" => {
                for paragraph in &paragraphs {
                    collect_long_paragraph_issues(document, paragraph, rule, &mut issues);
                }
            }
            "sensitive_word" => {
                for paragraph in &paragraphs {
                    collect_sensitive_word_issues(document, paragraph, rule, &mut issues);
                }
            }
            "name_consistency" => {
                for paragraph in &paragraphs {
                    collect_name_consistency_issues(
                        document,
                        paragraph,
                        rule,
                        character_names,
                        &mut issues,
                    );
                }
            }
            "custom_regex" => {
                for paragraph in &paragraphs {
                    collect_custom_regex_issues(document, paragraph, rule, &mut issues);
                }
            }
            _ => {}
        }
    }

    issues.sort_by(|a, b| {
        a.document_id
            .cmp(&b.document_id)
            .then(
                a.start_offset
                    .unwrap_or(0)
                    .cmp(&b.start_offset.unwrap_or(0)),
            )
            .then(a.rule_id.cmp(&b.rule_id))
    });
    issues
}

fn issue(
    document_id: &str,
    paragraph_id: &str,
    rule: &ProofreadingRuleDto,
    message: String,
    start_offset: usize,
    end_offset: usize,
    suggestion: Option<String>,
) -> ProofreadingIssueDto {
    ProofreadingIssueDto {
        id: Uuid::new_v4().to_string(),
        rule_id: Some(rule.id.clone()),
        document_id: document_id.to_string(),
        paragraph_id: Some(paragraph_id.to_string()),
        severity: rule.severity.clone(),
        message,
        start_offset: Some(start_offset as i64),
        end_offset: Some(end_offset as i64),
        suggestion,
    }
}

fn is_word_char(ch: char) -> bool {
    !ch.is_whitespace()
        && !matches!(
            ch,
            '，' | ','
                | '。'
                | '.'
                | '！'
                | '!'
                | '？'
                | '?'
                | '；'
                | ';'
                | '：'
                | ':'
                | '、'
                | '“'
                | '”'
                | '"'
                | '\''
                | '（'
                | '）'
                | '('
                | ')'
                | '【'
                | '】'
                | '['
                | ']'
                | '《'
                | '》'
                | '<'
                | '>'
                | '…'
                | '—'
                | '-'
                | '~'
        )
}

fn collect_duplicate_word_issues(
    document: &DocumentText,
    paragraph: &ParagraphSlice,
    rule: &ProofreadingRuleDto,
    issues: &mut Vec<ProofreadingIssueDto>,
) {
    let config = parse_config(&rule.config_json);
    let max_unit = config_i64(&config, "maxUnitCharacters", 4).clamp(1, 8) as usize;
    let chars: Vec<char> = paragraph.text.chars().collect();
    let mut seen_ranges: Vec<(usize, usize)> = Vec::new();

    for unit_len in (1..=max_unit).rev() {
        if chars.len() < unit_len * 2 {
            continue;
        }
        for start in 0..=(chars.len() - unit_len * 2) {
            let end = start + unit_len * 2;
            if seen_ranges.iter().any(|(s, e)| start >= *s && end <= *e) {
                continue;
            }
            let left = &chars[start..start + unit_len];
            let right = &chars[start + unit_len..end];
            if left != right || !left.iter().all(|ch| is_word_char(*ch)) {
                continue;
            }
            if unit_len == 1 && !is_cjk(left[0]) {
                continue;
            }

            let repeated: String = left.iter().collect();
            let start_offset = paragraph.start_offset + start;
            let end_offset = paragraph.start_offset + end;
            seen_ranges.push((start, end));
            issues.push(issue(
                &document.document_id,
                &paragraph.paragraph_id,
                rule,
                format!("发现重复词“{}{}”，建议检查是否误输入。", repeated, repeated),
                start_offset,
                end_offset,
                Some(repeated),
            ));
        }
    }
}

fn is_cjk(ch: char) -> bool {
    ('\u{4e00}'..='\u{9fff}').contains(&ch)
        || ('\u{3400}'..='\u{4dbf}').contains(&ch)
        || ('\u{f900}'..='\u{faff}').contains(&ch)
}

fn collect_continuous_punctuation_issues(
    document: &DocumentText,
    paragraph: &ParagraphSlice,
    rule: &ProofreadingRuleDto,
    issues: &mut Vec<ProofreadingIssueDto>,
) {
    let pattern = rule
        .pattern
        .as_deref()
        .unwrap_or(r#"[!！?？。…~、，,；;：:]{3,}"#);

    let regex = match Regex::new(pattern) {
        Ok(regex) => regex,
        Err(error) => {
            issues.push(ProofreadingIssueDto {
                id: Uuid::new_v4().to_string(),
                rule_id: Some(rule.id.clone()),
                document_id: document.document_id.clone(),
                paragraph_id: Some(paragraph.paragraph_id.clone()),
                severity: "error".to_string(),
                message: format!("连续标点规则正则无法编译：{}", error),
                start_offset: Some(paragraph.start_offset as i64),
                end_offset: Some(paragraph.start_offset as i64),
                suggestion: None,
            });
            return;
        }
    };

    for found in regex.find_iter(&paragraph.text) {
        let start = paragraph.start_offset + byte_to_char_offset(&paragraph.text, found.start());
        let end = paragraph.start_offset + byte_to_char_offset(&paragraph.text, found.end());
        issues.push(issue(
            &document.document_id,
            &paragraph.paragraph_id,
            rule,
            format!("连续标点“{}”较长，建议保留 1–2 个。", found.as_str()),
            start,
            end,
            Some(compact_punctuation(found.as_str())),
        ));
    }
}

fn compact_punctuation(value: &str) -> String {
    let mut chars = value.chars();
    match (chars.next(), chars.next()) {
        (Some(first), Some(second)) if first == second => format!("{}{}", first, second),
        (Some(first), _) => first.to_string(),
        _ => String::new(),
    }
}

fn collect_mixed_punctuation_issues(
    document: &DocumentText,
    paragraph: &ParagraphSlice,
    rule: &ProofreadingRuleDto,
    issues: &mut Vec<ProofreadingIssueDto>,
) {
    let config = parse_config(&rule.config_json);
    let pairs = config_string_list(&config, "pairs");
    let default_pairs = vec![
        "，,".to_string(),
        ",，".to_string(),
        "。.".to_string(),
        ".。".to_string(),
        "！!".to_string(),
        "!！".to_string(),
        "？?".to_string(),
        "?？".to_string(),
        "：:".to_string(),
        ":：".to_string(),
        "；;".to_string(),
        ";；".to_string(),
    ];
    let targets = if pairs.is_empty() {
        default_pairs
    } else {
        pairs
    };
    let chars: Vec<char> = paragraph.text.chars().collect();

    for start in 0..chars.len().saturating_sub(1) {
        let pair = format!("{}{}", chars[start], chars[start + 1]);
        if targets.iter().any(|target| target == &pair) {
            issues.push(issue(
                &document.document_id,
                &paragraph.paragraph_id,
                rule,
                format!("发现中英文标点混用“{}”，建议统一标点风格。", pair),
                paragraph.start_offset + start,
                paragraph.start_offset + start + 2,
                Some(normalize_punctuation_pair(&pair)),
            ));
        }
    }
}

fn normalize_punctuation_pair(pair: &str) -> String {
    let first = pair.chars().next().unwrap_or_default();
    let normalized = match first {
        ',' | '，' => '，',
        '.' | '。' => '。',
        '!' | '！' => '！',
        '?' | '？' => '？',
        ':' | '：' => '：',
        ';' | '；' => '；',
        _ => first,
    };
    normalized.to_string()
}

fn collect_long_sentence_issues(
    document: &DocumentText,
    paragraph: &ParagraphSlice,
    rule: &ProofreadingRuleDto,
    issues: &mut Vec<ProofreadingIssueDto>,
) {
    let config = parse_config(&rule.config_json);
    let max_chars = config_i64(&config, "maxCharacters", 120) as usize;
    let chars: Vec<char> = paragraph.text.chars().collect();

    let mut sentence_start = 0_usize;
    for (index, ch) in chars.iter().enumerate() {
        if is_sentence_end(*ch) {
            emit_long_sentence_if_needed(
                document,
                paragraph,
                rule,
                issues,
                &chars,
                sentence_start,
                index + 1,
                max_chars,
            );
            sentence_start = index + 1;
        }
    }

    if sentence_start < chars.len() {
        emit_long_sentence_if_needed(
            document,
            paragraph,
            rule,
            issues,
            &chars,
            sentence_start,
            chars.len(),
            max_chars,
        );
    }
}

fn is_sentence_end(ch: char) -> bool {
    matches!(ch, '。' | '！' | '!' | '？' | '?' | '；' | ';')
}

fn emit_long_sentence_if_needed(
    document: &DocumentText,
    paragraph: &ParagraphSlice,
    rule: &ProofreadingRuleDto,
    issues: &mut Vec<ProofreadingIssueDto>,
    chars: &[char],
    start: usize,
    end: usize,
    max_chars: usize,
) {
    let actual_len = chars[start..end]
        .iter()
        .filter(|ch| !ch.is_whitespace())
        .count();
    if actual_len <= max_chars {
        return;
    }

    issues.push(issue(
        &document.document_id,
        &paragraph.paragraph_id,
        rule,
        format!(
            "句子长度约 {} 字，超过当前阈值 {} 字。",
            actual_len, max_chars
        ),
        paragraph.start_offset + start,
        paragraph.start_offset + end,
        Some("拆分为更短的句子".to_string()),
    ));
}

fn collect_long_paragraph_issues(
    document: &DocumentText,
    paragraph: &ParagraphSlice,
    rule: &ProofreadingRuleDto,
    issues: &mut Vec<ProofreadingIssueDto>,
) {
    let config = parse_config(&rule.config_json);
    let max_chars = config_i64(&config, "maxCharacters", 500) as usize;
    let actual_len = paragraph
        .text
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .count();

    if actual_len > max_chars {
        issues.push(issue(
            &document.document_id,
            &paragraph.paragraph_id,
            rule,
            format!(
                "段落长度约 {} 字，超过当前阈值 {} 字。",
                actual_len, max_chars
            ),
            paragraph.start_offset,
            paragraph.start_offset + paragraph.text.chars().count(),
            Some("拆成多个段落".to_string()),
        ));
    }
}

fn collect_sensitive_word_issues(
    document: &DocumentText,
    paragraph: &ParagraphSlice,
    rule: &ProofreadingRuleDto,
    issues: &mut Vec<ProofreadingIssueDto>,
) {
    let config = parse_config(&rule.config_json);
    let mut words = config_string_list(&config, "words");

    if let Some(pattern) = rule.pattern.as_deref() {
        words.extend(
            pattern
                .split([',', '，', '\n', ';', '；'])
                .map(|word| word.trim().to_string())
                .filter(|word| !word.is_empty()),
        );
    }

    words.sort();
    words.dedup();

    for word in words {
        let regex = match RegexBuilder::new(&regex::escape(&word))
            .case_insensitive(true)
            .build()
        {
            Ok(regex) => regex,
            Err(_) => continue,
        };

        for found in regex.find_iter(&paragraph.text) {
            let start =
                paragraph.start_offset + byte_to_char_offset(&paragraph.text, found.start());
            let end = paragraph.start_offset + byte_to_char_offset(&paragraph.text, found.end());
            issues.push(issue(
                &document.document_id,
                &paragraph.paragraph_id,
                rule,
                format!("发现敏感词“{}”，建议替换或确认语境。", found.as_str()),
                start,
                end,
                Some("替换为更准确的表达".to_string()),
            ));
        }
    }
}

fn collect_name_consistency_issues(
    document: &DocumentText,
    paragraph: &ParagraphSlice,
    rule: &ProofreadingRuleDto,
    character_names: &[CharacterName],
    issues: &mut Vec<ProofreadingIssueDto>,
) {
    if character_names.is_empty() {
        return;
    }

    let config = parse_config(&rule.config_json);
    let max_distance = config_i64(&config, "maxDistance", 1) as usize;
    let min_len = config_i64(&config, "minNameCharacters", 2) as usize;
    let chars: Vec<char> = paragraph.text.chars().collect();

    for known in character_names {
        let name_chars: Vec<char> = known.name.chars().collect();
        let len = name_chars.len();
        if len < min_len || len > 12 || chars.len() < len {
            continue;
        }

        for start in 0..=(chars.len() - len) {
            let window = &chars[start..start + len];
            if window == name_chars.as_slice() {
                continue;
            }
            let likely_same_name =
                window.first() == name_chars.first() || window.last() == name_chars.last();
            if !likely_same_name {
                continue;
            }
            let distance = edit_distance(window, &name_chars);
            if distance > 0 && distance <= max_distance {
                let typo: String = window.iter().collect();
                issues.push(issue(
                    &document.document_id,
                    &paragraph.paragraph_id,
                    rule,
                    format!(
                        "“{}”疑似是角色名“{}”的错写，请确认设定名称一致性。",
                        typo, known.name
                    ),
                    paragraph.start_offset + start,
                    paragraph.start_offset + start + len,
                    Some(known.name.clone()),
                ));
                break;
            }
        }
    }
}

fn edit_distance(left: &[char], right: &[char]) -> usize {
    let mut dp = vec![vec![0_usize; right.len() + 1]; left.len() + 1];
    for (i, row) in dp.iter_mut().enumerate().take(left.len() + 1) {
        row[0] = i;
    }
    for j in 0..=right.len() {
        dp[0][j] = j;
    }
    for i in 1..=left.len() {
        for j in 1..=right.len() {
            let cost = if left[i - 1] == right[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }
    dp[left.len()][right.len()]
}

fn collect_custom_regex_issues(
    document: &DocumentText,
    paragraph: &ParagraphSlice,
    rule: &ProofreadingRuleDto,
    issues: &mut Vec<ProofreadingIssueDto>,
) {
    let Some(pattern) = rule.pattern.as_deref() else {
        return;
    };

    let regex = match Regex::new(pattern) {
        Ok(regex) => regex,
        Err(error) => {
            issues.push(ProofreadingIssueDto {
                id: Uuid::new_v4().to_string(),
                rule_id: Some(rule.id.clone()),
                document_id: document.document_id.clone(),
                paragraph_id: Some(paragraph.paragraph_id.clone()),
                severity: "error".to_string(),
                message: format!("自定义正则无法编译：{}", error),
                start_offset: Some(paragraph.start_offset as i64),
                end_offset: Some(paragraph.start_offset as i64),
                suggestion: None,
            });
            return;
        }
    };

    let config = parse_config(&rule.config_json);
    let suggestion = config
        .get("suggestion")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string());
    let message_template = config
        .get("message")
        .and_then(|value| value.as_str())
        .unwrap_or("命中自定义正则规则");

    for found in regex.find_iter(&paragraph.text) {
        let start = paragraph.start_offset + byte_to_char_offset(&paragraph.text, found.start());
        let end = paragraph.start_offset + byte_to_char_offset(&paragraph.text, found.end());
        issues.push(issue(
            &document.document_id,
            &paragraph.paragraph_id,
            rule,
            format!("{}：“{}”", message_template, found.as_str()),
            start,
            end,
            suggestion.clone(),
        ));
    }
}

fn byte_to_char_offset(text: &str, byte_index: usize) -> usize {
    text[..byte_index].chars().count()
}
