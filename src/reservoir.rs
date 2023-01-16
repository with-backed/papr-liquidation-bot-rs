use ethers::types::Bytes;
use reqwest::Client;
use reqwest::Error;
use serde::Deserialize;
use std::env;
use strum_macros::Display;

#[derive(Display)]
pub enum PriceKind {
    Upper,
    Lower,
    Twap,
    Spot,
}

#[derive(Display)]
enum OracleQueryParam {
    Kind,
    Currency,
    #[strum(serialize = "twapSeconds")]
    TwapSeconds,
    Collection,
    Token,
}

#[derive(Deserialize)]
pub struct ReservoirOracleResponse {
    pub price: f64,
    message: ReservoirOracleMessage,
    data: Bytes,
}

#[derive(Deserialize)]
pub struct ReservoirOracleMessage {
    id: String,
    payload: Bytes,
    signature: Bytes,
}

pub async fn max_collection_bid(
    collection: &str,
    price_kind: PriceKind,
    quote_currency: &str,
    twap_seconds: Option<u32>,
) -> Result<ReservoirOracleResponse, Error> {
    let api_key = env::var("RESERVOIR_API_KEY").expect("RESERVOIR_API_KEY not set");
    let base_url = env::var("RESERVOIR_URL").expect("RESERVOIR_URL not set");
    let url = format!(
        "{}/oracle/collections/{}/floor-ask/v3",
        base_url, collection
    );
    let client = Client::new();
    let res = client
        .get(&url)
        .query(&[
            (
                OracleQueryParam::Kind.to_string().to_lowercase(),
                price_kind.to_string().to_lowercase(),
            ),
            (
                OracleQueryParam::Currency.to_string().to_lowercase(),
                quote_currency.to_string(),
            ),
            (
                OracleQueryParam::TwapSeconds.to_string(),
                twap_seconds.unwrap_or(0).to_string(),
            ),
        ])
        .header("api_key", api_key)
        .send()
        .await?
        .json::<ReservoirOracleResponse>()
        .await?;
    Ok(res)
}
