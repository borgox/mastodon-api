use crate::MastodonClient;
use crate::error::Result;

/// Handler for domain block-related API endpoints.
pub struct DomainBlocksHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> DomainBlocksHandler<'a> {
    /// Creates a new `DomainBlocksHandler` for the given client.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches all domains blocked by the authenticated user.
    ///
    /// Returns:
    /// - `Result<Vec<String>>`: The blocked domains.
    ///
    /// Corresponds to `GET /api/v1/domain_blocks`.
    pub async fn list(&self) -> Result<Vec<String>> {
        let url = format!("{}/api/v1/domain_blocks", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Blocks a domain for the authenticated user.
    ///
    /// Parameters:
    /// - `domain`: The domain to block.
    ///
    /// Returns:
    /// - `Result<()>`: Success if the domain was blocked.
    ///
    /// Corresponds to `POST /api/v1/domain_blocks`.
    pub async fn block(&self, domain: &str) -> Result<()> {
        let url = format!("{}/api/v1/domain_blocks", self.client.base_url());
        let req = self
            .client
            .http_client()
            .post(&url)
            .form(&[("domain", domain)]);
        self.client.send(req).await
    }

    /// Unblocks a domain.
    ///
    /// Parameters:
    /// - `domain`: The domain to unblock.
    ///
    /// Returns:
    /// - `Result<()>`: Success if the domain was unblocked.
    ///
    /// Corresponds to `DELETE /api/v1/domain_blocks`.
    pub async fn unblock(&self, domain: &str) -> Result<()> {
        let url = format!("{}/api/v1/domain_blocks", self.client.base_url());
        let req = self
            .client
            .http_client()
            .delete(&url)
            .form(&[("domain", domain)]);
        self.client.send(req).await
    }
}
