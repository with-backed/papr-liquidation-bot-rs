use ethers::{types::U256, core::k256::ecdsa::SigningKey, providers::{Http, Provider}, middleware::SignerMiddleware, signers::{LocalWallet, Wallet, Signer} };
use once_cell::sync::{OnceCell, Lazy};
use std::{env, sync::Arc};

static ETH_RPC_PROVIDER: Lazy<String> =
    Lazy::new(|| env::var("ETH_RPC_PROVIDER").expect("ETH_RPC_PROVIDER not set"));

static CHAIN_ID: Lazy<String> =
    Lazy::new(|| env::var("CHAIN_ID").expect("CHAIN_ID not set"));

static PRIVATE_KEY: Lazy<String> =
    Lazy::new(|| env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set"));

pub fn signer_provider() -> &'static Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    static INSTANCE: OnceCell<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let provider = Provider::<Http>::try_from(ETH_RPC_PROVIDER.to_string()).unwrap();

        let chain_id = U256::from_dec_str(&CHAIN_ID.to_string()).expect("could not parse chain ID");
        let wallet = PRIVATE_KEY
                .parse::<LocalWallet>()
                .expect("error parsing private key")
                .with_chain_id(chain_id.as_u64());

        Arc::new(SignerMiddleware::new(provider, wallet))
    })
}