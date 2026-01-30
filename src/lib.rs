//! # Mastodon API Wrapper
//!
//! A comprehensive, asynchronous Rust wrapper for the Mastodon API, designed for advanced bot development.
//!
//! ## Core Concepts
//!
//! ### 1. Timelines
//! A **Timeline** is a chronological list of statuses (posts).
//! - **Public Timeline**: Every public post known to your instance ("The Federated Timeline").
//! - **Local Timeline**: Public posts from users specifically on your instance.
//! - **Home Timeline**: Personalized feed from accounts you follow.
//!
//! ### 2. Statuses
//! A **Status** is the technical name for a post. It contains text, media attachments, mentions, etc.
//! Statuses can be regular posts, replies, or reblogs (boosts).
//!
//! ### 3. Streaming
//! **Streaming** provides real-time updates via WebSockets. Instead of polling, the server
//! "pushes" events (new posts, mentions) to you as they happen.
//!
//! ### 4. Accounts
//! Represents a user. Identified by a local `id` and a unique `acct` string (`user@domain`).
//!
//! ### 5. Media Attachments
//! Media must be uploaded separately to get an ID before being attached to a new status.
//!
//! # Example
//! ```no_run
//! use mastodon_api::MastodonClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = MastodonClient::new("https://mastodon.social").with_token("your_token");
//!     client.statuses().create_simple("Hello from Rust!").await?;
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod methods;
pub mod models;
pub mod paging;
pub mod streaming;

pub use error::{MastodonError, Result};
pub use models::{
    Account, Announcement, AnnouncementReaction, FeaturedTag, Marker, Preferences, Relationship,
    Report, Status, Suggestion, Tag, WebPushAlerts, WebPushSubscription,
};

use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

/// The main entry point for interacting with the Mastodon API.
///
/// Use `MastodonClient::new` to create a new instance, and `with_token` to authenticate.
#[derive(Clone)]
pub struct MastodonClient {
    base_url: String,
    client: Client,
    access_token: Option<String>,
}

impl MastodonClient {
    /// Creates a new `MastodonClient` for the given instance URL.
    ///
    /// # Example
    /// ```
    /// use mastodon_api::MastodonClient;
    /// let client = MastodonClient::new("https://mastodon.social");
    /// ```
    pub fn new(instance_url: &str) -> Self {
        Self {
            base_url: instance_url.trim_end_matches('/').to_string(),
            client: Client::new(),
            access_token: None,
        }
    }

    /// Sets the access token for the client, enabling authenticated requests.
    pub fn with_token(mut self, token: &str) -> Self {
        self.access_token = Some(token.to_string());
        self
    }

