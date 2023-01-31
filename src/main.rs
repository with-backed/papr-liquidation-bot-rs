mod papr_controller;
mod papr_subgraph;
mod provider;
mod reservoir;
mod start;
use crate::{
    papr_subgraph::client::GraphQLClient,
    reservoir::{client::ReservoirClient, oracle::PriceKind},
    start::start_liquidations_for_whitelisted_controllers,
};

#[tokio::main]
async fn main() -> Result<(), eyre::Error> {
    // IGNORE FOR NOW

    // let oracle_info = ReservoirClient::default()
    //     .max_collection_bid(
    //         "0x8d04a8c79ceb0889bdd12acdf3fa9d207ed3ff63",
    //         PriceKind::Lower,
    //         "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    //         Some(1),
    //     )
    //     .await?;
    // println!("Oracle info price: {}", oracle_info.price);

    let graphql = GraphQLClient::default();
    let reservoir = ReservoirClient::default();

    let x = start_liquidations_for_whitelisted_controllers(&reservoir, &graphql).await;
    if let Some(err) = x.err() {
        println!("{}", err);
    }

    // let bids = ReservoirClient::default()
    //     .bids("0x42069ABFE407C60cf4ae4112bEDEaD391dBa1cdB", None)
    //     .await?;
    // println!(
    //     "Oracle info price: {}",
    //     bids.orders.first().unwrap().price.amount.usd
    // );
    // println!("Oracle info price: {}", bids.orders.len());
    // println!("Toadz");
    // let count =
    //     collection_bids_gt_percent_of_top_bid("0x1CB1A5e65610AEFF2551A50f76a87a7d3fB649C6", 0.5)
    //         .await;
    // println!(
    //     "bid count as % of total supply: {}%",
    //     ((count.unwrap() as f64) / 7013.0 * 100.0).floor()
    // );
    // println!("");

    // println!("DickButts");
    // let count =
    //     collection_bids_gt_percent_of_top_bid("0x42069ABFE407C60cf4ae4112bEDEaD391dBa1cdB", 0.5)
    //         .await;
    // println!(
    //     "bid count as % of total supply: {}%",
    //     ((count.unwrap() as f64) / 5198.0 * 100.0).floor()
    // );
    // println!("");

    // println!("Pudgy Penguins");
    // let count =
    //     collection_bids_gt_percent_of_top_bid("0xBd3531dA5CF5857e7CfAA92426877b022e612cf8", 0.5)
    //         .await;
    // println!(
    //     "bid count as % of total supply: {}%",
    //     ((count.unwrap() as f64) / 8888.0 * 100.0).floor()
    // );
    // println!("");

    // println!("mfers");
    // let count =
    //     collection_bids_gt_percent_of_top_bid("0x79FCDEF22feeD20eDDacbB2587640e45491b757f", 0.5)
    //         .await;
    // println!(
    //     "bid count as % of total supply: {}%",
    //     ((count.unwrap() as f64) / 10021.0 * 100.0).floor()
    // );
    // println!("");

    // println!("Wassies");
    // let count =
    //     collection_bids_gt_percent_of_top_bid("0x1D20A51F088492A0f1C57f047A9e30c9aB5C07Ea", 0.5)
    //         .await;
    // println!(
    //     "bid count as % of total supply: {}%",
    //     ((count.unwrap() as f64) / 12284.0 * 100.0).floor()
    // );
    // println!("");

    // println!("Runes");
    // let count =
    //     collection_bids_gt_percent_of_top_bid("0x521f9C7505005CFA19A8E5786a9c3c9c9F5e6f42", 0.5)
    //         .await;
    // println!(
    //     "bid count as % of total supply: {}%",
    //     ((count.unwrap() as f64) / 9316.0 * 100.0).floor()
    // );
    // println!("");

    // println!("Cool Cats");
    // let count =
    //     collection_bids_gt_percent_of_top_bid("0x1A92f7381B9F03921564a437210bB9396471050C", 0.5)
    //         .await;
    // println!(
    //     "bid count as % of total supply: {}%",
    //     ((count.unwrap() as f64) / 9960.0 * 100.0).floor()
    // );
    // println!("");

    // println!("Goblin Town");
    // let count =
    //     collection_bids_gt_percent_of_top_bid("0xbCe3781ae7Ca1a5e050Bd9C4c77369867eBc307e", 0.5)
    //         .await;
    // println!(
    //     "bid count as % of total supply: {}%",
    //     ((count.unwrap() as f64) / 9999.0 * 100.0).floor()
    // );
    // println!("");

    // println!("0xmons");
    // let count =
    //     collection_bids_gt_percent_of_top_bid("0x0427743DF720801825a5c82e0582B1E915E0F750", 0.5)
    //         .await;
    // println!(
    //     "bid count as % of total supply: {}%",
    //     ((count.unwrap() as f64) / 341.0 * 100.0).floor()
    // );
    // println!("");

    // println!("Loot");
    // let count =
    //     collection_bids_gt_percent_of_top_bid("0xFF9C1b15B16263C61d017ee9F65C50e4AE0113D7", 0.5)
    //         .await;
    // println!(
    //     "bid count as % of total supply: {}%",
    //     ((count.unwrap() as f64) / 7779.0 * 100.0).floor()
    // );
    // println!("");

    // println!("Milday");
    // let count =
    //     collection_bids_gt_percent_of_top_bid("0x5Af0D9827E0c53E4799BB226655A1de152A425a5", 0.5)
    //         .await;
    // println!(
    //     "bid count as % of total supply: {}%",
    //     ((count.unwrap() as f64) / 9823.0 * 100.0).floor()
    // );
    // println!("");

    // let gql = GraphQLClient::default();
    // let response = gql
    //     .collateral_vaults_exceeding_debt_per_collateral(
    //         "0x6df74b0653ba2b622d911ef5680d1776d850ace9",
    //         "0x8232c5fd480c2a74d2f25d3362f262ff3511ce49",
    //         U256::from_dec_str("246987190433877370000").unwrap(),
    //     )
    //     .await?;
    // println!("Response: {:?}", response.first().unwrap().account);

    Ok(())
}

async fn collection_bids_gt_percent_of_top_bid(
    collection: &str,
    percent: f64,
) -> Result<usize, eyre::Error> {
    let oracle_info = ReservoirClient::default()
        .max_collection_bid(
            collection,
            PriceKind::Twap,
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
            None, // 604800 = 7 days
        )
        .await?;
    println!("{}", oracle_info.message.signature);
    // println!("{}", oracle_info.message.signature.parse::<Signature>().unwrap());

    println!("top bid {}", oracle_info.price);

    collection_bids_gte(collection, oracle_info.price * percent).await
}

async fn collection_bids_gte(collection: &str, price: f64) -> Result<usize, eyre::Error> {
    let count = ReservoirClient::default()
        .bids(&collection, Some(1000))
        .await?
        .orders
        .into_iter()
        .filter(|o| o.price.amount >= reservoir::orders::Amount { usd: price })
        .collect::<Vec<reservoir::orders::Order>>()
        .len();

    return Ok(count);
}
