use crate::MastodonClient;
use crate::error::Result;
use crate::models::Account;

/// Handler for endorsements (pinned accounts on profile).
pub struct EndorsementsHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> EndorsementsHandler<'a> {
    /// Creates a new `EndorsementsHandler` for the given client.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches all accounts that the authenticated user is currently endorsing.
    ///
    /// Returns:
    /// - `Result<Vec<Account>>`: The endorsed accounts.
    ///
    /// Corresponds to `GET /api/v1/endorsements`.
    pub async fn list(&self) -> Result<Vec<Account>> {
        let url = format!("{}/api/v1/endorsements", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }
}
