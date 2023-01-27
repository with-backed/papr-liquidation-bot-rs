use ethers::types::{Bytes, U256};
use serde::Deserialize;
use strum_macros::Display;

#[strum(serialize_all = "camelCase")]
#[derive(Display)]
pub enum PriceKind {
    Upper,
    Lower,
    Twap,
    Spot,
}

#[strum(serialize_all = "camelCase")]
#[derive(Display)]
enum OracleQueryParam {
    Kind,
    Currency,
    TwapSeconds,
    Collection,
}

#[derive(Deserialize)]
pub struct OracleResponse {
    pub price: f64,
    message: OracleMessage,
}

#[derive(Deserialize)]
pub struct OracleMessage {
    id: String,
    payload: Bytes,
    signature: Bytes,
}

impl OracleResponse {
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

impl crate::reservoir::client::ReservoirClient {
    pub async fn max_collection_bid(
        &self,
        collection: &str,
        price_kind: PriceKind,
        quote_currency: &str,
        twap_seconds: Option<u32>,
    ) -> Result<OracleResponse, eyre::Error> {
        let url = "/oracle/collections/top-bid/v2";
        let mut query: Vec<(String, String)> = vec![
            (
                OracleQueryParam::Collection.to_string(),
                collection.to_string(),
            ),
            (OracleQueryParam::Kind.to_string(), price_kind.to_string()),
            (OracleQueryParam::Currency.to_string(), quote_currency.to_string()),
        ];
        if let Some(twap_seconds) = twap_seconds {
            query.push((OracleQueryParam::TwapSeconds.to_string(), twap_seconds.to_string()))
        }
        // let query = [
        //     (
        //         OracleQueryParam::Collection.to_string(),
        //         collection.to_string(),
        //     ),
        //     (OracleQueryParam::Kind.to_string(), price_kind.to_string()),
        //     (
        //         OracleQueryParam::TwapSeconds.to_string(),
        //         twap_seconds.unwrap_or(0).to_string(),
        //     ),
        // ];
        Ok(self.get::<_, OracleResponse>(&url, query).await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::reservoir::oracle::OracleMessage;
    use crate::reservoir::oracle::OracleResponse;
    use ethers::types::{Bytes, U256};
    use std::str::FromStr;

    #[test]
    fn price_in_atomic_units_computes_correctly() {
        let response = OracleResponse {
            price: 1.0,
            message: OracleMessage {
                id: "".to_string(),
                payload: Bytes::from_str("0x1213").unwrap(),
                signature: Bytes::from_str("0x1213").unwrap(),
            },
        };
        assert_eq!(
            response.price_in_atomic_units(6),
            U256::from_dec_str(&1e6.to_string()).unwrap()
        );
    }

    #[test]
    fn price_in_keeps_six_digits_precision() {
        let response = OracleResponse {
            price: 1.1234567,
            message: OracleMessage {
                id: "".to_string(),
                payload: Bytes::from_str("0x1213").unwrap(),
                signature: Bytes::from_str("0x1213").unwrap(),
            },
        };
        assert_eq!(
            response.price_in_atomic_units(6),
            U256::from_dec_str(&1.123456e6.to_string()).unwrap()
        );
    }
}
