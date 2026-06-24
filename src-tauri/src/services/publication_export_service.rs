use std::{
    fs::{self, File},
    io::{Seek, Write},
    path::{Path, PathBuf},
};

use chrono::Utc;
use regex::Regex;
use serde_json::Value;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use crate::{
    errors::{AppError, AppResult},
    models::export_template::ExportTemplateConfig,
};

#[derive(Debug, Clone)]
pub(crate) struct PublicationProject {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub language: String,
    pub cover_asset_id: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct PublicationDocumentSource {
    pub id: String,
    pub document_type: String,
    pub title: String,
    pub depth: usize,
    pub content_json: String,
    pub content_text: String,
}

#[derive(Debug, Clone)]
pub(crate) struct PublicationAsset {
    pub id: String,
    pub filename: String,
    pub mime: Option<String>,
    pub path: String,
}

#[derive(Debug, Clone, Default)]
struct InlineSpan {
    text: String,
    bold: bool,
    italic: bool,
    code: bool,
    emphasis: bool,
    ruby: Option<String>,
}

#[derive(Debug, Clone)]
enum PublicationBlock {
    Paragraph(Vec<InlineSpan>),
    Heading {
        level: u8,
        spans: Vec<InlineSpan>,
    },
    Quote(Vec<PublicationBlock>),
    List {
        ordered: bool,
        items: Vec<Vec<PublicationBlock>>,
    },
    Code(String),
    Rule,
}

#[derive(Debug, Clone)]
struct PublicationDocument {
    id: String,
    document_type: String,
    title: String,
    depth: usize,
    blocks: Vec<PublicationBlock>,
}

#[derive(Debug, Clone)]
struct PackedAsset {
    id: String,
    href: String,
    mime: String,
    bytes: Vec<u8>,
    is_cover: bool,
}

pub(crate) fn write_docx(
    path: &Path,
    project: &PublicationProject,
    sources: &[PublicationDocumentSource],
    config: &ExportTemplateConfig,
) -> AppResult<()> {
    let documents = build_documents(sources);
    let file = File::create(path).map_err(|error| {
        AppError::with_detail(
            "DOCX_CREATE_FAILED",
            "无法创建 DOCX 文件",
            error.to_string(),
        )
    })?;
    let mut zip = ZipWriter::new(file);

    zip_text(
        &mut zip,
        "[Content_Types].xml",
        content_types_xml(),
        CompressionMethod::Deflated,
    )?;
    zip_text(
        &mut zip,
        "_rels/.rels",
        package_rels_xml(),
        CompressionMethod::Deflated,
    )?;
    zip_text(
        &mut zip,
        "docProps/core.xml",
        core_props_xml(project),
        CompressionMethod::Deflated,
    )?;
    zip_text(
        &mut zip,
        "docProps/app.xml",
        app_props_xml(),
        CompressionMethod::Deflated,
    )?;
    zip_text(
        &mut zip,
        "word/document.xml",
        docx_document_xml(project, &documents, config),
        CompressionMethod::Deflated,
    )?;
    zip_text(
        &mut zip,
        "word/styles.xml",
        docx_styles_xml(config),
        CompressionMethod::Deflated,
    )?;
    zip_text(
        &mut zip,
        "word/numbering.xml",
        docx_numbering_xml(),
        CompressionMethod::Deflated,
    )?;
    zip_text(
        &mut zip,
        "word/settings.xml",
        docx_settings_xml(),
        CompressionMethod::Deflated,
    )?;
    zip_text(
        &mut zip,
        "word/_rels/document.xml.rels",
        document_rels_xml(),
        CompressionMethod::Deflated,
    )?;

    zip.finish().map_err(|error| {
        AppError::with_detail("DOCX_PACK_FAILED", "打包 DOCX 文件失败", error.to_string())
    })?;
    Ok(())
}

pub(crate) fn write_epub(
    path: &Path,
    database_path: &Path,
    project: &PublicationProject,
    sources: &[PublicationDocumentSource],
    assets: &[PublicationAsset],
    config: &ExportTemplateConfig,
) -> AppResult<()> {
    let documents = build_documents(sources);
    let packed_assets = load_epub_assets(database_path, project, assets, config);
    let file = File::create(path).map_err(|error| {
        AppError::with_detail(
            "EPUB_CREATE_FAILED",
            "无法创建 EPUB 文件",
            error.to_string(),
        )
    })?;
    let mut zip = ZipWriter::new(file);

    zip_text(
        &mut zip,
        "mimetype",
        "application/epub+zip".to_string(),
        CompressionMethod::Stored,
    )?;
    zip_text(
        &mut zip,
        "META-INF/container.xml",
        epub_container_xml(),
        CompressionMethod::Deflated,
    )?;
    zip_text(
        &mut zip,
        "OEBPS/styles.css",
        epub_styles_css(config),
        CompressionMethod::Deflated,
    )?;

    let mut spine_ids = Vec::new();
    if packed_assets.iter().any(|asset| asset.is_cover) {
        zip_text(
            &mut zip,
            "OEBPS/cover.xhtml",
            epub_cover_xhtml(project, &packed_assets),
            CompressionMethod::Deflated,
        )?;
        spine_ids.push("cover".to_string());
    }

    for (index, document) in documents.iter().enumerate() {
        let href = format!("OEBPS/chapter-{}.xhtml", index + 1);
        zip_text(
            &mut zip,
            &href,
            epub_chapter_xhtml(project, document, index, config),
            CompressionMethod::Deflated,
        )?;
        spine_ids.push(format!("chapter-{}", index + 1));
    }

    zip_text(
        &mut zip,
        "OEBPS/nav.xhtml",
        epub_nav_xhtml(project, &documents, config),
        CompressionMethod::Deflated,
    )?;
    zip_text(
        &mut zip,
        "OEBPS/content.opf",
        epub_package_opf(project, &documents, &packed_assets, &spine_ids, config),
        CompressionMethod::Deflated,
    )?;

    for asset in &packed_assets {
        zip_bytes(
            &mut zip,
            &format!("OEBPS/{}", asset.href),
            &asset.bytes,
            CompressionMethod::Deflated,
        )?;
    }

    zip.finish().map_err(|error| {
        AppError::with_detail("EPUB_PACK_FAILED", "打包 EPUB 文件失败", error.to_string())
    })?;
    Ok(())
}

pub(crate) fn render_pixiv(
    project: &PublicationProject,
    sources: &[PublicationDocumentSource],
    config: &ExportTemplateConfig,
) -> String {
    let documents = build_documents(sources);
    let mut sections = Vec::new();

    if !config.pixiv_tags.is_empty() {
        let tags = config
            .pixiv_tags
            .iter()
            .map(|tag| tag.trim().trim_start_matches('#'))
            .filter(|tag| !tag.is_empty())
            .map(|tag| format!("#{}", tag))
            .collect::<Vec<_>>();
        if !tags.is_empty() {
            sections.push(tags.join(" "));
        }
    }

    if config.include_title {
        sections.push(project.title.clone());
    }
    if config.include_author {
        if let Some(author) = project
            .author
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            sections.push(format!("作者：{}", author.trim()));
        }
    }

