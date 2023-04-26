use serde::Deserialize;
use strum_macros::Display;

#[derive(Display)]
#[strum(serialize_all = "camelCase")]
enum OrderQueryParam {
    Collection,
    Limit,
    SortBy,
}

#[derive(Deserialize)]
pub struct BidsResponse {
    pub orders: Vec<Order>,
}

#[derive(Deserialize)]
pub struct Order {
    pub id: String,
    pub kind: String,
    pub price: Price,
    pub criteria: Criteria,
}

#[derive(Deserialize)]
pub struct Price {
    pub amount: Amount,
    pub net_amount: NetAmount,
    pub currency: Currency,
}

#[derive(Deserialize)]
pub struct Amount {
    pub usd: f64,
    native: f64,
}

#[derive(Deserialize)]
pub struct NetAmount {
    raw: String,
    pub decimal: f64,
    native: f64,
}

#[derive(Deserialize)]
pub struct Currency {
    contract: String,
    decimals: u8,
}

#[derive(Deserialize)]
pub struct Criteria {
    pub kind: String,
}

impl crate::reservoir::client::ReservoirClient {
    pub async fn bids(
        &self,
        collection: &str,
        limit: Option<u64>,
    ) -> Result<BidsResponse, eyre::Error> {
        let url = "/orders/bids/v5";
        let mut query: Vec<(String, String)> = vec![
            (
                OrderQueryParam::Collection.to_string(),
                collection.to_string(),
            ),
            (OrderQueryParam::SortBy.to_string(), "price".to_string()),
        ];
        if let Some(limit) = limit {
            query.push((OrderQueryParam::Limit.to_string(), limit.to_string()))
        }
        Ok(self.get::<_, BidsResponse>(&url, query).await?)
    }
}
