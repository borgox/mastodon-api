use crate::MastodonClient;
use crate::error::Result;
use crate::models::CustomEmoji;

/// Handler for custom emoji API endpoints.
pub struct EmojisHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> EmojisHandler<'a> {
    /// Creates a new `EmojisHandler` for the given client.
    ///
    /// Parameters:
    /// - `client`: The client to use for making requests.
    ///
    /// Returns:
    /// - `EmojisHandler`: The created emojis handler.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches all custom emojis available on the instance.
    ///
    /// Returns:
    /// - `Result<Vec<CustomEmoji>>`: The fetched custom emojis.
    ///
    /// Corresponds to `GET /api/v1/custom_emojis`.
    pub async fn list(&self) -> Result<Vec<CustomEmoji>> {
        let url = format!("{}/api/v1/custom_emojis", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }
}
