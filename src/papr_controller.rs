use crate::provider::PROVIDER;
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    prelude::abigen,
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
    fn new(controller_addr_str: &str) -> Self {
        let controller_addr = controller_addr_str.parse::<Address>().unwrap();

        Self {
            controller: PaprControllerABI::new(controller_addr, Arc::clone(&PROVIDER)),
        }
    }

    async fn new_target(&self) -> Result<U256, eyre::Error> {
        Ok(self.controller.new_target().call().await?)
    }
}
