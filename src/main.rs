use anyhow::Result;
use structopt::StructOpt;

use doge_wallet::{cmd, dogecoind_rpc_wallet};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

fn main() -> Result<()> {
    let opt = Opt::from_args();
    env_logger::init();

    debug!("{:?}", opt);

    if opt.passphrase.is_some() {
        warn!("passphrase is currently unimplemented");
    }

    info!("Creating wallet");
    let wallet = dogecoind_rpc_wallet()?;
    //    wallet.sync(NoopProgress, None)?;

    match opt.cmd {
        Cmd::Balance => {
            let b = wallet.get_balance()?;
            println!("Balance: {}", b);
        }
        Cmd::NewAddress => cmd::new_address()?,
        Cmd::Send { amount, address } => cmd::send(amount, address)?,
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
    /// Get the current balance.
    Balance,
    /// Generate a new address.
    NewAddress,
    /// Send `amount` to `address`.
    Send { amount: u64, address: String },
}
