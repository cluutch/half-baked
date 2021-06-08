# Half Baked oracle library

Please read my substack for more details: https://cluutch.substack.com/p/solana-hackathon-submission-for-half

Assumes Solana SDK installed. The repo come with a sample keychain and binary. Its recommended that you use these existing files. Instructions to recreate your own are marked with [NOT RECOMMENDED]. 

```
# Git code
✗ git clone 
✗ cd half-baked

# Start local validator✗ solana-test-validator
# [NOT RECOMMENDED]: Generate a new account. Instead, use the existing admin account in the git repo for consistency.
✗ solana-keygen new -o admin.json

=> HS4DFksToksNYF3fyU8Rxmg2B7sAq5JYhHKzitaVbDCp

# Airdrop to new account
✗ solana airdrop --url http://127.0.0.1:8899 --keypair admin.json 1000

# Start website. Observe what it looks like with dummy data. `Sync from Solana` will not do anything yet.
✗ yarn start

# [NOT RECOMMENDED]: Compile Half Baked oracle
✗ cd program
✗ cargo build-bpf

# Deploy oracle
✗ cd ..
✗ solana program deploy \
    --url http://127.0.0.1:8899 \
    --keypair admin.json \
    program/target/deploy/half_baked.so

# Prepare the metadata account [LINE CHANGE]
✗ cd cli
✗ cargo run -- create-cluutch-data-account --keypair ../admin.json --url http://127.0.0.1:8899
=> EtmqeZo4r1pFXonVCegK8S5YwcnmG9eEXKj4BVbR5Sun

# Prepare the metadata account [LINE CHANGE]
✗ cargo run -- create-cluutch-data-account --keypair admin.json --url http://127.0.0.1:8899
=> 52iJqwHvBsUhEaNg3wJJLzNSBzksaVC6kFSV18w215ZJ

# Seed the oracle with some data through the CLI.
✗ cargo run -- process-api-payload --keypair ../admin.json --url http://127.0.0.1:8899



From the website, click `Sync from Solana`. You will now see the UI load the entry from on-chain.
```


## COPIED FROM DAPP-SCAFFOLD

```bash
git clone https://github.com/solana-labs/dapp-scaffold.git

cd dapp-scaffold
```

```bash

yarn

```

```bash

yarn start

```

# Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana v1.6.7 or later from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool
3. Install Node
4. Install NPM, Yarn

# Build Smart Contract (compiled for BPF)
Run the following from the program/ subdirectory:

```bash
$ cargo build-bpf
$ cargo test-bpf
```
# Directory structure

## program

Solana program template in Rust

### program/src/lib.rs
* process_instruction function is used to run all calls issued to the smart contract

## src/actions

Setup here actions that will interact with Solana programs using sendTransaction function

## src/contexts

React context objects that are used propagate state of accounts across the application

## src/hooks

Generic react hooks to interact with token program:
* useUserBalance - query for balance of any user token by mint, returns:
    - balance
    - balanceLamports
    - balanceInUSD
* useUserTotalBalance - aggregates user balance across all token accounts and returns value in USD
    - balanceInUSD
* useAccountByMint
* useTokenName
* useUserAccounts

## src/views

* home - main page for your app
* faucet - airdrops SOL on Testnet and Devnet
