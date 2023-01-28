use crate::{
    papr_controller::{Collateral, OracleInfo, PaprController},
    papr_subgraph::client::GraphQLClient,
    papr_subgraph::queries::{
        all_controllers::AllControllersPaprControllers as Controller,
        all_controllers::AllControllersPaprControllersAllowedCollateral as SubgraphCollateral,
        vaults_exceeding_debt_per_collateral::VaultsExceedingDebtPerCollateralVaults as Vault,
    },
    reservoir::{
        client::ReservoirClient,
        oracle::OracleResponse,
        oracle::{self, PriceKind},
    },
};
use ethers::{
    prelude::abigen,
    types::{Address, U256},
};
use once_cell::sync::Lazy;
use std::collections::HashSet;

const SEVEN_DAYS_SECONDS: u32 = 604800;

// goerli
pub static WHITELIST: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut m = HashSet::new();
    m.insert("0x6df74b0653ba2b622d911ef5680d1776d850ace9");
    m.insert("0x9b74e0be4220317dc2f796d3ed865ccb72698020");
    m
});

pub async fn start_liquidations_for_whitelisted_controllers(
    reservoir: &ReservoirClient,
    graphql: &GraphQLClient,
) {
    let controllers = graphql.all_papr_controllers().await.unwrap();

    for controller in controllers {
        if WHITELIST.contains(&*controller.id) {
            start_liqudations_for_controller(controller, reservoir, graphql);
        }
    }
}

async fn start_liqudations_for_controller(
    controller: Controller,
    reservoir: &ReservoirClient,
    graphql: &GraphQLClient,
) -> Result<(), eyre::Error> {
    let controller_provider = PaprController::new(&controller.id);
    let target = controller_provider.new_target().await?;
    let max_ltv = controller.max_ltv_as_u256();
    for collateral in controller.allowed_collateral {
        let oracle_response = reservoir
            .max_collection_bid(
                &collateral.token.id,
                PriceKind::Twap,
                &controller.underlying.id,
                Some(SEVEN_DAYS_SECONDS),
            )
            .await?;
        let max = max_debt(oracle_response.price_in_atomic_units(6), max_ltv, target);
        let liquidatable_values = graphql
            .collateral_vaults_exceeding_debt_per_collateral(
                &controller.id,
                &collateral.token.id,
                max,
            )
            .await?;
        start_liquidations_for_vaults(liquidatable_values, oracle_response, &controller_provider)
            .await?;
    }
    Ok(())
}

async fn start_liquidations_for_vaults(
    vaults: Vec<Vault>,
    oracle_response: OracleResponse,
    controller_provider: &PaprController,
) -> Result<(), eyre::Error> {
    // need to pick a random piece of collateral
    for vault in vaults {
        let vault_addr = vault
            .account
            .to_string()
            .parse::<Address>()
            .expect("error parsing vault address");
        let collateral = Collateral {
            addr: vault
                .token
                .id
                .parse::<Address>()
                .expect("error parsing collateral address"),
            id: U256::from_dec_str(&vault.collateral.first().unwrap().id)
                .expect("error parsing collateral id"),
        };
        controller_provider
            .start_liquidation_auction(
                vault_addr,
                collateral,
                oracle_response.message.as_contract_oracle_info()?,
            )
            .await?;
    }
    Ok(())
}

fn max_debt(collateral_value_underlying: U256, max_ltv: U256, target: U256) -> U256 {
    return collateral_value_underlying
        .checked_mul(max_ltv)
        .expect("Max debt multiplication overflow")
        .checked_div(target)
        .expect("Max debt divide by 0");
}

#[cfg(test)]
mod tests {
    use crate::start::max_debt;
    use ethers::types::U256;

    #[test]
    fn max_debt_correctly_computes() {
        // worth 1 USDC
        let value = u256_from_str("10").pow(u256_from_str(&"6"));
        // 50%
        let (max_ltv, _) = u256_from_str(&"10")
            .pow(u256_from_str(&"17"))
            .overflowing_mul(u256_from_str(&"5"));
        // 2 USDC
        let (papr_price, _) = u256_from_str(&"10")
            .pow(u256_from_str(&"6"))
            .overflowing_mul(u256_from_str(&"2"));
        let result = max_debt(value, max_ltv, papr_price);
        // (1 * .5)/2 = 0.25 => 0.25e18, papr has 18 decimals
        let (expected, _) = u256_from_str(&"10")
            .pow(u256_from_str(&"16"))
            .overflowing_mul(u256_from_str(&"25"));
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Max debt multiplication overflow")]
    fn max_debt_panics_if_multiplication_overflows() {
        max_debt(U256::max_value(), u256_from_str(&"5"), u256_from_str(&"5"));
    }

    #[test]
    #[should_panic(expected = "Max debt divide by 0")]
    fn max_debt_panics_if_division_underflows() {
        max_debt(
            u256_from_str(&"10"),
            u256_from_str(&"5"),
            u256_from_str(&"0"),
        );
    }

    fn u256_from_str(i: &str) -> U256 {
        return U256::from_dec_str(i).unwrap();
    }
}