    let mut document_sections = Vec::new();
    for (index, document) in documents.iter().enumerate() {
        let mut output = String::new();
        if config.include_chapter_title {
            output.push_str("[chapter:");
            output.push_str(&sanitize_pixiv_tag_text(&format_document_title(
                document,
                index,
                &config.chapter_title_format,
            )));
            output.push_str("]\n\n");
        }

        let body = render_pixiv_blocks(&document.blocks, config);
        output.push_str(body.trim());
        document_sections.push(output.trim().to_string());
    }

    if !document_sections.is_empty() {
        let separator = if config.pixiv_page_break {
            "\n\n[newpage]\n\n"
        } else {
            "\n\n"
        };
        sections.push(document_sections.join(separator));
    }

    let mut output = sections
        .into_iter()
        .filter(|section| !section.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n\n");
    output = convert_pixiv_notation(&output, config);
    if !output.ends_with('\n') {
        output.push('\n');
    }
    output
}

pub(crate) fn render_publication_preview_html(
    project: &PublicationProject,
    sources: &[PublicationDocumentSource],
    config: &ExportTemplateConfig,
    format: &str,
) -> String {
    let documents = build_documents(sources);
    let mut out = String::new();
    out.push_str("<!doctype html><html lang=\"");
    out.push_str(&escape_xml(&project.language));
    out.push_str("\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>");
    out.push_str(&escape_xml(&project.title));
    out.push_str("</title><style>");
    out.push_str(&publication_css(config));
    out.push_str(".preview-format{position:sticky;top:0;background:#fff8e7;border:1px solid #e5d6b8;border-radius:999px;display:inline-block;padding:.25rem .65rem;font:700 12px system-ui;color:#7a6040}.page-break{border-top:1px dashed #b7a88f;margin:2.5rem 0;padding-top:1rem}");
    out.push_str("</style></head><body><span class=\"preview-format\">");
    out.push_str(&escape_xml(&format!("{} 预览", format.to_uppercase())));
    out.push_str("</span>");

    if config.include_title {
        out.push_str("<header class=\"book-header\"><h1>");
        out.push_str(&escape_xml(&project.title));
        out.push_str("</h1>");
        if config.include_author {
            if let Some(author) = project
                .author
                .as_deref()
                .filter(|value| !value.trim().is_empty())
            {
                out.push_str("<p class=\"author\">作者：");
                out.push_str(&escape_xml(author.trim()));
                out.push_str("</p>");
            }
        }
        out.push_str("</header>");
    }

    for (index, document) in documents.iter().enumerate() {
        if index > 0 && config.document_page_break {
            out.push_str("<div class=\"page-break\">分页</div>");
        }
        out.push_str("<section>");
        if config.include_chapter_title {
            let level = document_heading_level(document);
            out.push_str(&format!("<h{}>", level));
            out.push_str(&escape_xml(&format_document_title(
                document,
                index,
                &config.chapter_title_format,
            )));
            out.push_str(&format!("</h{}>", level));
        }
        out.push_str(&render_blocks_xhtml(&document.blocks, config));
        out.push_str("</section>");
    }

    out.push_str("</body></html>");
    out
}

fn build_documents(sources: &[PublicationDocumentSource]) -> Vec<PublicationDocument> {
    sources
        .iter()
        .map(|source| {
            let blocks = serde_json::from_str::<Value>(&source.content_json)
                .ok()
                .map(|json| parse_blocks(json.get("content")))
                .filter(|blocks| !blocks.is_empty())
                .unwrap_or_else(|| plain_text_blocks(&source.content_text));
            PublicationDocument {
                id: source.id.clone(),
                document_type: source.document_type.clone(),
                title: source.title.clone(),
                depth: source.depth,
                blocks,
            }
        })
        .collect()
}

fn parse_blocks(content: Option<&Value>) -> Vec<PublicationBlock> {
    let Some(nodes) = content.and_then(Value::as_array) else {
        return Vec::new();
    };
    let mut blocks = Vec::new();
    for node in nodes {
        parse_block(node, &mut blocks);
    }
    blocks
}

fn parse_block(node: &Value, out: &mut Vec<PublicationBlock>) {
    let node_type = node.get("type").and_then(Value::as_str).unwrap_or("");
    match node_type {
        "paragraph" => out.push(PublicationBlock::Paragraph(parse_inlines(
            node.get("content"),
        ))),
        "heading" => {
            let level = node
                .get("attrs")
                .and_then(|attrs| attrs.get("level"))
                .and_then(Value::as_u64)
                .unwrap_or(2)
                .clamp(1, 6) as u8;
            out.push(PublicationBlock::Heading {
                level,
                spans: parse_inlines(node.get("content")),
            });
        }
        "blockquote" => out.push(PublicationBlock::Quote(parse_blocks(node.get("content")))),
        "bulletList" | "orderedList" => {
            let ordered = node_type == "orderedList";
            let items = node
                .get("content")
                .and_then(Value::as_array)
                .map(|nodes| {
                    nodes
                        .iter()
                        .filter(|item| item.get("type").and_then(Value::as_str) == Some("listItem"))
                        .map(|item| parse_blocks(item.get("content")))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            out.push(PublicationBlock::List { ordered, items });
        }
        "codeBlock" => out.push(PublicationBlock::Code(inline_plain_text(
            node.get("content"),
        ))),
        "horizontalRule" => out.push(PublicationBlock::Rule),
        "listItem" => out.extend(parse_blocks(node.get("content"))),
        _ => {
            let children = parse_blocks(node.get("content"));
            if children.is_empty() {
                let spans = parse_inlines(node.get("content"));
                if !spans.is_empty() {
                    out.push(PublicationBlock::Paragraph(spans));
                }
            } else {
                out.extend(children);
            }
        }
    }
}

fn parse_inlines(content: Option<&Value>) -> Vec<InlineSpan> {
    let Some(nodes) = content.and_then(Value::as_array) else {
        return Vec::new();
    };
    let mut spans = Vec::new();
    for node in nodes {
        let node_type = node.get("type").and_then(Value::as_str).unwrap_or("");
        if node_type == "text" {
            let mut span = InlineSpan {
                text: node
                    .get("text")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                ..InlineSpan::default()
            };
            if let Some(marks) = node.get("marks").and_then(Value::as_array) {
                for mark in marks {
                    let mark_type = mark.get("type").and_then(Value::as_str).unwrap_or("");
                    match mark_type {
                        "bold" | "strong" => span.bold = true,
                        "italic" | "em" => span.italic = true,
                        "code" => span.code = true,
                        "emphasis" | "emphasisMark" => span.emphasis = true,
                        "ruby" => {
                            span.ruby = mark
                                .get("attrs")
                                .and_then(|attrs| {
                                    attrs
                                        .get("reading")
                                        .or_else(|| attrs.get("ruby"))
                                        .or_else(|| attrs.get("annotation"))
                                })
                                .and_then(Value::as_str)
                                .map(ToString::to_string);
                        }
                        _ => {}
                    }
                }
            }
            if !span.text.is_empty() {
                spans.push(span);
            }
        } else if node_type == "hardBreak" {
            spans.push(InlineSpan {
                text: "\n".to_string(),
                ..InlineSpan::default()
            });
        } else {
            spans.extend(parse_inlines(node.get("content")));
        }
    }
    spans
}

fn inline_plain_text(content: Option<&Value>) -> String {
    parse_inlines(content)
        .into_iter()
        .map(|span| span.text)
        .collect::<Vec<_>>()
        .join("")
}

fn plain_text_blocks(text: &str) -> Vec<PublicationBlock> {
    text.split("\n\n")
        .map(str::trim)
        .filter(|paragraph| !paragraph.is_empty())
        .map(|paragraph| {
            PublicationBlock::Paragraph(vec![InlineSpan {
                text: paragraph.to_string(),
                ..InlineSpan::default()
            }])
        })
        .collect()
}

fn render_pixiv_blocks(blocks: &[PublicationBlock], config: &ExportTemplateConfig) -> String {
    let mut rendered = Vec::new();
    for block in blocks {
        match block {
            PublicationBlock::Paragraph(spans) => {
                rendered.push(render_pixiv_spans(spans, config));
            }
            PublicationBlock::Heading { spans, .. } => {
                rendered.push(format!(
                    "[chapter:{}]",
                    sanitize_pixiv_tag_text(&render_plain_spans(spans))
                ));
            }
            PublicationBlock::Quote(inner) => {
                let quote = render_pixiv_blocks(inner, config)
                    .lines()
                    .map(|line| format!("> {}", line))
                    .collect::<Vec<_>>()
                    .join("\n");
                rendered.push(quote);
            }
            PublicationBlock::List { ordered, items } => {
                let list = items
                    .iter()
                    .enumerate()
                    .map(|(index, item)| {
                        let prefix = if *ordered {
                            format!("{}. ", index + 1)
                        } else {
                            "・".to_string()
                        };
                        let content = render_pixiv_blocks(item, config).replace('\n', " ");
                        format!("{}{}", prefix, content.trim())
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                rendered.push(list);
            }
            PublicationBlock::Code(code) => rendered.push(code.clone()),
            PublicationBlock::Rule => rendered.push("――――".to_string()),
        }
    }
    rendered
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>()
        .join(&config.paragraph_separator)
}

fn render_pixiv_spans(spans: &[InlineSpan], config: &ExportTemplateConfig) -> String {
    spans
        .iter()
        .map(|span| {
            let mut text = sanitize_pixiv_inline_text(&span.text);
            if let Some(reading) = span
                .ruby
                .as_deref()
                .filter(|value| !value.trim().is_empty())
            {
                text = format!("[[rb:{} > {}]]", text, sanitize_pixiv_tag_text(reading));
            }
            if span.emphasis && config.pixiv_convert_emphasis {
                text = format!("[[emphasismark:{} > •]]", text);
            }
            if span.italic {
                text = format!("[i:{}]", text);
            }
            if span.bold {
                text = format!("[b:{}]", text);
            }
            text
        })
        .collect::<Vec<_>>()
        .join("")
}

fn convert_pixiv_notation(value: &str, config: &ExportTemplateConfig) -> String {
    let mut output = value.to_string();
    if config.pixiv_convert_ruby {
        if let Ok(regex) = Regex::new(r"｜([^《》\r\n]+)《([^《》\r\n]+)》") {
            output = regex.replace_all(&output, "[[rb:$1 > $2]]").to_string();
        }
    }
    if config.pixiv_convert_emphasis {
        if let Ok(regex) = Regex::new(r"《《([^《》\r\n]+)》》") {
            output = regex
                .replace_all(&output, "[[emphasismark:$1 > •]]")
                .to_string();
        }
    }
    output
}

fn sanitize_pixiv_tag_text(value: &str) -> String {
    value
        .replace('\r', " ")
        .replace('\n', " ")
        .replace(']', " ")
        .replace('[', " ")
        .trim()
        .to_string()
}

fn sanitize_pixiv_inline_text(value: &str) -> String {
    value.replace('\r', "")
}

fn render_plain_spans(spans: &[InlineSpan]) -> String {
    spans
        .iter()
        .map(|span| span.text.as_str())
        .collect::<Vec<_>>()
        .join("")
}

fn docx_document_xml(
    project: &PublicationProject,
    documents: &[PublicationDocument],
    config: &ExportTemplateConfig,
) -> String {
    let mut body = String::new();

    if config.include_title {
        body.push_str(&docx_paragraph(
            &[InlineSpan {
                text: project.title.clone(),
                bold: true,
                ..InlineSpan::default()
            }],
            Some("Title"),
            config,
            false,
            None,
        ));
    }
    if config.include_author {
        if let Some(author) = project
            .author
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            body.push_str(&docx_paragraph(
                &[InlineSpan {
                    text: format!("作者：{}", author.trim()),
                    ..InlineSpan::default()
                }],
                Some("Subtitle"),
                config,
                false,
                None,
            ));
        }
    }

    for (index, document) in documents.iter().enumerate() {
        if index > 0 && config.document_page_break {
            body.push_str("<w:p><w:r><w:br w:type=\"page\"/></w:r></w:p>");
        }
        if config.include_chapter_title {
            let style = format!("Heading{}", document_heading_level(document));
            body.push_str(&docx_paragraph(
                &[InlineSpan {
                    text: format_document_title(document, index, &config.chapter_title_format),
                    bold: true,
                    ..InlineSpan::default()
                }],
                Some(&style),
                config,
                false,
                None,
            ));
        }
        body.push_str(&docx_blocks_xml(&document.blocks, config, 0));
    }

    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>{}<w:sectPr><w:pgSz w:w="11906" w:h="16838"/><w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440" w:header="720" w:footer="720" w:gutter="0"/></w:sectPr></w:body>
</w:document>"#,
        body
    )
}

fn docx_blocks_xml(
    blocks: &[PublicationBlock],
    config: &ExportTemplateConfig,
    list_depth: usize,
) -> String {
    let mut out = String::new();
    for block in blocks {
        match block {
            PublicationBlock::Paragraph(spans) => {
                out.push_str(&docx_paragraph(spans, None, config, true, None));
            }
            PublicationBlock::Heading { level, spans } => {
                let style = format!("Heading{}", (*level).clamp(1, 6));
                out.push_str(&docx_paragraph(
                    spans,
                    Some(&style),
                    config,
                    false,
                    None,
                ));
            }
            PublicationBlock::Quote(inner) => {
                for child in inner {
                    match child {
                        PublicationBlock::Paragraph(spans) => out.push_str(&docx_paragraph(
                            spans,
                            Some("Quote"),
                            config,
                            false,
                            None,
                        )),
                        _ => out.push_str(&docx_blocks_xml(
                            std::slice::from_ref(child),
                            config,
                            list_depth,
                        )),
                    }
                }
            }
            PublicationBlock::List { ordered, items } => {
                for item in items {
                    let numbering = Some((if *ordered { 2 } else { 1 }, list_depth.min(1)));
                    let mut first = true;
                    for child in item {
                        match child {
                            PublicationBlock::Paragraph(spans) if first => {
                                out.push_str(&docx_paragraph(
                                    spans,
                                    None,
                                    config,
                                    false,
                                    numbering,
                                ));
                                first = false;
                            }
                            PublicationBlock::List { .. } => {
                                out.push_str(&docx_blocks_xml(
                                    std::slice::from_ref(child),
                                    config,
                                    list_depth + 1,
                                ));
                            }
                            _ => {
                                out.push_str(&docx_blocks_xml(
                                    std::slice::from_ref(child),
                                    config,
                                    list_depth,
                                ));
                                first = false;
                            }
                        }
                    }
                }
            }
            PublicationBlock::Code(code) => out.push_str(&docx_paragraph(
                &[InlineSpan {
                    text: code.clone(),
                    code: true,
                    ..InlineSpan::default()
                }],
                Some("Code"),
                config,
                false,
                None,
            )),
            PublicationBlock::Rule => out.push_str(
                "<w:p><w:pPr><w:pBdr><w:bottom w:val=\"single\" w:sz=\"6\" w:space=\"1\" w:color=\"999999\"/></w:pBdr></w:pPr></w:p>",
            ),
        }
    }
    out
}

fn docx_paragraph(
    spans: &[InlineSpan],
    style: Option<&str>,
    config: &ExportTemplateConfig,
    indent_first_line: bool,
    numbering: Option<(u32, usize)>,
) -> String {
    let line = (config.line_height * 240.0).round().clamp(240.0, 720.0) as i64;
    let mut properties = String::new();
    if let Some(style) = style {
        properties.push_str(&format!("<w:pStyle w:val=\"{}\"/>", escape_xml(style)));
    }
    properties.push_str(&format!(
        "<w:spacing w:after=\"120\" w:line=\"{}\" w:lineRule=\"auto\"/>",
        line
    ));
    if indent_first_line && config.first_line_indent {
        properties.push_str("<w:ind w:firstLine=\"420\"/>");
    }
    if let Some((num_id, level)) = numbering {
        properties.push_str(&format!(
            "<w:numPr><w:ilvl w:val=\"{}\"/><w:numId w:val=\"{}\"/></w:numPr>",
            level, num_id
        ));
    }

    let runs = if spans.is_empty() {
        "<w:r><w:t></w:t></w:r>".to_string()
    } else {
        spans
            .iter()
            .map(|span| docx_run_xml(span, config))
            .collect::<Vec<_>>()
            .join("")
    };

    format!("<w:p><w:pPr>{}</w:pPr>{}</w:p>", properties, runs)
}

fn docx_run_xml(span: &InlineSpan, config: &ExportTemplateConfig) -> String {
    let size = (config.font_size * 2.0).round().clamp(16.0, 144.0) as i64;
    let font = docx_font_name(&config.font_family);
    let mut props = format!(
        "<w:rFonts w:ascii=\"{}\" w:hAnsi=\"{}\" w:eastAsia=\"{}\"/><w:sz w:val=\"{}\"/><w:szCs w:val=\"{}\"/>",
        escape_xml(font),
        escape_xml(font),
        escape_xml(font),
        size,
        size
    );
    if span.bold {
        props.push_str("<w:b/><w:bCs/>");
    }
    if span.italic {
        props.push_str("<w:i/><w:iCs/>");
    }
    if span.code {
        props.push_str("<w:shd w:val=\"clear\" w:fill=\"F1F1F1\"/>");
    }
    if span.emphasis {
        props.push_str("<w:em w:val=\"dot\"/>");
    }

    let text = if let Some(reading) = span
        .ruby
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        format!("{}（{}）", span.text, reading)
    } else {
        span.text.clone()
    };

    if text == "\n" {
        return format!("<w:r><w:rPr>{}</w:rPr><w:br/></w:r>", props);
    }

    format!(
        "<w:r><w:rPr>{}</w:rPr><w:t xml:space=\"preserve\">{}</w:t></w:r>",
        props,
        escape_xml(&text)
    )
}

fn docx_font_name(font_family: &str) -> &'static str {
    match font_family {
        "sans-serif" => "Microsoft YaHei",
        "monospace" => "Consolas",
        "system" => "Microsoft YaHei",
        _ => "SimSun",
    }
}

fn docx_styles_xml(config: &ExportTemplateConfig) -> String {
    let normal_size = (config.font_size * 2.0).round().clamp(16.0, 144.0) as i64;
    let font = escape_xml(docx_font_name(&config.font_family));
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:docDefaults><w:rPrDefault><w:rPr><w:rFonts w:ascii="{font}" w:hAnsi="{font}" w:eastAsia="{font}"/><w:sz w:val="{normal_size}"/><w:szCs w:val="{normal_size}"/></w:rPr></w:rPrDefault></w:docDefaults>
<w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/><w:qFormat/></w:style>
<w:style w:type="paragraph" w:styleId="Title"><w:name w:val="Title"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/><w:pPr><w:jc w:val="center"/><w:spacing w:before="240" w:after="240"/></w:pPr><w:rPr><w:b/><w:sz w:val="40"/><w:szCs w:val="40"/></w:rPr></w:style>
<w:style w:type="paragraph" w:styleId="Subtitle"><w:name w:val="Subtitle"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/><w:pPr><w:jc w:val="center"/><w:spacing w:after="360"/></w:pPr><w:rPr><w:i/><w:color w:val="666666"/></w:rPr></w:style>
<w:style w:type="paragraph" w:styleId="Heading1"><w:name w:val="heading 1"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/><w:pPr><w:keepNext/><w:spacing w:before="360" w:after="180"/><w:outlineLvl w:val="0"/></w:pPr><w:rPr><w:b/><w:sz w:val="32"/><w:szCs w:val="32"/></w:rPr></w:style>
<w:style w:type="paragraph" w:styleId="Heading2"><w:name w:val="heading 2"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/><w:pPr><w:keepNext/><w:spacing w:before="300" w:after="160"/><w:outlineLvl w:val="1"/></w:pPr><w:rPr><w:b/><w:sz w:val="28"/><w:szCs w:val="28"/></w:rPr></w:style>
<w:style w:type="paragraph" w:styleId="Heading3"><w:name w:val="heading 3"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/><w:pPr><w:keepNext/><w:spacing w:before="260" w:after="140"/><w:outlineLvl w:val="2"/></w:pPr><w:rPr><w:b/><w:sz w:val="26"/><w:szCs w:val="26"/></w:rPr></w:style>
<w:style w:type="paragraph" w:styleId="Heading4"><w:name w:val="heading 4"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/><w:pPr><w:keepNext/><w:outlineLvl w:val="3"/></w:pPr><w:rPr><w:b/></w:rPr></w:style>
<w:style w:type="paragraph" w:styleId="Heading5"><w:name w:val="heading 5"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/><w:pPr><w:keepNext/><w:outlineLvl w:val="4"/></w:pPr><w:rPr><w:b/></w:rPr></w:style>
<w:style w:type="paragraph" w:styleId="Heading6"><w:name w:val="heading 6"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/><w:pPr><w:keepNext/><w:outlineLvl w:val="5"/></w:pPr><w:rPr><w:b/></w:rPr></w:style>
<w:style w:type="paragraph" w:styleId="Quote"><w:name w:val="Quote"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/><w:pPr><w:ind w:left="720" w:right="720"/><w:spacing w:before="120" w:after="120"/></w:pPr><w:rPr><w:i/><w:color w:val="555555"/></w:rPr></w:style>
<w:style w:type="paragraph" w:styleId="Code"><w:name w:val="Code"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/><w:pPr><w:ind w:left="360"/><w:spacing w:before="120" w:after="120"/></w:pPr><w:rPr><w:rFonts w:ascii="Consolas" w:hAnsi="Consolas" w:eastAsia="Microsoft YaHei"/></w:rPr></w:style>
</w:styles>"#
    )
}

