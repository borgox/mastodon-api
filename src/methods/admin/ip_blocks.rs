use crate::MastodonClient;
use crate::error::Result;

/// Handler for admin IP block management API endpoints.
pub struct AdminIpBlocksHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> AdminIpBlocksHandler<'a> {
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches all IP blocks.
    pub async fn list(&self) -> Result<serde_json::Value> {
        let url = format!("{}/api/v1/admin/ip_blocks", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }
}
