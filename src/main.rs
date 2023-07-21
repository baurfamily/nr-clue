mod nerdgraph;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = nerdgraph::Client::new();

    let actor = client.actor_info().await;

    if let Some(actor) = actor {
        println!("Num of Accounts: {}", actor.accounts.len());
        println!("First Account: {} ({})", actor.accounts[0].name, actor.accounts[0].id);
        println!("User name: {}", actor.user.name);
    }

    let results = client.nrql(265881,"SElECT count(*) FROM Transaction".to_string()).await;

    if let Some(results) = results {
        println!("results: {:?}", results);
    }

    Ok(())
}
