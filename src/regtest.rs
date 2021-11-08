use std::path::PathBuf;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use anyhow::Result;
use bdk::bitcoin::Network;
use bdk::blockchain::rpc::{Auth, RpcBlockchain, RpcConfig};
use bdk::blockchain::{ConfigurableBlockchain, ElectrumBlockchain};
use bdk::database::AnyDatabase;
use bdk::electrum_client::Client;
use bdk::Wallet;

const ELECTRS_URL: &str = "tcp://127.0.0.1:50001";

// This private key has testnet funds on it. Send them back to faucet before changing: mkHS9ne12qx9pS9VojpwU5xtRd4T7X7ZUt
const DESC: &str = "pkh(tprv8ZgxMBicQKsPeWaKVvhoETvieG37c9YEouU1wuD8zqkWhFowmbjJtS9PHRbzaKJtiixK1bEKFGUbWTru93spErRuxdaAwpH2aP5qMLQNdEN/0'/0'/*)";
const CHANGE_DESC: &str = "pkh(tprv8ZgxMBicQKsPeWaKVvhoETvieG37c9YEouU1wuD8zqkWhFowmbjJtS9PHRbzaKJtiixK1bEKFGUbWTru93spErRuxdaAwpH2aP5qMLQNdEN/1/*)";

/// Create a wallet that connects directly to bitcoind via RPC. Uses REGTEST.
pub fn rpc_wallet(db_path: PathBuf) -> Result<Wallet<RpcBlockchain, AnyDatabase>> {
    let network = Network::Regtest;
    info!(
        "Creating {} wallet, connecting directly to bitcoind via RPC",
        network
    );
    debug!("Using database at: {}", db_path.display());

    let db = sled::open(db_path)?;
    let tree = db.open_tree(b"electrumx wallet")?;

    let config = RpcConfig {
        url: "127.0.0.1".to_string(),
        auth: Auth::UserPass {
            username: "user".to_string(),
            password: "pass".to_string(),
        },
        network,
        wallet_name: "descwallet".to_string(),
        skip_blocks: None,
    };

    let blockchain = RpcBlockchain::from_config(&config)?;

    let wallet = Wallet::new(
        DESC,
        Some(CHANGE_DESC),
        network,
        AnyDatabase::from(tree),
        blockchain,
    )?;

    Ok(wallet)
}

/// Create a wallet that connects to bitcoind by way of electrs. This uses REGTEST.
pub fn electrs_wallet(db_path: PathBuf) -> Result<Wallet<ElectrumBlockchain, AnyDatabase>> {
    let network = Network::Regtest;
    info!("Creating {} wallet using electrs as the indexer", network);
    debug!("Using database at: {}", db_path.display());
    let db = sled::open(db_path)?;
    let tree = db.open_tree(b"electrumx wallet")?;

    let client = Client::new(ELECTRS_URL)?;
    let blockchain = ElectrumBlockchain::from(client);

    let wallet = Wallet::new(
        DESC,
        Some(CHANGE_DESC),
        network,
        AnyDatabase::from(tree),
        blockchain,
    )?;

    Ok(wallet)
}