fn docx_numbering_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:abstractNum w:abstractNumId="0"><w:multiLevelType w:val="hybridMultilevel"/>
<w:lvl w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="bullet"/><w:lvlText w:val="•"/><w:lvlJc w:val="left"/><w:pPr><w:tabs><w:tab w:val="num" w:pos="720"/></w:tabs><w:ind w:left="720" w:hanging="360"/></w:pPr></w:lvl>
<w:lvl w:ilvl="1"><w:start w:val="1"/><w:numFmt w:val="bullet"/><w:lvlText w:val="◦"/><w:lvlJc w:val="left"/><w:pPr><w:tabs><w:tab w:val="num" w:pos="1080"/></w:tabs><w:ind w:left="1080" w:hanging="360"/></w:pPr></w:lvl>
</w:abstractNum>
<w:abstractNum w:abstractNumId="1"><w:multiLevelType w:val="multilevel"/>
<w:lvl w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1."/><w:lvlJc w:val="left"/><w:pPr><w:tabs><w:tab w:val="num" w:pos="720"/></w:tabs><w:ind w:left="720" w:hanging="360"/></w:pPr></w:lvl>
<w:lvl w:ilvl="1"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1.%2."/><w:lvlJc w:val="left"/><w:pPr><w:tabs><w:tab w:val="num" w:pos="1080"/></w:tabs><w:ind w:left="1080" w:hanging="360"/></w:pPr></w:lvl>
</w:abstractNum>
<w:num w:numId="1"><w:abstractNumId w:val="0"/></w:num>
<w:num w:numId="2"><w:abstractNumId w:val="1"/></w:num>
</w:numbering>"#.to_string()
}

