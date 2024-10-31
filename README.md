# payment

cargo-near-new-project-description

## How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near build
```

## How to Test Locally?

```bash
cargo test
```

## How to Deploy?

Deployment is automated with GitHub Actions CI/CD pipeline.
To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near deploy <account-id>
```

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Interact with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)

# Flow

1. Creator deploy model, post to market, register with payment info

2. User will pay when using model by function-access key


## Init

- commit new changes
```bash
git commit -a -m "init"
```

- build without docker
```bash
cargo near build --no-docker
```

- create new account for Payment contract
```bash
cargo near create-dev-account use-random-account-id autogenerate-new-keypair save-to-keychain network-config testnet create
```

- deploy Payment contract
```bash
near contract deploy <contract-account-name.testnet> use-file ./target/near/<project-name>.wasm without-init-call network-config testnet sign-with-keychain send
```

- init Payment contract
```bash
near call tearful-sound.testnet new '{"ft_id": "adhesive-lizards.testnet"}' --accountId tearful-sound.testnet
```

## Set up FT for payment

- register Payment contract in FT contract
```bash
near call <ft-contract.testnet> storage_deposit '{"account_id": "<account-name.testnet>"}' --gas 100000000000000 --deposit 0.01 --accountId <account-name.testnet> --networkId testnet
```

- transfer token to contract
```bash
near call <ft-contract.testnet> ft_transfer_call '{"receiver_id": "<account-name.testnet>", "amount": "1000000000000000000000000", "msg": "Go Team :)"}' --gas 100000000000000 --depositYocto 1 --accountId <account-name.testnet> --networkId testnet
```

- get token deposit amount
```bash
near view tearful-sound.testnet ft_deposits_of '{"account_id": "medical-rings.testnet"}' --networkId testnet
```

- register creator in FT contract
```bash
near call <ft-contract.testnet> storage_deposit '{"account_id": "<account-name.testnet>"}' --gas 100000000000000 --deposit 0.01 --accountId <account-name.testnet> --networkId testnet
```

## Related to model
- register new model
```bash
near call tearful-sound.testnet register_model '{"fee_per_prompt": "100000000", "metadata_id": 1}' --gas 100000000000000 --accountId medical-rings.testnet --networkId testnet
```

- count models
```bash
near view tearful-sound.testnet get_model_count --networkId testnet
```

- get model info
```bash
near view tearful-sound.testnet get_model_info '{"model_id": 1}' --networkId testnet
```

- get model info by metadata id
```bash
near view tearful-sound.testnet get_model_by_metadata_id '{"metadata_id": 1}' --networkId testnet
```

- get models info by creator
```bash
near view tearful-sound.testnet get_models_by_creator '{"creator": "<account-name.testnet>"}' --networkId testnet
```

- get all models info
```bash
near view tearful-sound.testnet get_all_models --networkId testnet
```

- update model info
```bash
near call tearful-sound.testnet update_model_info '{"model_id": 1, "new_fee_per_prompt": "150000000"}' --gas 100000000000000 --accountId medical-rings.testnet --networkId testnet
```

- pay
```bash
near call tearful-sound.testnet pay '{"model_id": 1}' --gas 100000000000000 --accountId noxious-advertisement.testnet --networkId testnet
```
