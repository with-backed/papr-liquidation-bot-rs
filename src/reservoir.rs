use ethers::types::{Bytes, U256};
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

impl ReservoirOracleResponse {
    pub fn price_in_atomic_units(&self, decimals: u8) -> U256 {
        let one = U256::from_dec_str("10")
            .unwrap()
            .pow(U256::from_dec_str(&decimals.to_string()).unwrap());
        // scalar to prevent loss of precison when converting to atomic
        let scalar = 1e6;
        let u256_scalar = U256::from_dec_str(&scalar.to_string()).unwrap();
        let scaled_price = (self.price * scalar).floor();
        let u256_scaled_price = U256::from_dec_str(&scaled_price.to_string()).unwrap();
        return one
            .checked_mul(u256_scaled_price)
            .expect("price_atomic overflow")
            .checked_div(u256_scalar)
            .expect("price_atomic division error");
    }
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

#[cfg(test)]
mod tests {
    use crate::reservoir::ReservoirOracleMessage;
    use crate::reservoir::ReservoirOracleResponse;
    use ethers::types::{Bytes, U256};
    use std::str::FromStr;

    #[test]
    fn price_in_atomic_units_computes_correctly() {
        let response = ReservoirOracleResponse {
            price: 1.0,
            message: ReservoirOracleMessage {
                id: "".to_string(),
                payload: Bytes::from_str("0x1213").unwrap(),
                signature: Bytes::from_str("0x1213").unwrap(),
            },
            data: Bytes::from_str("0x1213").unwrap(),
        };
        assert_eq!(
            response.price_in_atomic_units(6),
            U256::from_dec_str(&1e6.to_string()).unwrap()
        );
    }

    #[test]
    fn price_in_keeps_six_digits_precision() {
        let response = ReservoirOracleResponse {
            price: 1.1234567,
            message: ReservoirOracleMessage {
                id: "".to_string(),
                payload: Bytes::from_str("0x1213").unwrap(),
                signature: Bytes::from_str("0x1213").unwrap(),
            },
            data: Bytes::from_str("0x1213").unwrap(),
        };
        assert_eq!(
            response.price_in_atomic_units(6),
            U256::from_dec_str(&1.123456e6.to_string()).unwrap()
        );
    }
}
