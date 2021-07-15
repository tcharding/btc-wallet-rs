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

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

/// Get the current balance.
pub fn balance() -> Result<()> {
    todo!("implement balance");
}

/// Generate a new address.
pub fn new_address() -> Result<()> {
    todo!("implement new_address")
}

/// Send `amount` to `address`.
pub fn send(_amount: u64, _address: String) -> Result<()> {
    todo!("implement send")
}
