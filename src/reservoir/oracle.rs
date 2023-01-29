use ethers::{
    types::{Bytes, Signature, U256},
    utils::hex::{FromHex, ToHex},
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
                id: <[u8; 32]>::from_hex(&self.id[2..])?,
                payload: self.payload.clone(),
                timestamp: self.timestamp.into(),
                signature: self.signature.clone(),
            },
            sig: papr_controller::Sig {
                r: signature_struct.r.into(),
                s: signature_struct.s.into(),
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
                id: "0x1213".to_string(),
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
                id: "0x1213".to_string(),
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

    #[test]
    fn as_contract_oracle_info_converts_values_correctly() {
        let timestamp = 1674959723;
        let message = OracleMessage {
            id: "0xc8c8fbbc02b65d74cc2266c31a5d773f5b73983830d7757ba80a14175f0fb189".to_string(),
            payload: Bytes::from_str("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000381d2cfbb2fe58000").unwrap(),
            signature: Bytes::from_str("0xcb14779852fb3cebb98cb5ee807051d162396108b65822a2c52147d739a100fb00185d38f2a4542e2e90c7f6493fad340333b5e1f9c2f20c2ed1956a56340e021c").unwrap(),
            timestamp: timestamp,
        };
        let info = message.as_contract_oracle_info().unwrap();
        assert_eq!(
            info.message.timestamp,
            U256::from_dec_str(&timestamp.to_string()).unwrap()
        );
        assert_eq!(info.message.payload, message.payload);
        assert_eq!(info.message.signature, message.signature);
        // manually checked the below in solidity
        assert_eq!(
            info.message.id,
            [
                200, 200, 251, 188, 2, 182, 93, 116, 204, 34, 102, 195, 26, 93, 119, 63, 91, 115,
                152, 56, 48, 215, 117, 123, 168, 10, 20, 23, 95, 15, 177, 137
            ]
        );
        assert_eq!(
            info.sig.r,
            [
                203, 20, 119, 152, 82, 251, 60, 235, 185, 140, 181, 238, 128, 112, 81, 209, 98, 57,
                97, 8, 182, 88, 34, 162, 197, 33, 71, 215, 57, 161, 0, 251
            ]
        );
        assert_eq!(
            info.sig.s,
            [
                0, 24, 93, 56, 242, 164, 84, 46, 46, 144, 199, 246, 73, 63, 173, 52, 3, 51, 181,
                225, 249, 194, 242, 12, 46, 209, 149, 106, 86, 52, 14, 2
            ]
        );
        assert_eq!(info.sig.v, 28);
    }
}
