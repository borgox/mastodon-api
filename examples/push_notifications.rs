use mastodon_api::{MastodonClient, WebPushAlerts};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MastodonClient::new("https://mastodon.social").with_token("YOUR_TOKEN");

    println!("Registering for Web Push Notifications...");

    // In a real scenario, these values come from the browser's PushManager.
    // Here we show how to use the API to subscribe the server to those browser-generated keys.
    let endpoint = "https://updates.push.services.com/wpush/v2/gAAAAAB...";
    let p256dh = "BCEYTCX9_ZpZ...";
    let auth = "8eBytA...";

    // Define which alerts we want to receive via push
    let alerts = WebPushAlerts {
        follow: true,
        favourite: false,
        reblog: true,
        mention: true,
        poll: true,
        status: true,
    };

    println!("Creating/Updating subscription...");
    let subscription = client
        .push()
        .subscribe(endpoint, p256dh, auth, Some(alerts.clone()))
        .await?;

    println!("Successfully subscribed to push notifications!");
    println!("Endpoint: {}", subscription.endpoint);
    println!("Enabled alerts: {:?}", subscription.alerts);

    // Later: update just the alerts without resending keys
    let mut updated_alerts = alerts;
    updated_alerts.favourite = true;

    println!("Updating alert preferences...");
    let updated_sub = client.push().update_alerts(updated_alerts).await?;
    println!("Favourites alert enabled: {}", updated_sub.alerts.favourite);

    // Verification: Fetch current subscription
    let current = client.push().get_subscription().await?;
    println!("Current subscription ID: {}", current.id);

    Ok(())
}
