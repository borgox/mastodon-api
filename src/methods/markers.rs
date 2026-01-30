use crate::MastodonClient;
use crate::error::Result;
use crate::models::Marker;
use std::collections::HashMap;

/// Handler for marker-related API endpoints.
pub struct MarkersHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> MarkersHandler<'a> {
    /// Creates a new `MarkersHandler` for the given client.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches markers for the given timelines.
    ///
    /// Parameters:
    /// - `timelines`: A list of timelines to fetch markers for (e.g., "home", "notifications").
    ///
    /// Returns:
    /// - `Result<HashMap<String, Marker>>`: A map of timeline names to markers.
    ///
    /// Corresponds to `GET /api/v1/markers`.
    pub async fn get(&self, timelines: &[&str]) -> Result<HashMap<String, Marker>> {
        let url = format!("{}/api/v1/markers", self.client.base_url());
        let mut req = self.client.http_client().get(&url);
        for timeline in timelines {
            req = req.query(&[("timeline[]", timeline)]);
        }
        self.client.send(req).await
    }

    /// Updates or creates a marker for a timeline.
    ///
    /// Parameters:
    /// - `timeline`: The timeline to update ("home" or "notifications").
    /// - `last_read_id`: The ID of the status to mark as read.
    ///
    /// Returns:
    /// - `Result<Marker>`: The updated marker.
    ///
    /// Corresponds to `POST /api/v1/markers`.
    pub async fn update(&self, timeline: &str, last_read_id: &str) -> Result<Marker> {
        let url = format!("{}/api/v1/markers", self.client.base_url());
        let mut params = HashMap::new();
        let mut inner_params = HashMap::new();
        inner_params.insert("last_read_id", last_read_id);
        params.insert(timeline, inner_params);

        let req = self.client.http_client().post(&url).form(&params);
        let results: HashMap<String, Marker> = self.client.send(req).await?;
        results
            .get(timeline)
            .cloned()
            .ok_or_else(|| crate::error::MastodonError::ApiError {
                status: reqwest::StatusCode::OK,
                message: format!("Marker for {} not returned", timeline),
            })
    }
}
