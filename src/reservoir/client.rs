use once_cell::sync::Lazy;
use std::env;
use serde::{de::DeserializeOwned, Serialize};

static API_KEY: Lazy<String> =
    Lazy::new(|| env::var("RESERVOIR_API_KEY").expect("RESERVOIR_API_KEY not set"));
static BASE_URL: Lazy<String> =
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
            .get(format!("{}{}", BASE_URL.to_string(), url))
            .query(&query)
            .header("api_key", API_KEY.to_string())
            .send()
            .await?
            .json::<D>()
            .await?;
        Ok(res)
    }
}
