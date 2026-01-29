use crate::MastodonClient;
use crate::error::Result;
use crate::models::{Status, Tag};

/// Handler for trending content API endpoints.
pub struct TrendsHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> TrendsHandler<'a> {
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches trending hashtags.
    ///
    /// Corresponds to `GET /api/v1/trends/tags`.
    pub async fn tags(&self) -> Result<Vec<Tag>> {
        let url = format!("{}/api/v1/trends/tags", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Fetches trending statuses.
    ///
    /// Corresponds to `GET /api/v1/trends/statuses`.
    pub async fn statuses(&self) -> Result<Vec<Status>> {
        let url = format!("{}/api/v1/trends/statuses", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }
}
