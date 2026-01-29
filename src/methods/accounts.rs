use crate::MastodonClient;
use crate::error::Result;
use crate::models::Account;

/// Handler for account-related API endpoints.
pub struct AccountsHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> AccountsHandler<'a> {
    /// Creates a new `AccountsHandler` for the given client.
    ///
    /// Parameters:
    /// - `client`: The client to use for making requests.
    ///
    /// Returns:
    /// - `AccountsHandler`: The created accounts handler.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Verifies the authenticated user's credentials and returns their account info.
    ///
    /// Returns:
    /// - `Result<Account>`: The verified account.
    ///
    /// Corresponds to `GET /api/v1/accounts/verify_credentials`.
    pub async fn verify_credentials(&self) -> Result<Account> {
        let url = format!(
            "{}/api/v1/accounts/verify_credentials",
            self.client.base_url()
        );
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Fetches an account by its ID.
    ///
    /// Parameters:
    /// - `id`: The ID of the account to fetch.
    ///
    /// Returns:
    /// - `Result<Account>`: The fetched account.
    ///
    /// Corresponds to `GET /api/v1/accounts/:id`.
    pub async fn get(&self, id: &str) -> Result<Account> {
        let url = format!("{}/api/v1/accounts/{}", self.client.base_url(), id);
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Follows the given account.
    ///
    /// Parameters:
    /// - `id`: The ID of the account to follow.
    ///
    /// Returns:
    /// - `Result<crate::models::Relationship>`: The followed relationship.
    ///
    /// Corresponds to `POST /api/v1/accounts/:id/follow`.
    pub async fn follow(&self, id: &str) -> Result<crate::models::Relationship> {
        let url = format!("{}/api/v1/accounts/{}/follow", self.client.base_url(), id);
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Unfollows the given account.
    ///
    /// Parameters:
    /// - `id`: The ID of the account to unfollow.
    ///
    /// Returns:
    /// - `Result<crate::models::Relationship>`: The unfollowed relationship.
    ///
    /// Corresponds to `POST /api/v1/accounts/:id/unfollow`.
    pub async fn unfollow(&self, id: &str) -> Result<crate::models::Relationship> {
        let url = format!("{}/api/v1/accounts/{}/unfollow", self.client.base_url(), id);
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Blocks the given account.
    ///
    /// Parameters:
    /// - `id`: The ID of the account to block.
    ///
    /// Returns:
    /// - `Result<crate::models::Relationship>`: The blocked relationship.
    ///
    /// Corresponds to `POST /api/v1/accounts/:id/block`.
    pub async fn block(&self, id: &str) -> Result<crate::models::Relationship> {
        let url = format!("{}/api/v1/accounts/{}/block", self.client.base_url(), id);
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Mutes the given account.
    ///
    /// Parameters:
    /// - `id`: The ID of the account to mute.
    ///
    /// Returns:
    /// - `Result<crate::models::Relationship>`: The muted relationship.
    ///
    /// Corresponds to `POST /api/v1/accounts/:id/mute`.
    pub async fn mute(&self, id: &str) -> Result<crate::models::Relationship> {
        let url = format!("{}/api/v1/accounts/{}/mute", self.client.base_url(), id);
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Searches for accounts matching the given query.
    ///
    /// Parameters:
    /// - `query`: The query to search for.
    /// - `limit`: The maximum number of results to return.
    ///
    /// Returns:
    /// - `Result<Vec<Account>>`: The fetched accounts.
    ///
    /// Corresponds to `GET /api/v1/accounts/search`.
    pub async fn search(&self, query: &str, limit: Option<u32>) -> Result<Vec<Account>> {
        let url = format!("{}/api/v1/accounts/search", self.client.base_url());
        let mut req = self.client.http_client().get(&url).query(&[("q", query)]);
        if let Some(l) = limit {
            req = req.query(&[("limit", l.to_string())]);
        }
        self.client.send(req).await
    }
}
