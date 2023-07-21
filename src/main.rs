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

    let results = client.nrql("SElECT count(*) FROM Transaction FACET name".to_string()).await.unwrap();

    // don't do unwrap ^^^ check the option first!

    // if let Some(results) = results {
    //     println!("results: {:?}", results);
    // }

    for hash in results.iter() {
        println!("-------");
        for (key, value) in hash {
            println!("{key} -> {value:?}");
        }
    }

    Ok(())
}
