use crate::MastodonClient;
use crate::error::Result;
use crate::methods::statuses::CreateStatusParams;
use crate::models::Status;

/// A builder for creating a new status.
pub struct StatusBuilder<'a> {
    client: &'a MastodonClient,
    params: CreateStatusParams,
}

impl<'a> StatusBuilder<'a> {
    pub fn new(client: &'a MastodonClient, text: &str) -> Self {
        Self {
            client,
            params: CreateStatusParams {
                status: text.to_string(),
                in_reply_to_id: None,
                sensitive: false,
                spoiler_text: None,
                visibility: None,
                language: None,
            },
        }
    }

    /// ID of the status being replied to.
    pub fn in_reply_to_id(mut self, value: &str) -> Self {
        self.params.in_reply_to_id = Some(value.to_string());
        self
    }

    /// Mark the status as sensitive.
    pub fn sensitive(mut self, value: bool) -> Self {
        self.params.sensitive = value;
        self
    }

    /// Text warning before the content.
    pub fn spoiler_text(mut self, value: &str) -> Self {
        self.params.spoiler_text = Some(value.to_string());
        self
    }

    /// Visibility of the status ("public", "unlisted", "private", "direct").
    pub fn visibility(mut self, value: &str) -> Self {
        self.params.visibility = Some(value.to_string());
        self
    }

    /// ISO 639 language code.
    pub fn language(mut self, value: &str) -> Self {
        self.params.language = Some(value.to_string());
        self
    }

    /// Executes the request to create the status.
    pub async fn send(self) -> Result<Status> {
        self.client.statuses().create(&self.params).await
    }
}
