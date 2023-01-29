use crate::provider::PROVIDER;
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    prelude::{abigen, PendingTransaction, TransactionReceipt},
    providers::{Http, Provider},
    signers::Wallet,
    types::{Address, U256},
};
use std::{env, sync::Arc};

abigen!(PaprControllerABI, "src/abis/PaprController.json");

pub struct PaprController {
    controller: PaprControllerABI<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl PaprController {
    pub fn new(controller_addr_str: &str) -> Self {
        let controller_addr = controller_addr_str
            .parse::<Address>()
            .expect("error parsing controller");

        Self {
            controller: PaprControllerABI::new(controller_addr, Arc::clone(&PROVIDER)),
        }
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
        Ok(self
            .controller
            .start_liquidation_auction(account, collateral, oracle_info)
            .send()
            .await?
            .await?
            .expect("start_liquidation_auction transaction error"))
        // TODO could dig in the logs here to return the auction object
    }
}
