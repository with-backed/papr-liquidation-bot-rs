use crate::{
    papr_subgraph::client::GraphQLClient,
    papr_subgraph::queries::{
        all_controllers::AllControllersPaprControllers as Controller,
        ongoing_auctions_by_controller::OngoingAuctionsByControllerAuctions as SubgraphAuction,
    },
    reservoir::{client::ReservoirClient},
};
use ethers::{
    types::U256,
    utils::{format_units, parse_units},
};
use once_cell::sync::Lazy;
use std::{
    collections::HashSet,
    time::{SystemTime, UNIX_EPOCH},
};

// goerli
pub static WHITELIST: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut m = HashSet::new();
    m.insert("0xd0a830278773282bbf635fd8e47b2447f1e9fe86");
    m
});

pub async fn purchase_auctions_from_whitelisted_controllers(
    reservoir: &ReservoirClient,
    graphql: &GraphQLClient,
) -> Result<(), eyre::Error> {
    let controllers = graphql.all_papr_controllers().await.unwrap();

    for controller in controllers {
        if WHITELIST.contains(&*controller.id) {
            println!("starting for {}", controller.id);
            println!("quote currency {}", controller.underlying.id);
            arb_auctions_for_controller(controller, reservoir, graphql).await?;
        }
    }
    Ok(())
}

async fn arb_auctions_for_controller(
    controller: Controller,
    reservoir: &ReservoirClient,
    graphql: &GraphQLClient,
) -> Result<(), eyre::Error> {
    let auctions = graphql.ongoing_auctions(&controller.id);

    Ok(())
}

fn current_price(auction: SubgraphAuction) -> Result<U256, eyre::Error> {
    let start_price = format_units(
        U256::from_dec_str(&auction.start_price)?,
        auction.payment_asset.decimals as u32,
    )?
    .parse::<f64>()?;
    let decay: f64 = format_units(
        U256::from_dec_str(&auction.per_period_decay_percent_wad)?,
        18,
    )?
    .parse::<f64>()?;

    let elapsed_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        .checked_sub(auction.start.timestamp as u64)
        .ok_or(eyre::eyre!("timestamp error"))?;
    println!("elapsed {}", elapsed_time);

    let period_ratio = elapsed_time as f64 / auction.seconds_in_period.parse::<f64>()?;
    let percent_remaining = 1 as f64 - decay;
    let multiplier = percent_remaining.powf(period_ratio);
    let price = multiplier * start_price;
    // actual 270459742027958058
    // what we get 270459742027958030

    Ok(parse_units(price, auction.payment_asset.decimals as u32)?.into())
}

// get all ongoing auctions
// compute current price for each
// convert papr price -> underlying price using uniswap
// (probably just use tick and later check slippage inclusive price?)
// try to find bids above that price
// swap -> on callback, purchase NFT and sell -> send funds owed back to Uniswap

#[cfg(test)]
mod tests {
    use crate::{
        papr_subgraph::queries::{
            ongoing_auctions_by_controller,
            ongoing_auctions_by_controller::OngoingAuctionsByControllerAuctions as SubgraphAuction,
        },
        purchase::current_price
    };
    use ethers::types::{Bytes, U256};
    use std::str::FromStr;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn current_price_computes_correctly() {
        use ongoing_auctions_by_controller::*;
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .checked_sub(4060)
            .unwrap();

        let auction = SubgraphAuction {
            id: "84921541788424467252204917851547836642895224820573188317255928948100032289381"
                .into(),
            nft_owner: Bytes::from_str("0xbc3ed6b537f2980e66f396fe14210a56ba3f72c4").unwrap(),
            started_by: Bytes::from_str("0xe89cb2053a04daf86abaa1f4bc6d50744e57d39e").unwrap(),
            auction_asset_id: "10".into(),
            auction_asset_contract: OngoingAuctionsByControllerAuctionsAuctionAssetContract {
                id: "0x79ab709dadc05cd2c0f7322bc7e3d70d2550942c".into(),
            },
            vault: OngoingAuctionsByControllerAuctionsVault {
                account: Bytes::from_str("0xbc3ed6b537f2980e66f396fe14210a56ba3f72c4").unwrap(),
            },
            seconds_in_period: "86400".into(),
            start_price: "286202279878974014".into(),
            per_period_decay_percent_wad: "700000000000000000".into(),
            payment_asset: OngoingAuctionsByControllerAuctionsPaymentAsset {
                id: "0x047067ad8b5bf37bb93bb61af73f73fd9f8ca5af".into(),
                decimals: 18,
            },
            start: OngoingAuctionsByControllerAuctionsStart {
                timestamp: start_time as i64,
            },
        };

        // computed from solidity = 270459742027958058
        // 25 difference which is sub second. For scale, a second later the price is 3T lower
        assert_eq!(
            U256::from_dec_str("270459742027958030").unwrap(),
            current_price(auction).unwrap()
        );
    }
}
