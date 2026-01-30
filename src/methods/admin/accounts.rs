use crate::MastodonClient;
use crate::error::Result;

/// Handler for admin account moderation API endpoints.
pub struct AdminAccountsHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> AdminAccountsHandler<'a> {
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Action against an account.
    pub async fn action(&self, id: &str, r#type: &str) -> Result<()> {
        let url = format!(
            "{}/api/v1/admin/accounts/{}/action",
            self.client.base_url(),
            id
        );
        let req = self
            .client
            .http_client()
            .post(&url)
            .form(&[("type", r#type)]);
        self.client.send(req).await
    }

    /// Suspends an account.
    pub async fn suspend(&self, id: &str) -> Result<()> {
        self.action(id, "suspend").await
    }

    /// Silences an account.
    pub async fn silence(&self, id: &str) -> Result<()> {
        self.action(id, "silence").await
    }
}
