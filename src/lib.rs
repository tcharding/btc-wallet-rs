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

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub mod cmd;
pub mod regtest;
pub mod testnet;

/// Minimum divisible unit for bitcoin (satoshis).
pub const SATS_IN_ONE_BITCOIN: u64 = 100_000_000;

#[allow(clippy::cast_precision_loss)]
/// Get a string representing btc amount i.e., limit to 8 decimal places.
pub fn display_btc(x: u64) -> String {
    let d = x as f32 / 100000000.0;
    format!("{:.8}", d)
}
