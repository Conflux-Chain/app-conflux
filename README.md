# App Conflux Rust

This is the `Conflux Core Space` Ledger device app, which can work with the [`Conflux Fluent Wallet`](https://fluentwallet.com/) to send transactions to the Conflux Network (Core Space). This app adds support for the Conflux native token(CFX) to the Ledger Nano S+/Flex/Stax hardware wallet.

Current Features:

- Public key queries
- Parse, display, and sign simple CFX transfer transactions

## Command APDU

[APDU specification](./docs/COMMANDS.md)

## Building and installing

This project is developed in Rust and started from the [Ledger Rust boilerplate](https://github.com/LedgerHQ/app-boilerplate-rust). 

### Prerequisites

To build and install the app on your Ledger Device, you must set up the development environment as described in the [Ledger documentation](https://developers.ledger.com/docs/device-app/beginner/vscode-extension). You need to install below tools:

1. Docker
2. X Server
3. VS Code
4. [The Ledger extension in VS Code](https://marketplace.visualstudio.com/items?itemName=LedgerHQ.ledger-dev-tools)

### Clone the repository

```bash
git clone git@github.com:Conflux-Chain/app-conflux.git
cd app-conflux
```

### Build & Test

![](./docs/ledger-device-app-build-shotcut.png)

## Resources

1. [Ledger Official Documentation](https://developers.ledger.com/docs/device-app/getting-started)
2. [Ledger Rust SDK](https://github.com/LedgerHQ/ledger-device-rust-sdk)
3. [Ledger Rust Boilerplate](https://github.com/LedgerHQ/app-boilerplate-rust)
4. [Ledger Discord](https://discord.gg/Ledger)
5. [Community Rust Libs](https://github.com/alamgu/)

### Other Ledger Apps in Rust

1. [Sui](https://github.com/LedgerHQ/app-sui)
2. [Radix Babylon](https://github.com/LedgerHQ/app-radix-babylon)
3. [Ledger Alephium](https://github.com/LedgerHQ/app-alephium)
4. [Ledger Starknet](https://github.com/LedgerHQ/app-starknet)
5. [Ledger Ironfish DKG](https://github.com/LedgerHQ/app-starknet)
6. [Near](https://github.com/LedgerHQ/app-near)
7. [app-pocket](https://github.com/LedgerHQ/app-pocket)

## Faqs

### 1. Why do not support the Ledger Nano S and Nano X?

The Nano S has not been sold since June 2022, and Ledger Live no longer supports new apps for the Nano S.

Due to chip limitations, the Nano X does not currently support Rust development (does not support atomic swap). We will consider adding support for the Nano X in the future.

### 2. If my device is a Nano S or Nano X, what should I do?

You can purchase new hardware wallets like the Nano S Plus, Flex, or Stax. Import your recovery phrase and use the wallet after restoring it.

### 3. I am a Conflux eSpace user, which app should I use?

Since the Conflux eSpace is fully compatible with the Ethereum, you can use the Ethereum app + MetaMask to interact with eSpace.

### 4. Does this app support the Conflux 1559 hardfork?

Yes
