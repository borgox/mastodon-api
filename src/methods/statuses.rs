use crate::MastodonClient;
use crate::error::Result;
use crate::methods::builders::StatusBuilder;
use crate::models::Status;
use serde::Serialize;

/// Handler for status-related API endpoints.
pub struct StatusesHandler<'a> {
    client: &'a MastodonClient,
}

/// Parameters for creating a new status.
#[derive(Serialize)]
pub struct CreateStatusParams {
    /// The text content of the status.
    pub status: String,
    /// ID of the status being replied to, if any.
    pub in_reply_to_id: Option<String>,
    /// Whether the status should be marked as sensitive.
    pub sensitive: bool,
    /// Text to be shown as a warning before the status content.
    pub spoiler_text: Option<String>,
    /// Visibility of the status ("public", "unlisted", "private", "direct").
    pub visibility: Option<String>,
    /// ISO 639 language code for the status.
    pub language: Option<String>,
}

impl<'a> StatusesHandler<'a> {
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Returns a builder for creating a new status.
    pub fn builder(&self, text: &str) -> StatusBuilder<'a> {
        StatusBuilder::new(self.client, text)
    }

    /// Fetches a specific status by its ID.
    ///
    /// Parameters:
    /// - `id`: The ID of the status to fetch.
    ///
    /// Returns:
    /// - `Result<Status>`: The fetched status.
    ///
    /// Corresponds to `GET /api/v1/statuses/:id`.
    pub async fn get(&self, id: &str) -> Result<Status> {
        let url = format!("{}/api/v1/statuses/{}", self.client.base_url(), id);
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Creates a new status.
    ///
    /// Parameters:
    /// - `params`: The parameters for the status to create.
    ///
    /// Returns:
    /// - `Result<Status>`: The created status.
    ///
    /// Corresponds to `POST /api/v1/statuses`.
    pub async fn create(&self, params: &CreateStatusParams) -> Result<Status> {
        let url = format!("{}/api/v1/statuses", self.client.base_url());
        let req = self.client.http_client().post(&url).json(params);
        self.client.send(req).await
    }

    /// Creates a new status with just text.
    ///
    /// Parameters:
    /// - `text`: The text content of the status.
    ///
    /// Returns:
    /// - `Result<Status>`: The created status.
    ///
    /// This is a convenience method for simple posts.
    pub async fn create_simple(&self, text: &str) -> Result<Status> {
        let params = CreateStatusParams {
            status: text.to_string(),
            in_reply_to_id: None,
            sensitive: false,
            spoiler_text: None,
            visibility: None,
            language: None,
        };
        self.create(&params).await
    }

    /// Deletes a status by its ID.
    ///
    /// Parameters:
    /// - `id`: The ID of the status to delete.
    ///
    /// Returns:
    /// - `Result<Status>`: The deleted status.
    ///
    /// Corresponds to `DELETE /api/v1/statuses/:id`.
    pub async fn delete(&self, id: &str) -> Result<Status> {
        let url = format!("{}/api/v1/statuses/{}", self.client.base_url(), id);
        let req = self.client.http_client().delete(&url);
        self.client.send(req).await
    }

    /// Reblogs (boosts) a status.
    ///
    /// Parameters:
    /// - `id`: The ID of the status to reblog.
    ///
    /// Returns:
    /// - `Result<Status>`: The reblogged status.
    ///
    /// Corresponds to `POST /api/v1/statuses/:id/reblog`.
    pub async fn reblog(&self, id: &str) -> Result<Status> {
        let url = format!("{}/api/v1/statuses/{}/reblog", self.client.base_url(), id);
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Removes a reblog of a status.
    ///
    /// Parameters:
    /// - `id`: The ID of the status to unreblog.
    ///
    /// Returns:
    /// - `Result<Status>`: The unreblogged status.
    ///
    /// Corresponds to `POST /api/v1/statuses/:id/unreblog`.
    pub async fn unreblog(&self, id: &str) -> Result<Status> {
        let url = format!("{}/api/v1/statuses/{}/unreblog", self.client.base_url(), id);
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Favourites a status.
    ///
    /// Parameters:
    /// - `id`: The ID of the status to favourite.
    ///
    /// Returns:
    /// - `Result<Status>`: The favourited status.
    ///
    /// Corresponds to `POST /api/v1/statuses/:id/favourite`.
    pub async fn favourite(&self, id: &str) -> Result<Status> {
        let url = format!(
            "{}/api/v1/statuses/{}/favourite",
            self.client.base_url(),
            id
        );
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Removes a status from favourites.
    ///
    /// Parameters:
    /// - `id`: The ID of the status to unfavourite.
    ///
    /// Returns:
    /// - `Result<Status>`: The unfavourited status.
    ///
    /// Corresponds to `POST /api/v1/statuses/:id/unfavourite`.
    pub async fn unfavourite(&self, id: &str) -> Result<Status> {
        let url = format!(
            "{}/api/v1/statuses/{}/unfavourite",
            self.client.base_url(),
            id
        );
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Bookmarks a status.
    ///
    /// Parameters:
    /// - `id`: The ID of the status to bookmark.
    ///
    /// Returns:
    /// - `Result<Status>`: The bookmarked status.
    ///
    /// Corresponds to `POST /api/v1/statuses/:id/bookmark`.
    pub async fn bookmark(&self, id: &str) -> Result<Status> {
        let url = format!("{}/api/v1/statuses/{}/bookmark", self.client.base_url(), id);
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Removes a status from bookmarks.
    ///
    /// Parameters:
    /// - `id`: The ID of the status to bookmark.
    ///
    /// Returns:
    /// - `Result<Status>`: The bookmarked status.
    ///
    /// Corresponds to `POST /api/v1/statuses/:id/unbookmark`.
    pub async fn unbookmark(&self, id: &str) -> Result<Status> {
        let url = format!(
            "{}/api/v1/statuses/{}/unbookmark",
            self.client.base_url(),
            id
        );
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Fetches the context (ancestors and descendants) for a status.
    ///
    /// Parameters:
    /// - `id`: The ID of the status to fetch context for.
    ///
    /// Returns:
    /// - `Result<Context>`: The fetched context.
    ///
    /// Corresponds to `GET /api/v1/statuses/:id/context`.
    pub async fn get_context(&self, id: &str) -> Result<crate::models::status::Context> {
        let url = format!("{}/api/v1/statuses/{}/context", self.client.base_url(), id);
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }
}
