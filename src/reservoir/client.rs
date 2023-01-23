use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};
use std::env;

static API_KEY: Lazy<String> =
    Lazy::new(|| env::var("RESERVOIR_API_KEY").expect("RESERVOIR_API_KEY not set"));
static URL: Lazy<String> =
    Lazy::new(|| env::var("RESERVOIR_URL").expect("RESERVOIR_API_KEY not set"));

pub struct ReservoirClient {
    client: reqwest::Client,
}

impl Default for ReservoirClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl ReservoirClient {
    pub async fn get<Q: Serialize, D: DeserializeOwned>(
        &self,
        url: &str,
        query: Q,
    ) -> Result<D, eyre::Error> {
        let res = self
            .client
            .get(format!("{}{}", URL.to_string(), url))
            .query(&query)
            .header("api_key", API_KEY.to_string())
            .send()
            .await?
            .json::<D>()
            .await?;
        Ok(res)
    }
}