fn content_types_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
<Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/>
<Override PartName="/word/numbering.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"/>
<Override PartName="/word/settings.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"/>
<Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
<Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
</Types>"#.to_string()
}

fn package_rels_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
<Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>"#.to_string()
}

fn document_rels_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>
<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering" Target="numbering.xml"/>
<Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings" Target="settings.xml"/>
</Relationships>"#.to_string()
}

fn core_props_xml(project: &PublicationProject) -> String {
    let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<dc:title>{}</dc:title><dc:creator>{}</dc:creator><cp:lastModifiedBy>Mythistorima</cp:lastModifiedBy><dcterms:created xsi:type="dcterms:W3CDTF">{}</dcterms:created><dcterms:modified xsi:type="dcterms:W3CDTF">{}</dcterms:modified>
</cp:coreProperties>"#,
        escape_xml(&project.title),
        escape_xml(project.author.as_deref().unwrap_or("")),
        now,
        now
    )
}

fn app_props_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes"><Application>Mythistorima</Application><DocSecurity>0</DocSecurity><ScaleCrop>false</ScaleCrop><Company></Company><LinksUpToDate>false</LinksUpToDate><SharedDoc>false</SharedDoc><HyperlinksChanged>false</HyperlinksChanged><AppVersion>1.0</AppVersion></Properties>"#.to_string()
}

