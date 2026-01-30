use crate::MastodonClient;
use crate::error::Result;
use serde::de::DeserializeOwned;

/// A builder for requests that return a list of items (e.g., timelines, account lists).
pub struct ListBuilder<'a, T> {
    client: &'a MastodonClient,
    url: String,
    limit: Option<u32>,
    since_id: Option<String>,
    min_id: Option<String>,
    max_id: Option<String>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: DeserializeOwned> ListBuilder<'a, T> {
    /// Creates a new `ListBuilder` for the given URL.
    pub fn new(client: &'a MastodonClient, url: String) -> Self {
        Self {
            client,
            url,
            limit: None,
            since_id: None,
            min_id: None,
            max_id: None,
            _marker: std::marker::PhantomData,
        }
    }

    /// Maximum number of results to return.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }

    /// Return results newer than this ID.
    pub fn since_id(mut self, value: &str) -> Self {
        self.since_id = Some(value.to_string());
        self
    }

    /// Return results immediately newer than this ID.
    pub fn min_id(mut self, value: &str) -> Self {
        self.min_id = Some(value.to_string());
        self
    }

    /// Return results older than this ID.
    pub fn max_id(mut self, value: &str) -> Self {
        self.max_id = Some(value.to_string());
        self
    }

    /// Executes the request and returns the list of results.
    pub async fn send(self) -> Result<Vec<T>> {
        let mut req = self.client.http_client().get(&self.url);

        if let Some(l) = self.limit {
            req = req.query(&[("limit", l.to_string())]);
        }
        if let Some(s) = self.since_id {
            req = req.query(&[("since_id", s)]);
        }
        if let Some(min) = self.min_id {
            req = req.query(&[("min_id", min)]);
        }
        if let Some(max) = self.max_id {
            req = req.query(&[("max_id", max)]);
        }

        self.client.send(req).await
    }
}
