use crate::{
    papr_controller::{Collateral, PaprController},
    papr_subgraph::client::GraphQLClient,
    papr_subgraph::queries::{
        all_controllers::AllControllersPaprControllers as Controller,
        vaults_exceeding_debt_per_collateral::VaultsExceedingDebtPerCollateralVaults as Vault,
    },
    reservoir::{client::ReservoirClient, oracle::OracleResponse, oracle::PriceKind},
};
use ethers::types::{Address, U256};
use once_cell::sync::Lazy;
use std::{
    collections::HashSet,
    env,
    time::{SystemTime, UNIX_EPOCH},
};

const SEVEN_DAYS_SECONDS: u32 = 604800;
const TWO_DAYS_SECONDS: u64 = 172800;
static DISABLE_EXECUTE_START_ACTION: Lazy<bool> =
    Lazy::new(|| env::var("DISABLE_EXECUTE_START_ACTION").unwrap_or("false".to_string()) == "true");


pub static WHITELIST: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut m = HashSet::new();
    // goerli paprHero
    m.insert("0xd0a830278773282bbf635fd8e47b2447f1e9fe86");
    // paprMeme
    m.insert("0x3b29c19ff2fcea0ff98d0ef5b184354d74ea74b0");
    m
});

pub async fn start_liquidations_for_whitelisted_controllers(
    reservoir: &ReservoirClient,
    graphql: &GraphQLClient,
) -> Result<(), eyre::Error> {
    let controllers = graphql.all_papr_controllers().await?;

    for controller in controllers {
        if WHITELIST.contains(&*controller.id) {
            println!("starting for {}", controller.id);
            println!("quote currency {}", controller.underlying.id);
            start_liqudations_for_controller(controller, reservoir, graphql).await?;
        }
    }
    Ok(())
}

async fn start_liqudations_for_controller(
    controller: Controller,
    reservoir: &ReservoirClient,
    graphql: &GraphQLClient,
) -> Result<(), eyre::Error> {
    let controller_provider = PaprController::new(&controller.id)?;
    let target = controller_provider.new_target().await?;
    let max_ltv = controller.max_ltv_as_u256()?;
    println!("target {}", target);
    println!("max_ltv {}", max_ltv);
    for collateral in controller.allowed_collateral {
        println!("fetching price for collateral {}", collateral.token.id);
        let oracle_response_result = reservoir
            .max_collection_bid(
                &collateral.token.id,
                PriceKind::Twap,
                &controller.underlying.id,
                Some(SEVEN_DAYS_SECONDS),
            )
            .await;
        if let Some(err) = oracle_response_result.as_ref().err() {
            // mainly to handle goerli issues
            println!("oracle err: {}", err);
            continue;
        }
        let oracle_response = oracle_response_result?;
        println!(
            "price {}",
            oracle_response
                .price_in_atomic_units(controller.underlying.decimals as u8)
                .unwrap()
        );
        let max = max_debt(
            oracle_response.price_in_atomic_units(controller.underlying.decimals as u8)?,
            max_ltv,
            target,
        )?;
        println!("max debt {}", max);
        let liquidatable_vaults = graphql
            .collateral_vaults_exceeding_debt_per_collateral(
                &controller.id,
                &collateral.token.id,
                max,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)?
                    .as_secs()
                    .checked_sub(TWO_DAYS_SECONDS)
                    .ok_or(eyre::eyre!("timestamp error"))?,
            )
            .await?;
        println!("found {} liquidatable vaults", liquidatable_vaults.len());
        start_liquidations_for_vaults(liquidatable_vaults, oracle_response, &controller_provider)
            .await?;
        // TODO should store auction IDs of started auctions so that we can remember we have a discount
    }
    Ok(())
}

async fn start_liquidations_for_vaults(
    vaults: Vec<Vault>,
    oracle_response: OracleResponse,
    controller_provider: &PaprController,
) -> Result<(), eyre::Error> {
    // have to sleep otherwise timestamp will be > block.timestamp
    std::thread::sleep(std::time::Duration::from_secs(10));
    for vault in vaults {
        let vault_addr = vault.account.to_string().parse::<Address>()?;
        let collateral = Collateral {
            addr: vault.token.id.parse::<Address>()?,
            id: U256::from_dec_str(
                &vault
                    .collateral
                    .first()
                    // should not be possible happen but just incase :)
                    .ok_or(eyre::eyre!("no collateral in vault"))?
                    .token_id,
            )?,
        };
        println!(
            "liquidating collateral {} id {} of account {}",
            vault.token.id, collateral.id, vault.account
        );

        if *DISABLE_EXECUTE_START_ACTION {
            continue;
        }

        controller_provider
            .start_liquidation_auction(
                vault_addr,
                collateral,
                oracle_response.message.as_contract_oracle_info()?,
            )
            .await?;
        println!("liquidation successful");
    }
    Ok(())
}

fn max_debt(
    collateral_value_underlying: U256,
    max_ltv: U256,
    target: U256,
) -> Result<U256, eyre::Error> {
    let max = collateral_value_underlying
        .checked_mul(max_ltv)
        .ok_or(eyre::eyre!("max_debt multiplication overflow"))?
        .checked_div(target)
        .ok_or(eyre::eyre!("max_debt divide by 0"))?;
    Ok(max)
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
        let result = max_debt(value, max_ltv, papr_price).unwrap();
        // (1 * .5)/2 = 0.25 => 0.25e18, papr has 18 decimals
        let (expected, _) = u256_from_str(&"10")
            .pow(u256_from_str(&"16"))
            .overflowing_mul(u256_from_str(&"25"));
        assert_eq!(result, expected);
    }

    #[test]
    fn max_debt_panics_if_multiplication_overflows() {
        let result = max_debt(U256::max_value(), u256_from_str(&"5"), u256_from_str(&"5"));
        assert_eq!(
            "max_debt multiplication overflow",
            result.err().unwrap().to_string()
        );
    }

    #[test]
    fn max_debt_panics_if_division_underflows() {
        let result = max_debt(
            u256_from_str(&"10"),
            u256_from_str(&"5"),
            u256_from_str(&"0"),
        );
        assert_eq!("max_debt divide by 0", result.err().unwrap().to_string());
    }

    fn u256_from_str(i: &str) -> U256 {
        return U256::from_dec_str(i).unwrap();
    }
}
