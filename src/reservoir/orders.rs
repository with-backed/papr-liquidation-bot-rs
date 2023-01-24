use serde::Deserialize;
use strum_macros::Display;

#[strum(serialize_all = "camelCase")]
#[derive(Display)]
enum OrderQueryParam {
    Collection,
    Limit,
    SortBy
}

#[derive(Deserialize)]
pub struct BidsResponse {
    pub orders: Vec<Order>,
}

#[derive(Deserialize)]
pub struct Order {
    pub price: Price,
    pub criteria: Criteria,
}

#[derive(Deserialize)]
pub struct Price {
    pub amount: Amount,
}

#[derive(Deserialize, PartialEq, PartialOrd)]
pub struct Amount {
    pub usd: f64,
}

// #[derive(Deserialize)]
// pub enum CriteriaKind {
//     Collection
// }

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
        let mut query: Vec<(String, String)> = vec![(
            OrderQueryParam::Collection.to_string(),
            collection.to_string(),
        ), (OrderQueryParam::SortBy.to_string(), "price".to_string())];
        if let Some(limit) = limit {
            query.push((OrderQueryParam::Limit.to_string(), limit.to_string()))
        }
        Ok(self.get::<_, BidsResponse>(&url, query).await?)
    }
}
