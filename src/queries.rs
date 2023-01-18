use ethers::types::{Bytes, U256};
use graphql_client::GraphQLQuery;
use String as BigInt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/paprSchema.graphql",
    query_path = "src/graphql/vaultsExceedingDebtPerCollateral.graphql"
)]
pub struct VaultsExceedingDebtPerCollateral;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/paprSchema.graphql",
    query_path = "src/graphql/collateralByController.graphql"
)]
pub struct CollateralByController;

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
