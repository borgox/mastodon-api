use serde::{Deserialize, Serialize};

/// Represents a user account on Mastodon.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    /// The local ID of the account.
    pub id: String,
    /// The username of the account, not including domain.
    pub username: String,
    /// The profile's display name.
    pub display_name: String,
    /// The unique identifier of the account, usually `username@domain`.
    /// For local users, it may just be the username.
    pub acct: String,
    /// The URL of the user's profile.
    pub url: String,
    /// The number of followers for the account.
    pub followers_count: u64,
    /// The number of accounts this user is following.
    pub following_count: u64,
    /// The number of statuses (posts) the user has created.
    pub statuses_count: u64,
    /// The user's bio (note).
    pub note: String,
    /// URL of the user's avatar image.
    pub avatar: String,
    /// URL of the user's header image.
    pub header: String,
    /// Whether the account is locked and requires follow requests.
    pub locked: bool,
    /// Whether the account is a bot.
    pub bot: bool,
    /// The time the account was created (ISO 8601).
    pub created_at: String,
}
