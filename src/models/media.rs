use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaAttachment {
    pub id: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub url: String,
    pub preview_url: Option<String>,
    pub remote_url: Option<String>,
    pub description: Option<String>,
    pub blurhash: Option<String>,
}
