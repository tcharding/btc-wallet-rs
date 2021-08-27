use std::str::FromStr;

use anyhow::{bail, Result};
use bdk::wallet::AddressIndex;
use bdk::SignOptions;
use bitcoin::Address;

use crate::{BtcWallet, SATS_IN_ONE_BITCOIN};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

/// Print the current balance.
pub fn balance(wallet: &BtcWallet) -> Result<()> {
    let b = wallet.get_balance()?;
    println!("Balance: {}", crate::display_btc(b));
    Ok(())
}

/// Generate and print the last unused address.
pub fn address(wallet: &BtcWallet) -> Result<()> {
    let info = wallet.get_address(AddressIndex::LastUnused)?;
    println!("Address: {}", *info);
    Ok(())
}

/// Send `amount` to `address`.
pub fn send(wallet: &BtcWallet, amount: u64, address: &str) -> Result<()> {
    let to = Address::from_str(address)?;

    let (mut psbt, details) = {
        let mut builder = wallet.build_tx();
        builder
            .add_recipient(to.script_pubkey(), amount)
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

/// Validate `address` is a valid standard Bitcoin address.
pub fn validate_address(address: &str) -> Result<()> {
    let addr = Address::from_str(address)?;
    if !addr.is_standard() {
        bail!("Non-standard address");
    }
    Ok(())
}

fn default_testnet_abs_fee() -> u64 {
    SATS_IN_ONE_BITCOIN
}

/// List transactions to/from this wallet.
pub fn list_transactions(wallet: &BtcWallet, include_raw: bool) -> Result<()> {
    for details in wallet.list_transactions(include_raw)? {
        println!("{}", serde_json::to_string_pretty(&details).unwrap());
    }
    Ok(())
}

/// List unspent transactions.
pub fn list_unspent(wallet: &BtcWallet) -> Result<()> {
    for utxo in wallet.list_unspent()? {
        println!("{}", serde_json::to_string_pretty(&utxo).unwrap());
    }
    Ok(())
}

/// Debug the wallet.
pub fn debug(wallet: &BtcWallet) -> Result<()> {
    let addr = wallet.get_address(AddressIndex::LastUnused)?;
    println!("Last unused address: {}", addr);

    let balance = wallet.get_balance()?;
    println!("Current balance: {}", balance);

    println!("Unspent UTXOs:");
    for utxo in wallet.list_unspent()?.iter() {
        println!("{:?}", utxo);
    }

    Ok(())
}
