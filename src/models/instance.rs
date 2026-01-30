use serde::{Deserialize, Serialize};

/// Represents weekly usage statistics for an instance.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Activity {
    /// The start of the week (Unix timestamp).
    pub week: String,
    /// The number of statuses published during the week.
    pub statuses: String,
    /// The number of logins during the week.
    pub logins: String,
    /// The number of new registrations during the week.
    pub registrations: String,
}

/// Represents a formal rule established by the server.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rule {
    /// The ID of the rule.
    pub id: String,
    /// The text of the rule.
    pub text: String,
}

/// Metadata about a Mastodon instance.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Instance {
    /// The domain name of the instance.
    pub uri: String,
    /// The title of the instance.
    pub title: String,
    /// A shorter description of the instance.
    pub description: String,
    /// The administrative contact email for the instance.
    pub email: String,
    /// The version of Mastodon installed on the instance.
    pub version: String,
}
