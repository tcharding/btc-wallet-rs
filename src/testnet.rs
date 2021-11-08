use std::path::PathBuf;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use anyhow::Result;
use bdk::bitcoin::Network;
use bdk::blockchain::{ElectrumBlockchain, EsploraBlockchain};
use bdk::database::AnyDatabase;
use bdk::electrum_client::Client;
use bdk::Wallet;

/// Blockstreams Esplora instance, this is a forked version of Electrs that has
/// had a JSON RPC API bolted on and a bunch of performance enhancements. It
/// still exposes the ElectrumX REST API.
const ESPLORA_URL: &str = "https://blockstream.info/api";

/// ElectrumX REST API using blockstreams esplora server.
const ELECTRS_URL: &str = "tcp://blockstream.info:143";

// This private key has testnet funds on it. Send them back to faucet before changing: mkHS9ne12qx9pS9VojpwU5xtRd4T7X7ZUt
const DESC: &str = "pkh(tprv8ZgxMBicQKsPeWaKVvhoETvieG37c9YEouU1wuD8zqkWhFowmbjJtS9PHRbzaKJtiixK1bEKFGUbWTru93spErRuxdaAwpH2aP5qMLQNdEN/0'/0'/*)";
const CHANGE_DESC: &str = "pkh(tprv8ZgxMBicQKsPeWaKVvhoETvieG37c9YEouU1wuD8zqkWhFowmbjJtS9PHRbzaKJtiixK1bEKFGUbWTru93spErRuxdaAwpH2aP5qMLQNdEN/1/*)";

const NETWORK: Network = Network::Testnet;

/// Create a wallet that connects to bitcoind by way of esplora.
pub fn esplora_wallet(db_path: PathBuf) -> Result<Wallet<EsploraBlockchain, AnyDatabase>> {
    info!("Creating {} wallet using esplora as the indexer", NETWORK);
    debug!("Using database at: {}", db_path.display());
    let db = sled::open(db_path)?;
    let tree = db.open_tree(b"esplora wallet")?;

    let blockchain = EsploraBlockchain::new(ESPLORA_URL, 20);

    let wallet = Wallet::new(
        DESC,
        Some(CHANGE_DESC),
        NETWORK,
        AnyDatabase::from(tree),
        blockchain,
    )?;

    Ok(wallet)
}

/// Create a wallet that connects to bitcoind by way of the ElectrumX REST API.
pub fn electrs_wallet(db_path: PathBuf) -> Result<Wallet<ElectrumBlockchain, AnyDatabase>> {
    info!("Creating {} wallet using electrs as the indexer", NETWORK);
    debug!("Using database at: {}", db_path.display());
    let db = sled::open(db_path)?;
    let tree = db.open_tree(b"electrumx wallet")?;

    let client = Client::new(ELECTRS_URL)?;
    let blockchain = ElectrumBlockchain::from(client);

    let wallet = Wallet::new(
        DESC,
        Some(CHANGE_DESC),
        NETWORK,
        AnyDatabase::from(tree),
        blockchain,
    )?;

    Ok(wallet)
}
