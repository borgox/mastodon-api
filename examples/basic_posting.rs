use mastodon_api::MastodonClient;
use mastodon_api::methods::statuses::CreateStatusParams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the client
    let client = MastodonClient::new("https://mastodonte.exelvi.tech")
        .with_token("IYwPRGIlygcAnjHQrm0pXvtqkaxdj6uYaTzGVwl0XJw");

    // 2. Simple post
    println!("Posting a simple status...");
    let status = client
        .statuses()
        .create_simple("Hello from the Mastodon API Wrapper! 🦀")
        .await?;
    println!("Posted status with ID: {}", status.id);

    // 3. Post with options
    println!("Posting with options...");
    let params = CreateStatusParams {
        status: "This is a sensitive post".to_string(),
        sensitive: true,
        spoiler_text: Some("Big surprise content".to_string()),
        visibility: Some("unlisted".to_string()),
        in_reply_to_id: None,
        language: Some("en".to_string()),
    };
    client.statuses().create(&params).await?;

    // 4. Interaction
    println!("Favouriting the post...");
    client.statuses().favourite(&status.id).await?;

    Ok(())
}
