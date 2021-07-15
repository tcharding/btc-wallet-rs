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