    /// Returns the access token used by the client, if any.
    pub fn access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }

    /// Returns the base URL of the Mastodon instance.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns a reference to the underlying HTTP client.
    pub fn http_client(&self) -> &Client {
        &self.client
    }

    pub(crate) async fn send<T: DeserializeOwned>(&self, builder: RequestBuilder) -> Result<T> {
        let mut retries = 0;
        let max_retries = 3;

        loop {
            let mut current_builder = builder.try_clone().ok_or(MastodonError::ApiError {
                status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to clone request for retry".to_string(),
            })?;

            if let Some(token) = &self.access_token {
                current_builder = current_builder.bearer_auth(token);
            }

            let response = current_builder.send().await?;
            let status = response.status();

            // Handle Rate Limiting
            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                if retries < max_retries {
                    if let Some(reset) = response.headers().get("X-RateLimit-Reset") {
                        if let Ok(_) = reset.to_str() {
                            let wait_secs = 2u64.pow(retries);
                            tokio::time::sleep(std::time::Duration::from_secs(wait_secs)).await;
                            retries += 1;
                            continue;
                        }
                    }
                }
            }

            // Handle transient server errors (5xx)
            if status.is_server_error() && retries < max_retries {
                let wait_secs = 2u64.pow(retries);
                tokio::time::sleep(std::time::Duration::from_secs(wait_secs)).await;
                retries += 1;
                continue;
            }

            if !status.is_success() {
                let message = response.text().await.unwrap_or_default();
                return Err(MastodonError::ApiError { status, message });
            }

            return Ok(response.json().await?);
        }
    }

    /// Access account-related endpoints.
    pub fn accounts(&self) -> methods::accounts::AccountsHandler<'_> {
        methods::accounts::AccountsHandler::new(self)
    }

    /// Access follow request-related endpoints.
    pub fn follow_requests(&self) -> methods::follow_requests::FollowRequestsHandler<'_> {
        methods::follow_requests::FollowRequestsHandler::new(self)
    }

    /// Access server-wide announcements.
    pub fn announcements(&self) -> methods::announcements::AnnouncementsHandler<'_> {
        methods::announcements::AnnouncementsHandler::new(self)
    }

    /// Access status-related (posting) endpoints.
    pub fn statuses(&self) -> methods::statuses::StatusesHandler<'_> {
        methods::statuses::StatusesHandler::new(self)
    }

    /// Access timeline-related endpoints (Home, Public, etc.).
    pub fn timelines(&self) -> methods::timelines::TimelinesHandler<'_> {
        methods::timelines::TimelinesHandler::new(self)
    }

    /// Access application registration and OAuth endpoints.
    pub fn apps(&self) -> methods::apps::AppsHandler<'_> {
        methods::apps::AppsHandler::new(self)
    }

    /// Access media upload and management endpoints.
    pub fn media(&self) -> methods::media::MediaHandler<'_> {
        methods::media::MediaHandler::new(self)
    }

    /// Access content filter management endpoints.
    pub fn filters(&self) -> methods::filters::FiltersHandler<'_> {
        methods::filters::FiltersHandler::new(self)
    }

    /// Access tag management endpoints (followed and featured tags).
    pub fn tags(&self) -> methods::tags::TagsHandler<'_> {
        methods::tags::TagsHandler::new(self)
    }

    /// Access reading position sync (markers).
    pub fn markers(&self) -> methods::markers::MarkersHandler<'_> {
        methods::markers::MarkersHandler::new(self)
    }

    /// Access reports made by the authenticated user.
    pub fn reports(&self) -> methods::reports::ReportsHandler<'_> {
        methods::reports::ReportsHandler::new(self)
    }

    /// Access accounts endorsed (pinned) by the authenticated user.
    pub fn endorsements(&self) -> methods::endorsements::EndorsementsHandler<'_> {
        methods::endorsements::EndorsementsHandler::new(self)
    }

    /// Access domain blocks for the authenticated user.
    pub fn domain_blocks(&self) -> methods::domain_blocks::DomainBlocksHandler<'_> {
        methods::domain_blocks::DomainBlocksHandler::new(self)
    }

    /// Access user preferences.
    pub fn preferences(&self) -> methods::preferences::PreferencesHandler<'_> {
        methods::preferences::PreferencesHandler::new(self)
    }

    /// Access Web Push API subscriptions.
    pub fn push(&self) -> methods::push::PushHandler<'_> {
        methods::push::PushHandler::new(self)
    }

    /// Access administrative moderation and management endpoints.
    pub fn admin(&self) -> methods::admin::AdminHandler<'_> {
        methods::admin::AdminHandler::new(self)
    }

    /// Access list management endpoints.
    pub fn lists(&self) -> methods::lists::ListsHandler<'_> {
        methods::lists::ListsHandler::new(self)
    }

    /// Access direct conversation (DM) endpoints.
    pub fn conversations(&self) -> methods::conversations::ConversationsHandler<'_> {
        methods::conversations::ConversationsHandler::new(self)
    }

    /// Access notification management endpoints.
    pub fn notifications(&self) -> methods::notifications::NotificationsHandler<'_> {
        methods::notifications::NotificationsHandler::new(self)
    }

    /// Access global search endpoints.
    pub fn search(&self) -> methods::search::SearchHandler<'_> {
        methods::search::SearchHandler::new(self)
    }

    /// Access account suggestions for follow.
    pub fn suggestions(&self) -> methods::suggestions::SuggestionsHandler<'_> {
        methods::suggestions::SuggestionsHandler::new(self)
    }

    /// Access trending content endpoints.
    pub fn trends(&self) -> methods::trends::TrendsHandler<'_> {
        methods::trends::TrendsHandler::new(self)
    }

    /// Access custom emoji endpoints.
    pub fn emojis(&self) -> methods::emojis::EmojisHandler<'_> {
        methods::emojis::EmojisHandler::new(self)
    }

    /// Access instance metadata endpoints.
    pub fn instance(&self) -> methods::instance::InstanceHandler<'_> {
        methods::instance::InstanceHandler::new(self)
    }

    /// Create a new streaming client for real-time events.
    pub fn streaming(&self) -> streaming::StreamingClient {
        streaming::StreamingClient::new(&self.base_url, self.access_token.clone())
    }
}
