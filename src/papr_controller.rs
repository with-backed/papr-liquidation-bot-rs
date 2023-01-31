use crate::provider::PROVIDER;
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    prelude::{abigen, TransactionReceipt},
    providers::{Http, Provider},
    signers::Wallet,
    types::{Address, U256},
};
use std::sync::Arc;

abigen!(PaprControllerABI, "src/abis/PaprController.json");
pub struct PaprController {
    controller: PaprControllerABI<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl PaprController {
    pub fn new(controller_addr_str: &str) -> Result<Self, eyre::Error> {
        let controller_addr = controller_addr_str.parse::<Address>()?;

        Ok(Self {
            controller: PaprControllerABI::new(controller_addr, Arc::clone(&PROVIDER)),
        })
    }

    pub async fn new_target(&self) -> Result<U256, eyre::Error> {
        Ok(self.controller.new_target().call().await?)
    }

    pub async fn start_liquidation_auction(
        &self,
        account: Address,
        collateral: Collateral,
        oracle_info: OracleInfo,
    ) -> Result<TransactionReceipt, eyre::Error> {
        self.controller
            .start_liquidation_auction(account, collateral, oracle_info)
            .send()
            .await?
            .await?
            .ok_or(eyre::eyre!(
                "start_liquidation_auction no transaction receipt"
            ))
        // TODO could dig in the logs here to return the auction object
    }
}
