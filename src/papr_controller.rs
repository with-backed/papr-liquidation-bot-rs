use std::{env, sync::Arc};
use ethers::{core::k256::ecdsa::SigningKey, types::{U256, Address}, prelude::abigen, providers::{Http, Provider}, middleware::SignerMiddleware, signers::{Wallet} };
use crate::provider::signer_provider;

abigen!(PaprControllerABI, "src/abis/PaprController.json");

pub struct PaprController {
    controller: PaprControllerABI<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>
}

impl PaprController {
    fn new(controller_addr_str: &str) -> Self {
        let controller_addr = controller_addr_str.parse::<Address>().unwrap();

        Self {
            controller: PaprControllerABI::new(controller_addr, Arc::clone(signer_provider()))
        }
    }

    async fn newTarget(&self) -> Result<U256, eyre::Error>  {
        Ok(self.controller.new_target().call().await?)
    }
}
