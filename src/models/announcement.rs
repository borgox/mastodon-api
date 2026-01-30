use crate::models::{CustomEmoji, Mention, Status, Tag};
use serde::{Deserialize, Serialize};

/// Represents an announcement from the server.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Announcement {
    /// The ID of the announcement in the database.
    pub id: String,
    /// The textual content of the announcement.
    pub content: String,
    /// When the announcement will start (ISO 8601).
    pub starts_at: Option<String>,
    /// When the announcement will end (ISO 8601).
    pub ends_at: Option<String>,
    /// Whether the announcement is a continuous event.
    pub all_day: bool,
    /// When the announcement was published (ISO 8601).
    pub published_at: String,
    /// When the announcement was last updated (ISO 8601).
    pub updated_at: String,
    /// Whether the announcement has been read by the current user.
    pub read: bool,
    /// Accounts mentioned in the announcement content.
    pub mentions: Vec<Mention>,
    /// Statuses linked in the announcement content.
    pub statuses: Vec<Status>,
    /// Tags linked in the announcement content.
    pub tags: Vec<Tag>,
    /// Custom emoji used in the announcement content.
    pub emojis: Vec<CustomEmoji>,
    /// Reactions to the announcement.
    pub reactions: Vec<AnnouncementReaction>,
}

/// Represents a reaction to an announcement.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnnouncementReaction {
    /// The name of the reaction (emoji).
    pub name: String,
    /// The number of times this reaction has been used.
    pub count: u64,
    /// Whether the current user has added this reaction.
    pub me: bool,
    /// A static URL to the emoji.
    pub static_url: Option<String>,
    /// A URL to the emoji.
    pub url: Option<String>,
}