fn docx_settings_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:settings xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:zoom w:percent="100"/><w:defaultTabStop w:val="720"/><w:characterSpacingControl w:val="doNotCompress"/></w:settings>"#.to_string()
}

fn epub_container_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container"><rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles></container>"#.to_string()
}

fn epub_package_opf(
    project: &PublicationProject,
    documents: &[PublicationDocument],
    assets: &[PackedAsset],
    spine_ids: &[String],
    config: &ExportTemplateConfig,
) -> String {
    let modified = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let publisher = if config.epub_publisher.trim().is_empty() {
        "Mythistorima"
    } else {
        config.epub_publisher.trim()
    };
    let mut manifest = String::from(
        "<item id=\"nav\" href=\"nav.xhtml\" media-type=\"application/xhtml+xml\" properties=\"nav\"/><item id=\"style\" href=\"styles.css\" media-type=\"text/css\"/>",
    );
    if assets.iter().any(|asset| asset.is_cover) {
        manifest.push_str(
            "<item id=\"cover\" href=\"cover.xhtml\" media-type=\"application/xhtml+xml\"/>",
        );
    }
    for (index, _) in documents.iter().enumerate() {
        manifest.push_str(&format!(
            "<item id=\"chapter-{}\" href=\"chapter-{}.xhtml\" media-type=\"application/xhtml+xml\"/>",
            index + 1,
            index + 1
        ));
    }
    for asset in assets {
        manifest.push_str(&format!(
            "<item id=\"asset-{}\" href=\"{}\" media-type=\"{}\"{} />",
            escape_xml(&asset.id),
            escape_xml(&asset.href),
            escape_xml(&asset.mime),
            if asset.is_cover {
                " properties=\"cover-image\""
            } else {
                ""
            }
        ));
    }

    let spine = spine_ids
        .iter()
        .map(|id| format!("<itemref idref=\"{}\"/>", escape_xml(id)))
        .collect::<Vec<_>>()
        .join("");

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="book-id" xml:lang="{language}">
<metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
<dc:identifier id="book-id">urn:uuid:{id}</dc:identifier>
<dc:title>{title}</dc:title>
<dc:language>{language}</dc:language>
<dc:creator>{author}</dc:creator>
<dc:publisher>{publisher}</dc:publisher>
<meta property="dcterms:modified">{modified}</meta>
</metadata>
<manifest>{manifest}</manifest>
<spine>{spine}</spine>
</package>"#,
        language = escape_xml(&project.language),
        id = escape_xml(&project.id),
        title = escape_xml(&project.title),
        author = escape_xml(project.author.as_deref().unwrap_or("")),
        publisher = escape_xml(publisher),
        modified = modified,
        manifest = manifest,
        spine = spine,
    )
}

