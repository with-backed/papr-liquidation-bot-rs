use ethers::{types::U256, prelude::abigen};

abigen!(PaprController, "src/abis/PaprController.json");

lazy_static! {
    let PROVIDER = Arc::new({
        // connect to the network
        let provider = Provider::<Http>::try_from(
            env::var("JSON_RPC_PROVIDER").expect("RESERVOIR_API_KEY not set"),
        )?;
        let chain_id = provider.get_chainid().await?;

        // this wallet's private key
        let wallet = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set")
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        SignerMiddleware::new(provider, wallet)
    });
}