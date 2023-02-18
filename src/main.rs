mod auth;
mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = client::Client::new();
    client.auth().await?;

    println!("{}", client.auth_token);

    Ok(())
}
