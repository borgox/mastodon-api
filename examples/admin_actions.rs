use mastodon_api::MastodonClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MastodonClient::new("https://mastodon.social").with_token("ADMIN_TOKEN");

    println!("Fetching admin reports...");

    let reports = client.admin().reports().list().await?;
    println!("Total reports: {:?}", reports);

    // Example: Suspend an account (CAUTION!)
    // client.admin().accounts().suspend("12345").await?;

    Ok(())
}
