use mastodon_api::MastodonClient;
use mockito::Server;
use serde_json::json;

#[tokio::test]
async fn test_get_instance() {
    let mut server = Server::new_async().await;
    let url = server.url();

    let _m = server
        .mock("GET", "/api/v1/instance")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "uri": "mastodon.social",
                "title": "Mastodon",
                "description": "The original Mastodon instance",
                "email": "admin@mastodon.social",
                "version": "4.2.0"
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = MastodonClient::new(&url);
    let instance = client.instance().get().await.unwrap();

    assert_eq!(instance.uri, "mastodon.social");
    assert_eq!(instance.version, "4.2.0");
}

#[tokio::test]
async fn test_rate_limiting_retry() {
    let mut server = Server::new_async().await;
    let url = server.url();

    // First request fails with 429
    let _m1 = server
        .mock("GET", "/api/v1/instance")
        .with_status(429)
        .with_header("X-RateLimit-Reset", "2026-01-30T12:00:00Z")
        .create_async()
        .await;

    // Second request (retry) succeeds
    let _m2 = server
        .mock("GET", "/api/v1/instance")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "uri": "mastodon.social",
                "title": "Mastodon",
                "description": "The original Mastodon instance",
                "email": "admin@mastodon.social",
                "version": "4.2.0"
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = MastodonClient::new(&url);
    let instance = client.instance().get().await.unwrap();

    assert_eq!(instance.uri, "mastodon.social");
}
