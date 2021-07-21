use std::path::PathBuf;
use std::env;
use std::process;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use anyhow::Result;
use bdk::blockchain;
use structopt::StructOpt;

use doge_wallet::{cmd, electrumx_wallet};

fn main() -> Result<()> {
    let opt = Opt::from_args();
    env_logger::init();

    debug!("{:?}", opt);

    if opt.passphrase.is_some() {
        warn!("passphrase is currently unimplemented");
    }

    if let Cmd::Validate { address } = opt.cmd {
        cmd::validate_address(&address)?;
        println!("Address is a valid standard Dogecoin address");
        process::exit(0);
    }

    let db = database_path()?;
    let wallet = crate::electrumx_wallet(db)?;
    wallet.sync(blockchain::log_progress(), None)?;

    match opt.cmd {
        Cmd::Balance => cmd::balance(&wallet)?,
        Cmd::Address => cmd::address(&wallet)?,
        Cmd::Validate { .. } => unreachable!("we checked this already above"),
        Cmd::Send { amount, address } => cmd::send(&wallet, amount, &address)?,
        Cmd::ListTransactions { include_raw } => cmd::list_transactions(&wallet, include_raw)?,
        Cmd::ListUnspent => cmd::list_unspent(&wallet)?,
        Cmd::Debug => cmd::debug(&wallet)?,
    }

    Ok(())
}

fn database_path() -> Result<PathBuf> {
    let mut db = PathBuf::new();
    db.push(env::var("HOME")?);
    db.push(".cache");
    db.push("doge-wallet-rs");
    db.push("db");

    Ok(db)
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(about = "Dogecoin wallet")]
pub struct Opt {
    /// Wallet passphrase
    #[structopt(short, long)]
    passphrase: Option<String>,

    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(about = "Simple Dogecoin wallet")]
pub enum Cmd {
    /// Print the current balance.
    Balance,
    /// Generate and print the last unused address.
    Address,
    /// Send Dogecoin to `address`. `amount` is in dogecoin (not koinus).
    Send { amount: u64, address: String },
    /// Validate address.
    Validate { address: String },
    /// List transactions to/from this wallet.
    ListTransactions {
        /// Include raw transaction data.
        #[structopt(long)]
        include_raw: bool,
    },
    /// List unspent transactions.
    ListUnspent,
    /// Debug the wallet.
    Debug,
}
