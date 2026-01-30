use crate::MastodonClient;
use crate::error::Result;

/// Handler for admin domain federation API endpoints.
pub struct AdminDomainFederationHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> AdminDomainFederationHandler<'a> {
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches all allowed domains.
    pub async fn list_allows(&self) -> Result<serde_json::Value> {
        let url = format!("{}/api/v1/admin/domain_allows", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Fetches all blocked domains.
    pub async fn list_blocks(&self) -> Result<serde_json::Value> {
        let url = format!("{}/api/v1/admin/domain_blocks", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }
}
