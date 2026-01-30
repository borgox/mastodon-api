use mastodon_api::MastodonClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MastodonClient::new("https://mastodon.social").with_token("YOUR_TOKEN");

    println!("Fetching instance announcements...");

    let announcements = client.announcements().list().await?;
    for announcement in announcements {
        println!(
            "Announcement (ID {}): {}",
            announcement.id, announcement.content
        );

        // Example: Add a reaction
        // client.announcements().add_reaction(&announcement.id, "🚀").await?;
    }

    Ok(())
}
