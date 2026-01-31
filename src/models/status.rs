use crate::models::Account;
use serde::{Deserialize, Serialize};

/// Represents a status (post) on Mastodon.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    /// The ID of the status.
    pub id: String,
    /// The time the status was created (ISO 8601).
    pub created_at: String,
    /// ID of the status being replied to, if any.
    pub in_reply_to_id: Option<String>,
    /// ID of the account being replied to, if any.
    pub in_reply_to_account_id: Option<String>,
    /// Whether the status is marked as sensitive (should be hidden).
    pub sensitive: bool,
    /// Text to be shown as a warning before the status content.
    pub spoiler_text: String,
    /// Visibility of the status ("public", "unlisted", "private", "direct").
    pub visibility: String,
    /// ISO 639 language code for the status.
    pub language: Option<String>,
    /// URI of the status used for federation.
    pub uri: String,
    /// Public URL of the status.
    pub url: Option<String>,
    /// Number of replies to the status.
    pub replies_count: u64,
    /// Number of reblogs (boosts) for the status.
    pub reblogs_count: u64,
    /// Number of favourites (likes) for the status.
    pub favourites_count: u64,
    /// HTML content of the status.
    pub content: String,
    /// The account that created the status.
    pub account: Account,
}

/// Represents the context of a status (ancestors and descendants).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Context {
    /// Ancestors in the conversation tree.
    pub ancestors: Vec<Status>,
    /// Descendants in the conversation tree.
    pub descendants: Vec<Status>,
}
