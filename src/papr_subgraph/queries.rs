use ethers::types::{Bytes, U256};
use graphql_client::GraphQLQuery;
use String as BigInt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/papr_subgraph/graphql/paprSchema.graphql",
    query_path = "src/papr_subgraph/graphql/vaultsExceedingDebtPerCollateral.graphql"
)]
pub struct VaultsExceedingDebtPerCollateral;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/papr_subgraph/graphql/paprSchema.graphql",
    query_path = "src/papr_subgraph/graphql/collateralByController.graphql"
)]
pub struct CollateralByController;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/papr_subgraph/graphql/paprSchema.graphql",
    query_path = "src/papr_subgraph/graphql/allControllers.graphql"
)]
pub struct AllControllers;

impl all_controllers::AllControllersPaprControllers {
    pub fn max_ltv_as_u256(&self) -> Result<U256, eyre::Error> {
        Ok(U256::from_dec_str(&self.max_ltv)?)
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/papr_subgraph/graphql/paprSchema.graphql",
    query_path = "src/papr_subgraph/graphql/ongoingAuctionsByController.graphql"
)]
pub struct OngoingAuctionsByController;
