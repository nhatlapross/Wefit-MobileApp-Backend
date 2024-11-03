# `ICP Token Wallet`

The `ICP Token Wallet` is a Rust-based smart contract deployed on the Internet Computer Protocol (ICP) blockchain. It supports basic functionalities for managing IRCRC2 tokens, including sending and receiving tokens, and querying token balances. This project demonstrates proficiency with Rust and blockchain principles, and it is intended to provide a secure and functional wallet for handling IRCRC2 tokens on the ICP network.

## Features
`Send Tokens`: Transfer tokens between addresses.

`Receive Tokens`: Add tokens to a specified address.

`Query Balance`: Retrieve the token balance for a given address.

## Project Structure
`src/icp-token-wallet-backend.rs`: Contains the Rust code for the token wallet smart contract.

`src/icp-token-wallet-backend.did`: The Candid interface defining the service methods and types for the smart contract.

`Cargo.toml`: The Cargo configuration file for managing dependencies.

`dfx.json`: Configuration for deploying and managing the canisters.


## Prerequisites
`DFX`: The DFINITY SDK for developing and deploying canisters. You can install it from DFINITY's official website.

`Rust`: The programming language used for writing the smart contract. Install Rust from rust-lang.org.

## Setup and Installation
Clone the Repository

```bash
git clone https://github.com/negianoop/icp-token-wallet.git
cd icp-token-wallet
```

## Install Dependencies:

Make sure you have dfx and rustc installed. If not, follow the installation instructions on the respective websites.

## Run project locally

If you want to test this project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```
Use the DFX CLI to call functions on your deployed canisters. For example:

```bash
dfx canister call <canister_id> send '("sender_address", "receiver_address", 100)'
dfx canister call <canister_id> receive '("receiver_address", 100)'
dfx canister call <canister_id> get_balance '("address")'
```