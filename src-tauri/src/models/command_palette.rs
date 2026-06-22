use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandPaletteItemDto {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub target_type: String,
    pub target_id: Option<String>,
    pub shortcut: Option<String>,
}
