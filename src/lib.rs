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
//! ```rust
//! let client = MastodonClient::new("https://mastodon.social").with_token("your_token");
//! client.statuses().create_simple("Hello from Rust!").await?;
//! ```

pub mod error;
pub mod methods;
pub mod models;
pub mod paging;
pub mod streaming;

pub use error::{MastodonError, Result};
pub use models::{Account, Status};

use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

/// The main entry point for interacting with the Mastodon API.
///
/// Use `MastodonClient::new` to create a new instance, and `with_token` to authenticate.
pub struct MastodonClient {
    base_url: String,
    client: Client,
    access_token: Option<String>,
}

impl MastodonClient {
    /// Creates a new `MastodonClient` for the given instance URL.
    ///
    /// # Example
    /// ```rust
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
        let mut builder = builder;
        if let Some(token) = &self.access_token {
            builder = builder.bearer_auth(token);
        }

        let response = builder.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let message = response.text().await.unwrap_or_default();
            return Err(MastodonError::ApiError { status, message });
        }

        Ok(response.json().await?)
    }

    /// Access account-related endpoints.
    pub fn accounts(&self) -> methods::accounts::AccountsHandler<'_> {
        methods::accounts::AccountsHandler::new(self)
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
