use crate::MastodonClient;
use crate::error::Result;
use crate::models::{Account, Relationship};
use crate::paging::PagedRequest;

/// Handler for follow request-related API endpoints.
pub struct FollowRequestsHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> FollowRequestsHandler<'a> {
    /// Creates a new `FollowRequestsHandler` for the given client.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches pending follow requests.
    ///
    /// Returns:
    /// - `Result<Vec<Account>>`: The pending follow requests.
    ///
    /// Corresponds to `GET /api/v1/follow_requests`.
    pub async fn list(&self) -> Result<Vec<Account>> {
        let url = format!("{}/api/v1/follow_requests", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Returns a paged request for fetching pending follow requests.
    pub fn list_paged(&self) -> PagedRequest<'a, Account> {
        let url = format!("{}/api/v1/follow_requests", self.client.base_url());
        PagedRequest::new(self.client, url)
    }

    /// Accepts a follow request from the given account.
    ///
    /// Parameters:
    /// - `id`: The ID of the account whose follow request should be accepted.
    ///
    /// Returns:
    /// - `Result<Relationship>`: The relationship with the accepted account.
    ///
    /// Corresponds to `POST /api/v1/follow_requests/:id/authorize`.
    pub async fn authorize(&self, id: &str) -> Result<Relationship> {
        let url = format!(
            "{}/api/v1/follow_requests/{}/authorize",
            self.client.base_url(),
            id
        );
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Rejects a follow request from the given account.
    ///
    /// Parameters:
    /// - `id`: The ID of the account whose follow request should be rejected.
    ///
    /// Returns:
    /// - `Result<Relationship>`: The relationship with the rejected account.
    ///
    /// Corresponds to `POST /api/v1/follow_requests/:id/reject`.
    pub async fn reject(&self, id: &str) -> Result<Relationship> {
        let url = format!(
            "{}/api/v1/follow_requests/{}/reject",
            self.client.base_url(),
            id
        );
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }
}
