use serde::{Deserialize, Serialize};

/// Represents the preferences of the authenticated user.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Preferences {
    /// Default visibility for new posts.
    #[serde(rename = "posting:default:visibility")]
    pub default_visibility: String,
    /// Whether new posts should be marked as sensitive by default.
    #[serde(rename = "posting:default:sensitive")]
    pub default_sensitive: bool,
    /// Default language for new posts.
    #[serde(rename = "posting:default:language")]
    pub default_language: Option<String>,
    /// Whether media should be expanded by default.
    #[serde(rename = "reading:expand:media")]
    pub expand_media: String,
    /// Whether content warnings (spoilers) should be expanded by default.
    #[serde(rename = "reading:expand:spoilers")]
    pub expand_spoilers: bool,
}
