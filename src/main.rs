mod papr_controller;
mod papr_subgraph;
mod provider;
mod reservoir;
mod start;
mod purchase;
use crate::{
    papr_subgraph::client::GraphQLClient,
    reservoir::{client::ReservoirClient, oracle::PriceKind},
    start::start_liquidations_for_whitelisted_controllers,
};

#[tokio::main]
async fn main() -> Result<(), eyre::Error> {
    let graphql = GraphQLClient::default();
    let reservoir = ReservoirClient::default();

    let x = start_liquidations_for_whitelisted_controllers(&reservoir, &graphql).await;
    if let Some(err) = x.err() {
        println!("{}", err);
    }

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

    println!("top bid ${}", oracle_info.price);

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
