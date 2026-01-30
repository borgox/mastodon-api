use crate::MastodonClient;
use crate::error::Result;

/// Handler for admin report management API endpoints.
pub struct AdminReportsHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> AdminReportsHandler<'a> {
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches all reports (admin view).
    pub async fn list(&self) -> Result<serde_json::Value> {
        let url = format!("{}/api/v1/admin/reports", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Resolves a report.
    pub async fn resolve(&self, id: &str) -> Result<()> {
        let url = format!(
            "{}/api/v1/admin/reports/{}/resolve",
            self.client.base_url(),
            id
        );
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }
}
