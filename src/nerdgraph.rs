
// pub mod nerdgraph {
  use std::env;
  use gql_client;
  use serde::{Deserialize}; //, Serialize};
  use std::collections::HashMap;

  pub struct Client {
    api_key: String,
    api_endpoint: String,
  }


  #[derive(Deserialize)]
  pub struct Data {
    pub actor: Actor
  }

  #[derive(Deserialize)]
  pub struct Actor {
      pub user: User
  }

  #[derive(Deserialize)]
  pub struct User {
    pub name: String
  }

  impl Client {
    pub fn new() -> Self {
      let mut api_key = "".to_string();
      let mut api_endpoint = "https://api.newrelic.com/graphql".to_string();

      // it should be considered an error if no API key is present
      // but I'm not sure the best way to do this yet
      if let Ok(env_api_key) = env::var("NR_API_KEY") {
        api_key = env_api_key;
      }

      // we expect this won't be specified for most users,
      // but we'll allow it since sometimes it does need to get
      // overridden (staging, non-us accounts, etc)
      if let Ok(env_endpoint) = env::var("NR_API_ENDPOINT") {
        api_endpoint = env_endpoint;
      }

      Self { api_key, api_endpoint }
    }

    pub async fn user_info(self) -> Option<User> {
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
      headers.insert("api-key", self.api_key);

      let client = gql_client::Client::new_with_headers(self.api_endpoint, headers);
      let maybe_data = client.query::<Data>(query).await.unwrap();

      match maybe_data {
        Some(data) => Some(data.actor.user),
        None => None,
      }
    }
  }

// }
