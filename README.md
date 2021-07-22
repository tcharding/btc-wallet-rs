# Dogecoin Wallet in Rust

Command line dogecoin wallet written in Rust.

Only ever used on testnet, use at your own risk.

## Usage

```
$ doge-wallet --help
doge-wallet 0.1.0
Simple Dogecoin wallet

USAGE:
    doge-wallet [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --passphrase <passphrase>    Wallet passphrase

SUBCOMMANDS:
    address              Generate and print the last unused address
    balance              Print the current balance
    debug                Debug the wallet
    help                 Prints this message or the help of the given subcommand(s)
    list-transactions    List transactions to/from this wallet
    list-unspent         List unspent transactions
    send                 Send Dogecoin to `address`. `amount` is in dogecoin (not koinus)
    validate             Validate address

```