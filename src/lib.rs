#![warn(
    unused_extern_crates,
    missing_copy_implementations,
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::fallible_impl_from,
    clippy::print_stdout,
    clippy::dbg_macro
)]
#![allow(clippy::nonstandard_macro_braces)]

use std::path::PathBuf;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use anyhow::Result;
use bdk::blockchain::ElectrumBlockchain;
use bdk::database::AnyDatabase;
use bdk::electrum_client::Client;
use bdk::Wallet;
use bitcoin::Network;

pub mod cmd;

/// Minimum divisible unit for bitcoin (satoshis).
pub const SATS_IN_ONE_BITCOIN: u64 = 100_000_000;

/// Blockstreams Esplora instance, this is a forked version of Electrs that has
/// had a JSON RPC API bolted on and a bunch of performance enhancements. It
/// still exposes the ElectrumX REST API.
const ELECTRUMX_URL: &str = "tcp://blockstream.info:143";

type BtcWallet = Wallet<ElectrumBlockchain, AnyDatabase>;

// This private key has testnet funds on it. Send them back to faucet before changing: mkHS9ne12qx9pS9VojpwU5xtRd4T7X7ZUt
const DESC: &str = "pkh(tprv8ZgxMBicQKsPeWaKVvhoETvieG37c9YEouU1wuD8zqkWhFowmbjJtS9PHRbzaKJtiixK1bEKFGUbWTru93spErRuxdaAwpH2aP5qMLQNdEN/0'/0'/*)";
const CHANGE_DESC: &str = "pkh(tprv8ZgxMBicQKsPeWaKVvhoETvieG37c9YEouU1wuD8zqkWhFowmbjJtS9PHRbzaKJtiixK1bEKFGUbWTru93spErRuxdaAwpH2aP5qMLQNdEN/1/*)";

pub fn electrumx_wallet(db_path: PathBuf) -> Result<BtcWallet> {
    info!("Creating wallet");
    debug!("Using database at: {}", db_path.display());
    let db = sled::open(db_path)?;
    let tree = db.open_tree(b"electrumx wallet")?;

    let client = Client::new(ELECTRUMX_URL)?;
    let blockchain = ElectrumBlockchain::from(client);
    let wallet = Wallet::new(
        DESC,
        Some(CHANGE_DESC),
        Network::Testnet,
        AnyDatabase::from(tree),
        blockchain,
    )?;

    Ok(wallet)
}

#[allow(clippy::cast_precision_loss)]
/// Get a string representing btc amount i.e., limit to 8 decimal places.
pub fn display_btc(x: u64) -> String {
    let d = x as f32 / 100000000.0;
    format!("{:.8}", d)
}
