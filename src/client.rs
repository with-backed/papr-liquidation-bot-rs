use ethers::types::{Bytes, U256};
use graphql_client::{GraphQLQuery, QueryBody, Response};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;
use std::error::Error;
use String as BigInt;

use crate::queries::{
    all_controllers, collateral_by_controller, vaults_exceeding_debt_per_collateral,
    AllControllers, CollateralByController, VaultsExceedingDebtPerCollateral,
};

static SUBGRAPH_URL: Lazy<String> =
    Lazy::new(|| env::var("PAPR_SUBGRAPH_URL").expect("PAPR_SUBGRAPH_URL not set"));

pub struct GraphQLClient {
    client: reqwest::Client,
}

impl Default for GraphQLClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl GraphQLClient {
    pub async fn collateral_vaults_exceeding_debt_per_collateral(
        &self,
        controller: &str,
        collateral: &str,
        debt_per_collateral: U256,
    ) -> Result<
        Vec<vaults_exceeding_debt_per_collateral::VaultsExceedingDebtPerCollateralVaults>,
        eyre::Error,
    > {
        use vaults_exceeding_debt_per_collateral::*;
        let variables = Variables {
            controller: Some(controller.to_string()),
            collateral: Some(collateral.to_string()),
            debt_per_collateral: Some(debt_per_collateral.to_string()),
        };
        let query = VaultsExceedingDebtPerCollateral::build_query(variables);
        Ok(self.query::<_, ResponseData>(query).await?.vaults)
    }

    // NOTE we do not filter on allowed collateral = true because there could be vaults
    // with now disallowed collateral that need to be liquidated
    pub async fn collateral(
        &self,
        controller: &str,
    ) -> Result<Vec<collateral_by_controller::CollateralByControllerAllowedCollaterals>, eyre::Error>
    {
        use collateral_by_controller::*;
        let variables = Variables {
            controller: Some(controller.to_string()),
        };
        let query = CollateralByController::build_query(variables);
        Ok(self
            .query::<_, ResponseData>(query)
            .await?
            .allowed_collaterals)
    }

    pub async fn all_papr_controllers(
        &self,
    ) -> Result<Vec<all_controllers::AllControllersPaprControllers>, eyre::Error> {
        use all_controllers::*;
        let query = AllControllers::build_query(Variables);
        Ok(self.query::<_, ResponseData>(query).await?.papr_controllers)
    }

    async fn query<V: Serialize, D: DeserializeOwned>(
        &self,
        query: QueryBody<V>,
    ) -> Result<D, eyre::Error> {
        let response = self.client.post(&*SUBGRAPH_URL).json(&query).send().await?;
        let body: Response<D> = response.json().await?;
        body.data
            .ok_or(eyre::eyre!("missing response data for query"))
    }
}
