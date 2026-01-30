use serde::{Deserialize, Serialize};

/// Represents a tag that is featured on a user's profile.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeaturedTag {
    /// The internal ID of the featured tag.
    pub id: String,
    /// The name of the tag.
    pub name: String,
    /// The URL to the tag's page on the instance.
    pub url: String,
    /// The number of statuses posted with this tag.
    pub statuses_count: u64,
    /// The time of the last status posted with this tag (ISO 8601).
    pub last_status_at: String,
}
