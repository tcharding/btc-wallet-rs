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

use anyhow::Result;
use bdk::blockchain::{ConfigurableBlockchain, RpcBlockchain, RpcConfig};
use bdk::database::MemoryDatabase;
use bdk::Wallet;
use bitcoin::Network;
use bitcoincore_rpc::Auth;

pub mod cmd;

const DOGE_ELECTRS_URL: &str = "http://127.0.0.1:51001";
// const RPC_USER: &str = "tobin";
// const RPC_PASS: &str = "Jw0jFIMHly_lCMde2Mq28_ZIyQlVdslv-ScmonTRPyc=";

// Descriptors created using `bdk-cli`, see `~/docs/bdk/testnet.md`.
const DESC: &str = "wpkh(tprv8ZgxMBicQKsPdT8dRdm7Ae7ZxLTCKNPaZwt7aBWNRyxUCMvY7xhjRG4iBLerk2FTBv6zrzMMw18M3LwJEvn9QhbzsiYJefwUmzcUXcAPDmt/0/*)";
const CHANGE_DESC: &str = "wpkh(tprv8ZgxMBicQKsPdT8dRdm7Ae7ZxLTCKNPaZwt7aBWNRyxUCMvY7xhjRG4iBLerk2FTBv6zrzMMw18M3LwJEvn9QhbzsiYJefwUmzcUXcAPDmt/1/*)";

// This does not work because dogecoind does not implement `listwallets`.
// Therefore to use the `RpcBlockchain` we would have to backport that feature
// to dogecoincore - that means C++ hacking.
pub fn dogecoind_rpc_wallet() -> Result<Wallet<RpcBlockchain, MemoryDatabase>> {
    // let auth = Auth::UserPass(RPC_USER.to_string(), RPC_PASS.to_string());
    let auth = Auth::CookieFile(PathBuf::from("/home/tobin/.dogecoin/testnet3/.cookie"));

    let config = RpcConfig {
        url: DOGE_ELECTRS_URL.to_string(),
        auth,
        network: Network::Testnet,
        wallet_name: "doge_1".to_string(),
        skip_blocks: None,
    };

    let wallet = Wallet::new(
        DESC,
        Some(CHANGE_DESC),
        Network::Testnet,
        MemoryDatabase::default(),
        RpcBlockchain::from_config(&config)?,
    )?;

    Ok(wallet)
}
