use gql_client::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Data {
   user: User
}

#[derive(Deserialize)]
pub struct User {
   id: String,
   name: String
}

#[derive(Serialize)]
pub struct Vars {
   id: u32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let endpoint = "https://graphqlzero.almansi.me/api";
   let query = r#"
       query UserByIdQuery($id: ID!) {
           user(id: $id) {
               id
               name
           }
       }
   "#;

   let client = Client::new(endpoint);
   let vars = Vars { id: 1 };
   let maybe_data = client.query_with_vars::<Data, Vars>(query, vars).await.unwrap();

   if let Some(data) = maybe_data {
     println!("Id: {}, Name: {}", data.user.id, data.user.name);
   }

   Ok(())
}
