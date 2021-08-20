# Bitcoin Wallet in Rust

Simple command line Bitcoin wallet written in Rust using [bdk](https://github.com/bitcoindevkit/bdk).

Only ever used on testnet, use at your own risk.

The primary purpose of this repository is to test things I'm hacking on in `bdk`.

## Usage

```
$ btc-wallet --help
btc-wallet 0.1.0
Simple Bitcoin wallet

USAGE:
    btc-wallet [OPTIONS] <SUBCOMMAND>

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
    send                 Send Bitcoin to `address`. `amount` is in satoshis. 
    validate             Validate address

```