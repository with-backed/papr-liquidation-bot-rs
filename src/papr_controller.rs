use std::{env, sync::Arc};
use ethers::{core::k256::ecdsa::SigningKey, types::{U256, Address}, prelude::abigen, providers::{Http, Provider}, middleware::SignerMiddleware, signers::{LocalWallet, Signer, Wallet} };

abigen!(PaprController, "src/abis/PaprController.json");

pub struct PaprControllerProvider {
    controller: PaprController<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>
}

impl PaprControllerProvider {
    fn new(controller_addr_str: &str) -> Self {
        let provider = Provider::<Http>::try_from(
            env::var("JSON_RPC_PROVIDER").expect("JSON_RPC_PROVIDER not set"),
        ).expect("err");
        let chain_id = U256::from_dec_str(&env::var("CHAIN_ID").expect("CHAIN_ID not set")).unwrap();
        // this wallet's private key
        let wallet = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set")
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(chain_id.as_u64());

        let signer = SignerMiddleware::new(provider, wallet);
        let controller_addr = controller_addr_str.parse::<Address>().unwrap();

        Self {
            controller: PaprController::new(controller_addr, Arc::new(signer))
        }
    }

    async fn newTarget(&self) -> Result<U256, eyre::Error>  {
        Ok(self.controller.new_target().call().await?)
    }
}


lazy_static! {
    static ref PROVIDER : Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> = {
        let p = Arc::new({
            // connect to the network
            let provider = Provider::<Http>::try_from(
                env::var("JSON_RPC_PROVIDER").expect("JSON_RPC_PROVIDER not set"),
            ).unwrap();
            let chain_id = U256::from_dec_str(&env::var("CHAIN_ID").expect("CHAIN_ID not set"));
            // this wallet's private key
            let wallet = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set")
                .parse::<LocalWallet>()
                .unwrap()
                .with_chain_id(chain_id.unwrap().as_u64());

            SignerMiddleware::new(provider, wallet)
        });
        p
    };
}

async fn newTarget(controller_addr_str: &str) -> Result<U256, eyre::Error>  {
    let controller_addr = controller_addr_str.parse::<Address>().unwrap();
    let controller = PaprController::new(controller_addr, Arc::clone(&PROVIDER));
    Ok(controller.new_target().call().await?)
}

use crate::provider::signer_provider;

async fn new_target(controller_addr_str: &str) -> Result<U256, eyre::Error>  {
    let controller_addr = controller_addr_str.parse::<Address>().unwrap();
    let controller = PaprController::new(controller_addr, Arc::clone(signer_provider()));
    Ok(controller.new_target().call().await?)
}