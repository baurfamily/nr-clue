pub mod nerdgraph;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = nerdgraph::Client::new();

  let maybe_user = client.user_info().await;

  if let Some(user) = maybe_user {
    println!("Name: {}", user.name);
  }

  Ok(())
}
