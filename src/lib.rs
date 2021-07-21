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

use anyhow::Result;
use bdk::blockchain::ElectrumBlockchain;
use bdk::database::MemoryDatabase;
use bdk::electrum_client::Client;
use bdk::Wallet;
use bitcoin::Network;

pub mod cmd;

/// Dogecoin is divisible to 
pub const KOINU_IN_ONE_DOGECOIN: u64 = 100_000_000; // FIXME: Is this correct?

const ELECTRUMX_URL: &str = "tcp://127.0.0.1:50001";

type DogeWallet = Wallet<ElectrumBlockchain, MemoryDatabase>;

// Taken from dogecoind wallet dump (dcli dumpwallet ~/tmp/wallet)
const DESC: &str = "pkh(tprv8ZgxMBicQKsPeWaKVvhoETvieG37c9YEouU1wuD8zqkWhFowmbjJtS9PHRbzaKJtiixK1bEKFGUbWTru93spErRuxdaAwpH2aP5qMLQNdEN/0'/0'/*)";
const CHANGE_DESC: &str = "pkh(tprv8ZgxMBicQKsPeWaKVvhoETvieG37c9YEouU1wuD8zqkWhFowmbjJtS9PHRbzaKJtiixK1bEKFGUbWTru93spErRuxdaAwpH2aP5qMLQNdEN/1/*)";

pub fn electrumx_wallet() -> Result<DogeWallet> {
    let client = Client::new(ELECTRUMX_URL)?;
    let blockchain = ElectrumBlockchain::from(client);
    let wallet = Wallet::new(
        DESC,
        Some(CHANGE_DESC),
        Network::Testnet,
        MemoryDatabase::default(),
        blockchain,
    )?;

    Ok(wallet)
}

#[allow(clippy::cast_precision_loss)]
/// Get a string representing doge amount i.e., limit to 8 decimal places.
pub fn display_doge(x: u64) -> String {
    let d = x as f32 / 100000000.0;
    format!("{:.8}", d)
}
