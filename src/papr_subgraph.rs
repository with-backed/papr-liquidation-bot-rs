use ethers::types::{Bytes, U256};
use graphql_client::{GraphQLQuery, Response};
use std::env;
use std::error::Error;
use String as BigInt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/paprSchema.graphql",
    query_path = "src/graphql/vaultsExceedingDebtPerCollateral.graphql"
)]
pub struct VaultsExceedingDebtPerCollateral;

pub async fn collateral_vaults_exceeding_debt_per_collateral(
    controller: &str,
    collateral: &str,
    debt_per_collateral: U256,
) -> Result<
    Vec<vaults_exceeding_debt_per_collateral::VaultsExceedingDebtPerCollateralVaults>,
    Box<dyn Error>,
> {
    let variables = vaults_exceeding_debt_per_collateral::Variables {
        controller: Some(controller.to_string()),
        collateral: Some(collateral.to_string()),
        debt_per_collateral: Some(debt_per_collateral.to_string()),
    };
    let request_body = VaultsExceedingDebtPerCollateral::build_query(variables);
    let client = reqwest::Client::new();
    let url = env::var("PAPR_SUBGRAPH_URL").expect("PAPR_SUBGRAPH_URL not set");
    let res = client.post(url).json(&request_body).send().await?;
    let response_body: Response<vaults_exceeding_debt_per_collateral::ResponseData> =
        res.json().await?;
    if let Some(errors) = response_body.errors {
        // error!("there are errors:");

        // for error in &errors {
        //     error!("{:?}", error);
        // }
    }
    let response_data: vaults_exceeding_debt_per_collateral::ResponseData =
        response_body.data.expect("missing response data");
    Ok(response_data.vaults)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/paprSchema.graphql",
    query_path = "src/graphql/collateralByController.graphql"
)]
pub struct CollateralByController;

// NOTE we do not filter on allowed collateral = true because there could be vaults
// with now disallowed collateral that need to be liquidated
pub async fn collateral(
    controller: &str,
) -> Result<Vec<collateral_by_controller::CollateralByControllerAllowedCollaterals>, Box<dyn Error>>
{
    let variables = collateral_by_controller::Variables {
        controller: Some(controller.to_string()),
    };
    let request_body = CollateralByController::build_query(variables);
    let client = reqwest::Client::new();
    let url = env::var("PAPR_SUBGRAPH_URL").expect("PAPR_SUBGRAPH_URL not set");
    let res = client.post(url).json(&request_body).send().await?;
    let response_body: Response<collateral_by_controller::ResponseData> = res.json().await?;
    let response_data: collateral_by_controller::ResponseData =
        response_body.data.expect("missing response data");
    Ok(response_data.allowed_collaterals)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/paprSchema.graphql",
    query_path = "src/graphql/allControllers.graphql"
)]
pub struct AllControllers;

impl all_controllers::AllControllersPaprControllers {
    pub fn max_ltv_as_u256(&self) -> U256 {
        return U256::from_dec_str(&self.max_ltv).expect("max_ltv_as_u256 error");
    }
}

pub async fn all_papr_controllers() -> Result<Vec<all_controllers::AllControllersPaprControllers>, Box<dyn Error>> {
    let variables = all_controllers::Variables;
    let request_body = AllControllers::build_query(variables);
    let client = reqwest::Client::new();
    let url = env::var("PAPR_SUBGRAPH_URL").expect("PAPR_SUBGRAPH_URL not set");
    let res = client.post(url).json(&request_body).send().await?;
    let response_body: Response<all_controllers::ResponseData> = res.json().await?;
    let response_data: all_controllers::ResponseData =
        response_body.data.expect("missing response data");
    Ok(response_data.papr_controllers)
}
