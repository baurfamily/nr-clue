mod nerdgraph;
mod ui;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = nerdgraph::Client::new();

//     let actor = client.actor_info().await;

//     if let Some(actor) = actor {
//         println!("Num of Accounts: {}", actor.accounts.len());
//         println!("First Account: {} ({})", actor.accounts[0].name, actor.accounts[0].id);
//         println!("User name: {}", actor.user.name);
//     }

//     let results = client.nrql(265881,"SElECT count(*) FROM Transaction FACET name".to_string()).await;

//     if let Some(results) = results {
//         // println!("results: {:?}", results);
//         for hash in &results {
//             println!("------------------");
//             for (key, value) in hash {
//                 match value {
//                     nerdgraph::JsonValue::String(value) => println!("\t{} = {}", key,  value) ,
//                     nerdgraph::JsonValue::Integer(value) => println!("\t{} = {}", key, value) ,
//                     nerdgraph::JsonValue::Boolean(value) => println!("\t{} = {}", key,  value) ,
//                     nerdgraph::JsonValue::Float(value) => println!("\t{} = {}", key,  value) ,
//                 }

//             }
//         }
//     }

//     Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ui::event_loop()
}
