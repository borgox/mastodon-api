use crate::MastodonClient;
use crate::error::Result;
use crate::models::MediaAttachment;
use reqwest::multipart;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

/// Handler for media upload API endpoints.
pub struct MediaHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> MediaHandler<'a> {
    // Creates a new `MediaHandler` for the given client.
    //
    // Parameters:
    // - `client`: The client to use for making requests.
    //
    // Returns:
    // - `MediaHandler`: The created media handler.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Uploads a media file (image, video, etc.) to the instance.
    ///
    /// This is an asynchronous operation. You should use the returned `MediaAttachment` ID
    /// when creating a status to attach the media.
    ///
    /// Parameters:
    /// - `file_path`: The path to the file to upload.
    /// - `description`: An optional description for the media.
    ///
    /// Returns:
    /// - `Result<MediaAttachment>`: The uploaded media attachment.
    ///
    /// Corresponds to `POST /api/v1/media`.
    pub async fn upload(
        &self,
        file_path: &str,
        description: Option<String>,
    ) -> Result<MediaAttachment> {
        let file = File::open(file_path)
            .await
            .map_err(|e| crate::error::MastodonError::Custom(e.to_string()))?;
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = reqwest::Body::wrap_stream(stream);

        let part = multipart::Part::stream(body).file_name("file.png"); // Generic filename
        let mut form = multipart::Form::new().part("file", part);

        if let Some(desc) = description {
            form = form.text("description", desc);
        }

        let url = format!("{}/api/v1/media", self.client.base_url());
        let req = self.client.http_client().post(&url).multipart(form);
        self.client.send(req).await
    }
}
