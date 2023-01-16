use ethers::types::U256;
use tokio::runtime::Runtime;

mod reservoir;
mod papr_subgraph;
use crate::{
    reservoir::{max_collection_bid, PriceKind},
    papr_subgraph::vaults_by_collateral_and_debt_per_collateral
};

fn main() {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async {
        match max_collection_bid(
            "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d",
            PriceKind::Lower,
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
            Some(1),
        )
        .await
        {
            Ok(body) => {
                println!("Response body: {}", body.price);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    });
    runtime.block_on(async {
        match vaults_by_collateral_and_debt_per_collateral(
            "0x8232c5fd480c2a74d2f25d3362f262ff3511ce49",
            U256::from_dec_str("246987190433877370000").unwrap(),
        )
        .await
        {
            Ok(body) => {
                println!("Response body: {}", body.vaults.first().unwrap().account);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    });
}

// fetch all vaults
// group by collateral, or iterate through allowed collateral and fetch?
// then get the price for that collateral
// get the max debt
// then compare that to the max debt per vault (we probably don't need to fetch vault's till the step)
// then we can liquidate

// max debt that calls for target
// get max debt for each collateral type
// compare this to debt_per_nft for all the vaults with that type
// find vaults in violation

// fn get controller allowed collateral

// fn collateral price

// fetch vaults exceeding

// liquidate

// https://api.reservoir.tools/oracle/collections/top-bid/v1?collection=0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d

/*
{"price":64.6,"message":{"id":"0x9dbcb059816093c07c2ec708ca46dbfdc9f01111f181854c0f735bc435678045","payload":"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000380814dbc0a3c0000","timestamp":1673832343,"signature":"0x391555b9c392bb7cf1414fafc391b6fab4fad9acc5d41db75fc07c7f42ac97896daee4f3c9557da4fc9f7bf9c514f1589c935cc1d240c6bd3580561a076a89ac1c"}}
*/

// bytes32 id;
//         bytes payload;
//         // The UNIX timestamp when the message was signed by the oracle
//         uint256 timestamp;
//         // ECDSA signature or EIP-2098 compact signature
//         bytes signature;

fn max_debt(collateral_value_underlying: U256, max_ltv: U256, papr_price_underlying: U256) -> U256 {
    return collateral_value_underlying
        .checked_mul(max_ltv)
        .expect("Max debt multiplication overflow")
        .checked_div(papr_price_underlying)
        .expect("Max debt divide by 0");
}

#[cfg(test)]
mod tests {
    use crate::max_debt;
    use ethers::types::U256;

    #[test]
    fn max_debt_correctly_computes() {
        // worth 1 USDC
        let value = u256_from_str(&"10").pow(u256_from_str(&"6"));
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