fn epub_nav_xhtml(
    project: &PublicationProject,
    documents: &[PublicationDocument],
    config: &ExportTemplateConfig,
) -> String {
    let mut items = String::new();
    if config.epub_include_toc {
        for (index, document) in documents.iter().enumerate() {
            items.push_str("<li><a href=\"chapter-");
            items.push_str(&(index + 1).to_string());
            items.push_str(".xhtml\">");
            items.push_str(&escape_xml(&format_document_title(
                document,
                index,
                &config.chapter_title_format,
            )));
            items.push_str("</a></li>");
        }
    }
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" lang="{language}">
<head><meta charset="utf-8"/><title>{title} — 目录</title><link rel="stylesheet" type="text/css" href="styles.css"/></head>
<body><nav epub:type="toc" id="toc"><h1>目录</h1><ol>{items}</ol></nav></body></html>"#,
        language = escape_xml(&project.language),
        title = escape_xml(&project.title),
        items = items
    )
}

fn epub_chapter_xhtml(
    project: &PublicationProject,
    document: &PublicationDocument,
    index: usize,
    config: &ExportTemplateConfig,
) -> String {
    let heading = if config.include_chapter_title {
        let level = document_heading_level(document);
        format!(
            "<h{level}>{}</h{level}>",
            escape_xml(&format_document_title(
                document,
                index,
                &config.chapter_title_format,
            ))
        )
    } else {
        String::new()
    };
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" lang="{language}">
<head><meta charset="utf-8"/><title>{title}</title><link rel="stylesheet" type="text/css" href="styles.css"/></head>
<body><article id="document-{id}">{heading}{body}</article></body></html>"#,
        language = escape_xml(&project.language),
        title = escape_xml(&document.title),
        id = escape_xml(&document.id),
        heading = heading,
        body = render_blocks_xhtml(&document.blocks, config),
    )
}

