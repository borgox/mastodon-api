use serde::{Deserialize, Serialize};

/// Represents a subscription to the Web Push API.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebPushSubscription {
    /// The ID of the Web Push subscription in the database.
    pub id: u64,
    /// The endpoint of the Web Push subscription.
    pub endpoint: String,
    /// The server key used for the Web Push subscription.
    pub server_key: String,
    /// Which alerts should be delivered to the push endpoint.
    pub alerts: WebPushAlerts,
}

/// Represents the alerts that can be delivered via Web Push.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebPushAlerts {
    /// Whether to notify on new followers.
    pub follow: bool,
    /// Whether to notify on new favourites.
    pub favourite: bool,
    /// Whether to notify on new reblogs.
    pub reblog: bool,
    /// Whether to notify on new mentions.
    pub mention: bool,
    /// Whether to notify on new poll results.
    pub poll: bool,
    /// Whether to notify on new statuses from followed accounts.
    pub status: bool,
}
