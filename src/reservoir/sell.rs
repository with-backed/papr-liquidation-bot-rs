use serde::Deserialize;
use strum_macros::Display;
use ethers::types::U256;

#[derive(Display)]
#[strum(serialize_all = "camelCase")]
enum QueryParams {
    Token,
    Taker,
}

#[derive(Deserialize)]
pub struct Response {
    pub steps: Vec<Step>,
}

#[derive(Deserialize)]
pub struct Step {
    pub items: Vec<Item>
}

#[derive(Deserialize)]
pub struct Item {
    status: String, 
    data: ItemData
}

#[derive(Deserialize)]
pub struct ItemData {
    from: String, 
    to: String, 
    data: String
}

impl crate::reservoir::client::ReservoirClient {
    pub async fn sell_token(
        &self,
        collection: &str,
        token_id: U256,
        seller: String
    ) -> Result<Response, eyre::Error> {
        let url = "/execute/sell/v6";
        let query: Vec<(String, String)> = vec![
            (
                QueryParams::Token.to_string(),
                format!("{}:{}", collection, token_id.to_string()),
            ),
            (
                QueryParams::Taker.to_string(),
                seller,
            ),
        ];
        Ok(self.get::<_, Response>(&url, query).await?)
    }
}