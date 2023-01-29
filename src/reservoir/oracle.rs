use ethers::{
    types::{Bytes, Signature, U256},
    utils::format_bytes32_string,
};
use serde::Deserialize;
use strum_macros::Display;

use crate::papr_controller;

#[derive(Display)]
#[strum(serialize_all = "camelCase")]
pub enum PriceKind {
    Upper,
    Lower,
    Twap,
    Spot,
}

#[derive(Display)]
#[strum(serialize_all = "camelCase")]
enum OracleQueryParam {
    Kind,
    Currency,
    TwapSeconds,
    Collection,
}

#[derive(Deserialize)]
pub struct OracleResponse {
    pub price: f64,
    pub message: OracleMessage,
}

#[derive(Deserialize)]
pub struct OracleMessage {
    pub id: String,
    pub payload: Bytes,
    pub timestamp: u64,
    pub signature: Bytes,
}

impl OracleResponse {
    pub fn price_in_atomic_units(&self, decimals: u8) -> Result<U256, eyre::Error> {
        let one = U256::from_dec_str("10")?.pow(U256::from_dec_str(&decimals.to_string())?);
        // scalar to prevent loss of precison when converting to atomic
        let scalar = 1e6;
        let u256_scalar = U256::from_dec_str(&scalar.to_string())?;
        let scaled_price = (self.price * scalar).floor();
        let u256_scaled_price = U256::from_dec_str(&scaled_price.to_string())?;
        let result = one
            .checked_mul(u256_scaled_price)
            .ok_or(eyre::eyre!("price_atomic overflow"))?
            .checked_div(u256_scalar)
            .ok_or(eyre::eyre!("price_atomic division error"))?;
        Ok(result)
    }
}

impl OracleMessage {
    pub fn as_contract_oracle_info(&self) -> Result<papr_controller::OracleInfo, eyre::Error> {
        let signature_struct = self.signature.to_string().parse::<Signature>()?;
        let info = papr_controller::OracleInfo {
            message: papr_controller::Message {
                id: format_bytes32_string(&self.id)?,
                payload: self.payload.clone(),
                timestamp: U256::from_dec_str(&self.timestamp.to_string())?,
                signature: self.signature.clone(),
            },
            sig: papr_controller::Sig {
                r: format_bytes32_string(&signature_struct.r.to_string())?,
                s: format_bytes32_string(&signature_struct.s.to_string())?,
                v: signature_struct.v as u8,
            },
        };
        Ok(info)
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
            (
                OracleQueryParam::Currency.to_string(),
                quote_currency.to_string(),
            ),
        ];
        if let Some(twap_seconds) = twap_seconds {
            query.push((
                OracleQueryParam::TwapSeconds.to_string(),
                twap_seconds.to_string(),
            ))
        }
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
                timestamp: 1,
            },
        };
        assert_eq!(
            response.price_in_atomic_units(6).unwrap(),
            U256::from_dec_str(&1e6.to_string()).unwrap()
        );
    }

    #[test]
    fn price_in_atomic_units_keeps_six_digits_precision() {
        let response = OracleResponse {
            price: 1.1234567,
            message: OracleMessage {
                id: "".to_string(),
                payload: Bytes::from_str("0x1213").unwrap(),
                signature: Bytes::from_str("0x1213").unwrap(),
                timestamp: 1,
            },
        };
        assert_eq!(
            response.price_in_atomic_units(6).unwrap(),
            U256::from_dec_str(&1.123456e6.to_string()).unwrap()
        );
    }

    // TODO test as_contract_oracle_info
}
