use std::str::FromStr;

use anyhow::{bail, Result};
use bdk::wallet::AddressIndex;
use bdk::SignOptions;
use bitcoin::Address;

use crate::{KOINU_IN_ONE_DOGECOIN, DogeWallet};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

/// Print the current balance.
pub fn balance(wallet: &DogeWallet) -> Result<()> {
    let b = wallet.get_balance()?;
    println!("Balance: {}", crate::display_doge(b));
    Ok(())
}

/// Generate and print the last unused address.
pub fn address(wallet: &DogeWallet) -> Result<()> {
    let info = wallet.get_address(AddressIndex::LastUnused)?;
    println!("Address: {}", *info);
    Ok(())
}

/// Send `amount` to `address`.
pub fn send(wallet: &DogeWallet, dogecoin: u64, address: &str) -> Result<()> {
    let koinus = dogecoin * KOINU_IN_ONE_DOGECOIN;
    let to = Address::from_str(address)?;

    let (mut psbt, details) = {
        let mut builder = wallet.build_tx();
        builder
            .add_recipient(to.script_pubkey(), koinus)
            .enable_rbf()
            .fee_absolute(default_testnet_abs_fee());
        builder.finish()?
    };

    debug!("txn details: {:?}", details);

    wallet.sign(&mut psbt, SignOptions::default())?;
    let txid = wallet.broadcast(psbt.extract_tx())?;
    println!("Broadcast transaction: {}", txid);

    Ok(())
}

/// Validate `address` is a valid standard Dogecoin address.
pub fn validate_address(address: &str) -> Result<()> {
    let addr = Address::from_str(address)?;
    if !addr.is_standard() {
        bail!("Non-standard address");
    }
    Ok(())
}

fn default_testnet_abs_fee() -> u64 {
    KOINU_IN_ONE_DOGECOIN
}

/// Debug the wallet.
pub fn debug(wallet: &DogeWallet) -> Result<()> {
    // Print unspent UTXOs.
    println!("Listing all transactions");
    for details in wallet.list_transactions(false)? {
        println!("{}", serde_json::to_string_pretty(&details).unwrap());
    }

    // TODO: Print all used addresses.
    Ok(())
}
