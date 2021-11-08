use std::str::FromStr;

use anyhow::{bail, Result};
use bdk::bitcoin::{Address, Txid};
use bdk::blockchain::Blockchain;
use bdk::database::BatchDatabase;
use bdk::wallet::AddressIndex;
use bdk::{SignOptions, Wallet};

// use crate::taproot;
use crate::SATS_IN_ONE_BITCOIN;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

/// Print the current balance.
pub fn balance<B, D>(wallet: &Wallet<B, D>) -> Result<()>
where
    B: Blockchain,
    D: BatchDatabase,
{
    let b = wallet.get_balance()?;
    println!("Balance: {}", crate::display_btc(b));
    Ok(())
}

/// Generate and print the last unused address.
pub fn address<B, D>(wallet: &Wallet<B, D>) -> Result<()>
where
    B: Blockchain,
    D: BatchDatabase,
{
    let info = wallet.get_address(AddressIndex::LastUnused)?;
    println!("Address: {}", *info);
    Ok(())
}

/// Send `amount` to `address`.
pub fn send<B, D>(wallet: &Wallet<B, D>, amount: u64, address: &str) -> Result<()>
where
    B: Blockchain,
    D: BatchDatabase,
{
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
pub fn list_transactions<B, D>(wallet: &Wallet<B, D>, include_raw: bool) -> Result<()>
where
    B: Blockchain,
    D: BatchDatabase,
{
    for details in wallet.list_transactions(include_raw)? {
        println!("{}", serde_json::to_string_pretty(&details).unwrap());
    }
    Ok(())
}

/// List unspent transactions.
pub fn list_unspent<B, D>(wallet: &Wallet<B, D>) -> Result<()>
where
    B: Blockchain,
    D: BatchDatabase,
{
    for utxo in wallet.list_unspent()? {
        println!("{}", serde_json::to_string_pretty(&utxo).unwrap());
    }
    Ok(())
}

/// Debug the wallet.
pub fn debug<B, D>(wallet: &Wallet<B, D>) -> Result<()>
where
    B: Blockchain,
    D: BatchDatabase,
{
    let reset = wallet.get_address(AddressIndex::Reset(0))?;
    let last_unused = wallet.get_address(AddressIndex::LastUnused)?;
    let new = wallet.get_address(AddressIndex::New)?;

    println!();
    println!("used addr:   {}", reset);
    println!("last unused: {}", last_unused);
    println!("new addr:    {}", new);

    let b = wallet.get_balance()?;
    println!();
    println!("Balance: {}", crate::display_btc(b));

    let txid = Txid::from_str("e8a7d8bc97f114255d99aaf20a71a7e9d80ed627c6217be8d00528b2c8daaa09")?;
    let tx = wallet.get_tx(&txid, true);
    println!();
    println!("{:?}", tx);

    Ok(())
}
