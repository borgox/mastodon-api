use crate::MastodonClient;
use crate::error::Result;
use crate::models::Search;

/// Handler for global search API endpoints.
pub struct SearchHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> SearchHandler<'a> {
    /// Creates a new `SearchHandler` for the given client.
    ///
    /// Parameters:
    /// - `client`: The client to use for making requests.
    ///
    /// Returns:
    /// - `SearchHandler`: The created search handler.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Searches for accounts, statuses, and hashtags matching the given query.
    ///
    /// Parameters:
    /// - `query`: The search query.
    ///
    /// Returns:
    /// - `Result<Search>`: The search results.
    ///
    /// Corresponds to `GET /api/v2/search`.
    pub async fn v2(&self, query: &str) -> Result<Search> {
        let url = format!("{}/api/v2/search", self.client.base_url());
        let req = self.client.http_client().get(&url).query(&[("q", query)]);
        self.client.send(req).await
    }
}
