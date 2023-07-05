use std::env;
use gql_client::Client;
use serde::{Deserialize}; //, Serialize};
use std::collections::HashMap;


#[derive(Deserialize)]
pub struct Data {
   actor: Actor
}

#[derive(Deserialize)]
pub struct Actor {
    user: User
}

#[derive(Deserialize)]
pub struct User {
   name: String
}

// #[derive(Serialize)]
// pub struct Vars {
//    id: u32
// }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let endpoint = "https://api.newrelic.com/graphql";
   let query = "
     query ShowUser {
        actor {
            user {
                name
            }
        }
     }
   ";

    let mut headers = HashMap::new();
    if let Ok(api_key) = env::var("NR_API_KEY") {
        headers.insert("api-key", api_key);
    }

   let client = Client::new_with_headers(endpoint, headers);

   // let vars = Vars { id: 1 };
   // let maybe_data = client.query_with_vars::<Data, Vars>(query, vars).await.unwrap();

   let maybe_data = client.query::<Data>(query).await.unwrap();

   if let Some(data) = maybe_data {
     println!("Name: {}", data.actor.user.name);
   }

   Ok(())
}
