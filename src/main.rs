use std::process;

use anyhow::Result;
use bdk::blockchain;
use structopt::StructOpt;

use doge_wallet::{cmd, electrumx_wallet};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

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

    info!("Creating wallet");
    let wallet = electrumx_wallet()?;
    wallet.sync(blockchain::log_progress(), None)?;

    match opt.cmd {
        Cmd::Balance => cmd::balance(&wallet)?,
        Cmd::Address => cmd::address(&wallet)?,
        Cmd::Validate { .. } => unreachable!("we checked this already above"),
        Cmd::Send { amount, address } => cmd::send(&wallet, amount, &address)?,
        Cmd::Debug => cmd::debug(&wallet)?,
    }

    Ok(())
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
    /// Debug the wallet.
    Debug,
}
