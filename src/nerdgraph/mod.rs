use gql_client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Serialize)]
struct Vars {
    account_id: i32,
    nrql_query: String,
}

pub struct Client {
    client: gql_client::Client,
}

#[derive(Deserialize)]
#[serde(untagged)] // serde looks at subfields to determine which one to use
pub enum Actor {
    Nrql(NrqlActor),
    Info(InfoActor),
}

#[derive(Deserialize)]
pub struct Data {
    pub actor: Actor,
}

#[derive(Deserialize)]
pub struct InfoActor {
    pub accounts: Vec<Account>,
    pub user: User,
}

#[derive(Deserialize)]
pub struct NrqlActor {
    pub account: NrqlAccount,
}

#[derive(Deserialize)]
pub struct NrqlAccount {
    pub nrql: Nrql,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)] // serde looks at subfields to determine which one to use
pub enum JsonValue {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
}

type Result = HashMap<String, JsonValue>;
type Results = Vec<Result>;

#[derive(Deserialize)]
pub struct Nrql {
    pub nrql: String,
    pub results: Results,
}

#[derive(Deserialize)]
pub struct Account {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct User {
    pub name: String,
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

        let mut headers = HashMap::new();
        headers.insert("api-key", &api_key);

        let client: gql_client::Client =
            gql_client::Client::new_with_headers(&api_endpoint, headers);

        Self { client }
    }

    pub async fn actor_info(&self) -> Option<InfoActor> {
        let query = "
            query nrClueGetActorInfo {
                actor {
                    accounts { id name }
                    user { name }
                }
            }
        ";

        let data = self.client.query::<Data>(query).await.unwrap();

        match data {
            Some(data) => match data.actor {
                Actor::Info(actor) => Some(actor),
                Actor::Nrql(_) => None,
            },
            None => None,
        }
    }

    pub async fn nrql(&self, account_id: i32, nrql_query: String) -> Option<Results> {
        let query = "
            query nrClueGetNrql($account_id: Int!, $nrql_query: Nrql!) {
                actor {
                    account(id: $account_id) {
                        nrql(query:$nrql_query) {
                            results
                            nrql
                        }
                    }
                }
            }
        ";

        let vars = Vars {
            account_id,
            nrql_query,
        };
        let data = self
            .client
            .query_with_vars::<Data, Vars>(query, vars)
            .await
            .unwrap();
        // let data = self.client.query::<Data>(query).await.unwrap();

        match data {
            Some(data) => match data.actor {
                Actor::Info(_) => None,
                Actor::Nrql(actor) => Some(actor.account.nrql.results),
            },
            None => None,
        }
    }
}

// }