fn epub_cover_xhtml(project: &PublicationProject, assets: &[PackedAsset]) -> String {
    let cover = assets.iter().find(|asset| asset.is_cover);
    let image = cover
        .map(|asset| {
            format!(
                "<img src=\"{}\" alt=\"{}\"/>",
                escape_xml(&asset.href),
                escape_xml(&project.title)
            )
        })
        .unwrap_or_default();
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" lang="{language}">
<head><meta charset="utf-8"/><title>{title}</title><link rel="stylesheet" type="text/css" href="styles.css"/></head>
<body><section epub:type="cover" class="cover">{image}</section></body></html>"#,
        language = escape_xml(&project.language),
        title = escape_xml(&project.title),
        image = image
    )
}

fn render_blocks_xhtml(blocks: &[PublicationBlock], config: &ExportTemplateConfig) -> String {
    let mut out = String::new();
    for block in blocks {
        match block {
            PublicationBlock::Paragraph(spans) => {
                out.push_str("<p>");
                out.push_str(&render_spans_xhtml(spans));
                out.push_str("</p>");
            }
            PublicationBlock::Heading { level, spans } => {
                let level = (*level).clamp(1, 6);
                out.push_str(&format!("<h{}>", level));
                out.push_str(&render_spans_xhtml(spans));
                out.push_str(&format!("</h{}>", level));
            }
            PublicationBlock::Quote(inner) => {
                out.push_str("<blockquote>");
                out.push_str(&render_blocks_xhtml(inner, config));
                out.push_str("</blockquote>");
            }
            PublicationBlock::List { ordered, items } => {
                let tag = if *ordered { "ol" } else { "ul" };
                out.push_str(&format!("<{}>", tag));
                for item in items {
                    out.push_str("<li>");
                    out.push_str(&render_blocks_xhtml(item, config));
                    out.push_str("</li>");
                }
                out.push_str(&format!("</{}>", tag));
            }
            PublicationBlock::Code(code) => {
                out.push_str("<pre><code>");
                out.push_str(&escape_xml(code));
                out.push_str("</code></pre>");
            }
            PublicationBlock::Rule => out.push_str("<hr/>"),
        }
    }
    out
}

fn render_spans_xhtml(spans: &[InlineSpan]) -> String {
    spans
        .iter()
        .map(|span| {
            if span.text == "\n" {
                return "<br/>".to_string();
            }
            let mut value =
                if let Some(reading) = span.ruby.as_deref().filter(|v| !v.trim().is_empty()) {
                    format!(
                        "<ruby>{}<rp>（</rp><rt>{}</rt><rp>）</rp></ruby>",
                        escape_xml(&span.text),
                        escape_xml(reading)
                    )
                } else {
                    escape_xml(&span.text)
                };
            if span.code {
                value = format!("<code>{}</code>", value);
            }
            if span.emphasis {
                value = format!("<em class=\"emphasis\">{}</em>", value);
            }
            if span.italic {
                value = format!("<em>{}</em>", value);
            }
            if span.bold {
                value = format!("<strong>{}</strong>", value);
            }
            value
        })
        .collect::<Vec<_>>()
        .join("")
}

