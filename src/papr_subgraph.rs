use ethers::types::{U256, Bytes};
use String as BigInt;
use graphql_client::{GraphQLQuery, Response};
use std::env;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/paprSchema.graphql",
    query_path = "src/graphql/vaultsByCollateralAndDebtPerCollateral.graphql"
)]
pub struct VaultsByCollateralAndDebtPerCollateral;

pub async fn vaults_by_collateral_and_debt_per_collateral(collateral: &str, debt_per_collateral: U256) -> Result<vaults_by_collateral_and_debt_per_collateral::ResponseData, Box<dyn Error>> {
    let variables = vaults_by_collateral_and_debt_per_collateral::Variables {
        collateral: Some(collateral.to_string()),
        debt_per_collateral: Some(debt_per_collateral.to_string())
    };
    let request_body = VaultsByCollateralAndDebtPerCollateral::build_query(variables);
    let client = reqwest::Client::new();
    let url = env::var("PAPR_SUBGRAPH_URL").expect("PAPR_SUBGRAPH_URL not set");
    let mut res = client.post(url).json(&request_body).send().await?;
    let response_body: Response<vaults_by_collateral_and_debt_per_collateral::ResponseData> = res.json().await?;
    let response_data: vaults_by_collateral_and_debt_per_collateral::ResponseData = response_body.data.expect("missing response data");
    Ok(response_data)
}