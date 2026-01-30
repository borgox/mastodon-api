use futures_util::StreamExt;
use mastodon_api::MastodonClient;
use mastodon_api::streaming::MastodonEvent;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the client
    let base_url = "https://mastodon.social";
    let token = "YOUR_TOKEN";
    let client = MastodonClient::new(base_url).with_token(token);

    println!("Starting Interaction Bot on {}...", base_url);

    // 2. Start a background task for periodic status updates
    let client_clone = client.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Every hour
        loop {
            interval.tick().await;
            println!("Posting periodic status update...");
            let stats = client_clone.instance().activity().await;
            if let Ok(activity) = stats {
                if let Some(week) = activity.first() {
                    let msg = format!(
                        "🤖 Weekly Instance Stats:\nStatuses: {}\nLogins: {}\nNew Registrations: {}",
                        week.statuses, week.logins, week.registrations
                    );
                    let _ = client_clone.statuses().create_simple(&msg).await;
                }
            }
        }
    });

    // 3. Subscribe to the 'user' stream for real-time interactions
    println!("Subscribing to 'user' stream...");
    let mut stream = client.streaming().subscribe("user").await?;

    while let Some(event) = stream.next().await {
        match event {
            Ok(MastodonEvent::Notification(notification)) => {
                match notification.r#type.as_str() {
                    "mention" => {
                        println!("Got mention from @{}", notification.account.username);
                        let status = notification.status.unwrap();
                        let reply = format!(
                            "@{} Hello! Thanks for the mention! 🚀",
                            notification.account.acct
                        );

                        client
                            .statuses()
                            .builder(&reply)
                            .in_reply_to_id(&status.id)
                            .send()
                            .await?;
                        println!("Replied to status {}", status.id);
                    }
                    "follow" => {
                        println!("New follower: @{}", notification.account.username);
                        // Auto-follow back if they aren't followed yet
                        let rel = client
                            .accounts()
                            .relationships(&[notification.account.id.clone()])
                            .await?;
                        if let Some(r) = rel.first() {
                            if !r.following {
                                client.accounts().follow(&notification.account.id).await?;
                                println!("Followed back @{}", notification.account.username);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(MastodonEvent::Update(status)) => {
                println!("New update in home timeline: {}", status.id);
            }
            Err(e) => {
                eprintln!("Stream error: {}", e);
                // Optionally handle reconnection here
            }
            _ => {}
        }
    }

    Ok(())
}