fn epub_styles_css(config: &ExportTemplateConfig) -> String {
    publication_css(config)
}

fn publication_css(config: &ExportTemplateConfig) -> String {
    let indent = if config.first_line_indent { "2em" } else { "0" };
    format!(
        "html{{font-size:100%;}}body{{max-width:42em;margin:0 auto;padding:1.5em;color:#2f2a24;background:#fffdf8;font-family:{};font-size:{:.1}pt;line-height:{:.2};}}h1,h2,h3,h4,h5,h6{{line-height:1.3;page-break-after:avoid;}}p{{margin:.75em 0;text-indent:{};}}blockquote{{border-left:.25em solid #b7a88f;margin:1em 0;padding:.2em 1em;color:#665f57;}}ul,ol{{padding-left:2em;}}pre{{white-space:pre-wrap;background:#f2eee7;padding:1em;border-radius:.4em;}}code{{font-family:monospace;}}ruby rt{{font-size:.55em;}}.emphasis{{text-emphasis:filled dot;}}.cover{{display:flex;align-items:center;justify-content:center;min-height:90vh;}}.cover img{{max-width:100%;max-height:90vh;object-fit:contain;}}.book-header{{margin:2em 0 3em;text-align:center;}}.author{{color:#6f665e;text-indent:0;}}",
        css_font_family(&config.font_family),
        config.font_size,
        config.line_height,
        indent
    )
}

fn load_epub_assets(
    database_path: &Path,
    project: &PublicationProject,
    assets: &[PublicationAsset],
    config: &ExportTemplateConfig,
) -> Vec<PackedAsset> {
    if !config.epub_include_assets {
        return Vec::new();
    }
    let base = database_path.parent().unwrap_or_else(|| Path::new("."));
    assets
        .iter()
        .filter_map(|asset| {
            let path = PathBuf::from(&asset.path);
            let resolved = if path.is_absolute() {
                path
            } else {
                base.join(path)
            };
            let bytes = fs::read(resolved).ok()?;
            let mime = asset
                .mime
                .clone()
                .filter(|value| value.starts_with("image/"))
                .or_else(|| infer_image_mime(&asset.filename).map(ToString::to_string))?;
            let extension = extension_for_mime(&mime)
                .or_else(|| {
                    Path::new(&asset.filename)
                        .extension()
                        .and_then(|value| value.to_str())
                })
                .unwrap_or("bin");
            let filename = format!(
                "{}-{}.{}",
                sanitize_archive_name(&asset.id),
                sanitize_archive_name(
                    Path::new(&asset.filename)
                        .file_stem()
                        .and_then(|value| value.to_str())
                        .unwrap_or("asset")
                ),
                sanitize_archive_name(extension)
            );
            Some(PackedAsset {
                id: asset.id.clone(),
                href: format!("assets/{}", filename),
                mime,
                bytes,
                is_cover: project.cover_asset_id.as_deref() == Some(asset.id.as_str()),
            })
        })
        .collect()
}

fn infer_image_mime(filename: &str) -> Option<&'static str> {
    match Path::new(filename)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "jpg" | "jpeg" => Some("image/jpeg"),
        "png" => Some("image/png"),
        "gif" => Some("image/gif"),
        "svg" => Some("image/svg+xml"),
        "webp" => Some("image/webp"),
        _ => None,
    }
}

fn extension_for_mime(mime: &str) -> Option<&'static str> {
    match mime {
        "image/jpeg" => Some("jpg"),
        "image/png" => Some("png"),
        "image/gif" => Some("gif"),
        "image/svg+xml" => Some("svg"),
        "image/webp" => Some("webp"),
        _ => None,
    }
}

fn format_document_title(document: &PublicationDocument, index: usize, pattern: &str) -> String {
    pattern
        .replace("{index}", &(index + 1).to_string())
        .replace("{title}", &document.title)
        .replace("{type}", document_type_label(&document.document_type))
        .replace("{depth}", &document.depth.to_string())
}

fn document_heading_level(document: &PublicationDocument) -> u8 {
    match document.document_type.as_str() {
        "volume" => 1,
        "chapter" => 2,
        "scene" => 3,
        _ => (document.depth + 2).clamp(1, 6) as u8,
    }
}

fn document_type_label(document_type: &str) -> &'static str {
    match document_type {
        "volume" => "卷",
        "chapter" => "章",
        "scene" => "场景",
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

fn sanitize_archive_name(value: &str) -> String {
    let output: String = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                ch
            } else {
                '_'
            }
        })
        .collect();
    if output.is_empty() {
        "asset".to_string()
    } else {
        output
    }
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn zip_text<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    path: &str,
    content: String,
    method: CompressionMethod,
) -> AppResult<()> {
    zip_bytes(zip, path, content.as_bytes(), method)
}

fn zip_bytes<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    path: &str,
    content: &[u8],
    method: CompressionMethod,
) -> AppResult<()> {
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o644);
    zip.start_file(path, options).map_err(|error| {
        AppError::with_detail(
            "ARCHIVE_ENTRY_FAILED",
            format!("无法写入归档条目 {}", path),
            error.to_string(),
        )
    })?;
    zip.write_all(content).map_err(|error| {
        AppError::with_detail(
            "ARCHIVE_WRITE_FAILED",
            format!("无法写入归档内容 {}", path),
            error.to_string(),
        )
    })?;
    Ok(())
}
