use crate::MastodonClient;
use crate::error::Result;
use crate::models::suggestion::Suggestion;

/// Handler for suggestion-related API endpoints.
pub struct SuggestionsHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> SuggestionsHandler<'a> {
    /// Creates a new `SuggestionsHandler` for the given client.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches accounts suggested for follow.
    ///
    /// Parameters:
    /// - `limit`: Maximum number of results to return.
    ///
    /// Returns:
    /// - `Result<Vec<Suggestion>>`: The account suggestions.
    ///
    /// Corresponds to `GET /api/v1/suggestions`.
    pub async fn list(&self, limit: Option<u32>) -> Result<Vec<Suggestion>> {
        let url = format!("{}/api/v1/suggestions", self.client.base_url());
        let mut req = self.client.http_client().get(&url);
        if let Some(l) = limit {
            req = req.query(&[("limit", l.to_string())]);
        }
        self.client.send(req).await
    }

    /// Removes an account from suggestions.
    ///
    /// Parameters:
    /// - `account_id`: The ID of the account to remove from suggestions.
    ///
    /// Returns:
    /// - `Result<()>`: Success if the account was removed from suggestions.
    ///
    /// Corresponds to `DELETE /api/v1/suggestions/:account_id`.
    pub async fn remove(&self, account_id: &str) -> Result<()> {
        let url = format!(
            "{}/api/v1/suggestions/{}",
            self.client.base_url(),
            account_id
        );
        let req = self.client.http_client().delete(&url);
        self.client.send(req).await
    }
}
