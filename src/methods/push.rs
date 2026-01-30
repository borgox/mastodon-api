use crate::MastodonClient;
use crate::error::Result;
use crate::models::push::{WebPushAlerts, WebPushSubscription};

/// Handler for Web Push API endpoints.
pub struct PushHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> PushHandler<'a> {
    /// Creates a new `PushHandler` for the given client.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches the current Web Push subscription.
    ///
    /// Returns:
    /// - `Result<WebPushSubscription>`: The subscription.
    ///
    /// Corresponds to `GET /api/v1/push/subscription`.
    pub async fn get_subscription(&self) -> Result<WebPushSubscription> {
        let url = format!("{}/api/v1/push/subscription", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Creates or updates a Web Push subscription.
    ///
    /// Parameters:
    /// - `endpoint`: The endpoint of the Web Push subscription.
    /// - `p256dh`: The p256dh public key.
    /// - `auth`: The authentication secret.
    /// - `alerts`: The alerts to enable.
    ///
    /// Returns:
    /// - `Result<WebPushSubscription>`: The created/updated subscription.
    ///
    /// Corresponds to `POST /api/v1/push/subscription`.
    pub async fn subscribe(
        &self,
        endpoint: &str,
        p256dh: &str,
        auth: &str,
        alerts: Option<WebPushAlerts>,
    ) -> Result<WebPushSubscription> {
        let url = format!("{}/api/v1/push/subscription", self.client.base_url());
        let mut form = vec![
            ("subscription[endpoint]", endpoint.to_string()),
            ("subscription[keys][p256dh]", p256dh.to_string()),
            ("subscription[keys][auth]", auth.to_string()),
        ];

        if let Some(a) = alerts {
            if a.follow {
                form.push(("data[alerts][follow]", "true".to_string()));
            }
            if a.favourite {
                form.push(("data[alerts][favourite]", "true".to_string()));
            }
            if a.reblog {
                form.push(("data[alerts][reblog]", "true".to_string()));
            }
            if a.mention {
                form.push(("data[alerts][mention]", "true".to_string()));
            }
            if a.poll {
                form.push(("data[alerts][poll]", "true".to_string()));
            }
            if a.status {
                form.push(("data[alerts][status]", "true".to_string()));
            }
        }

        let req = self.client.http_client().post(&url).form(&form);
        self.client.send(req).await
    }

    /// Updates existing Web Push alerts.
    ///
    /// Parameters:
    /// - `alerts`: The updated alerts.
    ///
    /// Returns:
    /// - `Result<WebPushSubscription>`: The updated subscription.
    ///
    /// Corresponds to `PUT /api/v1/push/subscription`.
    pub async fn update_alerts(&self, alerts: WebPushAlerts) -> Result<WebPushSubscription> {
        let url = format!("{}/api/v1/push/subscription", self.client.base_url());
        let mut form = vec![];
        if alerts.follow {
            form.push(("data[alerts][follow]", "true".to_string()));
        }
        if alerts.favourite {
            form.push(("data[alerts][favourite]", "true".to_string()));
        }
        if alerts.reblog {
            form.push(("data[alerts][reblog]", "true".to_string()));
        }
        if alerts.mention {
            form.push(("data[alerts][mention]", "true".to_string()));
        }
        if alerts.poll {
            form.push(("data[alerts][poll]", "true".to_string()));
        }
        if alerts.status {
            form.push(("data[alerts][status]", "true".to_string()));
        }

        let req = self.client.http_client().put(&url).form(&form);
        self.client.send(req).await
    }

    /// Deletes the current Web Push subscription.
    ///
    /// Returns:
    /// - `Result<()>`: Success if the subscription was deleted.
    ///
    /// Corresponds to `DELETE /api/v1/push/subscription`.
    pub async fn unsubscribe(&self) -> Result<()> {
        let url = format!("{}/api/v1/push/subscription", self.client.base_url());
        let req = self.client.http_client().delete(&url);
        self.client.send(req).await
    }
}
