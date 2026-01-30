use crate::MastodonClient;
use crate::error::Result;
use crate::methods::builders::SearchBuilder;
use crate::models::Search;

/// Handler for global search API endpoints.
pub struct SearchHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> SearchHandler<'a> {
    /// Creates a new `SearchHandler` for the given client.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Returns a builder for creating a search request.
    pub fn builder(&self, query: &str) -> SearchBuilder<'a> {
        SearchBuilder::new(self.client, query)
    }

    /// Searches for accounts, statuses, and hashtags matching the given query (v2).
    pub async fn v2(&self, query: &str) -> Result<Search> {
        self.builder(query).send().await
    }
}
