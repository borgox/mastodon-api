use crate::MastodonClient;
use crate::error::Result;
use crate::models::Status;

/// Handler for timeline-related API endpoints.
pub struct TimelinesHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> TimelinesHandler<'a> {
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches the public timeline.
    ///
    /// Corresponds to `GET /api/v1/timelines/public`.
    pub async fn public(&self) -> Result<Vec<Status>> {
        let url = format!("{}/api/v1/timelines/public", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Returns a paged request for the public timeline.
    ///
    /// This allows for iterating through the timeline page-by-page.
    pub async fn public_paged(&self) -> Result<crate::paging::PagedRequest<'_, Status>> {
        let url = format!("{}/api/v1/timelines/public", self.client.base_url());
        Ok(crate::paging::PagedRequest::new(self.client, url))
    }

    /// Fetches the home timeline for the authenticated user.
    ///
    /// Corresponds to `GET /api/v1/timelines/home`.
    pub async fn home(&self) -> Result<Vec<Status>> {
        let url = format!("{}/api/v1/timelines/home", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }
}
