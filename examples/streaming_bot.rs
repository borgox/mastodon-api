use futures_util::StreamExt;
use mastdoon_api::MastodonClient;
use mastdoon_api::streaming::MastodonEvent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MastodonClient::new("https://mastodon.social")
        .with_token("IYwPRGIlygcAnjHQrm0pXvtqkaxdj6uYaTzGVwl0XJw");

    println!("Connecting to 'user' stream (mentions and notifications)...");
    let mut stream = client.streaming().subscribe("user").await?;

    while let Some(event_result) = stream.next().await {
        match event_result {
            Ok(MastodonEvent::Update(status)) => {
                println!(
                    "New Status from @{}: {}",
                    status.account.username, status.content
                );
            }
            Ok(MastodonEvent::Notification(notif)) => {
                println!(
                    "New Notification: {} from @{}",
                    notif.r#type, notif.account.username
                );

                // Auto-mention back
                if notif.r#type == "mention" {
                    if let Some(_status) = notif.status {
                        let reply_text =
                            format!("@{}: Thanks for the mention!", notif.account.username);
                        client.statuses().create_simple(&reply_text).await?;
                    }
                }
            }
            Ok(MastodonEvent::Delete(id)) => {
                println!("Status {} was deleted", id);
            }
            Err(e) => eprintln!("Error in stream: {}", e),
            _ => {}
        }
    }

    Ok(())
}
