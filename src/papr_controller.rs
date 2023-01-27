use std::{env, sync::Arc};
use ethers::{types::{U256, Address}, prelude::abigen, providers::{Http, Provider}, middleware::SignerMiddleware, signers::{LocalWallet, Signer, Wallet} };

abigen!(PaprController, "src/abis/PaprController.json");

// pub struct PaprControllerProvider {
//     provider: SignerMiddleware<ethers::providers::Provider<_>, Wallet<ethers::core::k256::ecdsa::SigningKey>>,
// }

// impl Default for PaprControllerProvider {
//     fn default() -> Self {
//         let provider = Provider::<Http>::try_from(
//             env::var("JSON_RPC_PROVIDER").expect("JSON_RPC_PROVIDER not set"),
//         ).expect("err");
//         let chain_id = U256::from_dec_str(&env::var("CHAIN_ID").expect("CHAIN_ID not set")).unwrap();
//         // this wallet's private key
//         let wallet = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set")
//             .parse::<LocalWallet>()
//             .unwrap()
//             .with_chain_id(chain_id.as_u64());

//         Self {
//             provider: SignerMiddleware::new(provider, wallet)
//         }
//     }
// }

// impl PaprControllerProvider {
    // async fn newTarget(&self, controller_addr_str: &str) -> Result<U256, eyre::Error>  {
    //     let controller_addr = controller_addr_str.parse::<Address>().unwrap();
    //     let controller = PaprController::new(controller_addr, Arc::new(self.provider.clone()));
    //     Ok(controller.new_target().call().await?)
    // }
// }

lazy_static! {
    static ref PROVIDER : Arc<SignerMiddleware<Provider<Http>, Wallet<ethers::core::k256::ecdsa::SigningKey>>> = {
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
    let controller = PaprController::new(controller_addr, Arc::new(PROVIDER.clone()));
    Ok(controller.new_target().call().await?)
}