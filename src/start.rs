use crate::{
    papr_controller,
    papr_subgraph::client::GraphQLClient,
    papr_subgraph::queries::{
        all_controllers::AllControllersPaprControllers as Controller,
        collateral_by_controller::CollateralByControllerAllowedCollaterals as Collateral,
        vaults_exceeding_debt_per_collateral::VaultsExceedingDebtPerCollateralVaults as Vault,
    },
    reservoir::oracle::OracleResponse,
};
use ethers::{prelude::abigen, types::U256};
use std::collections::HashSet;

// goerli
lazy_static! {
    static ref WHITELIST: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.insert("0x6df74b0653ba2b622d911ef5680d1776d850ace9");
        m.insert("0x9b74e0be4220317dc2f796d3ed865ccb72698020");
        m
    };
}

pub async fn start_liquidations_for_whitelisted_controllers() {
    let controllers = GraphQLClient::default()
        .all_papr_controllers()
        .await
        .unwrap();

    for controller in controllers {
        if WHITELIST.contains(&*controller.id) {
            start_liqudations(controller);
        }
    }
}

async fn start_liqudations(controller: Controller) {
    // papr_controller.new_target();
    // fetch target for controller
    // fetch oracle info
    // let controller = PaprController::new(controller.id.parse::<Address>()?, )
}

async fn liquidatable_vaults(
    client: &GraphQLClient,
    controller: Controller,
    collateral: Collateral,
    target: U256,
    oracle_info: OracleResponse,
) -> Result<Vec<Vault>, eyre::Error> {
    let price_atomic = oracle_info.price_in_atomic_units(controller.underlying.decimals as u8);
    let max_debt = max_debt(price_atomic, controller.max_ltv_as_u256(), target);
    let res = client
        .collateral_vaults_exceeding_debt_per_collateral(
            &controller.id,
            &collateral.token.id,
            max_debt,
        )
        .await
        .map_err(|err| {
            eyre::eyre!("error fetching vaults exeeding debt per collateral: {err:?}")
        })?;

    return Ok(res);
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
