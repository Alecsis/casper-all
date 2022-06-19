# Casper ALL

This repository contains my contracts and resources for Casper related projects.

To generate a wallet, run:

```sh
mkdir wallet
cd wallet
casper-client keygen
```

To build the contracts, run:

```sh
# Make sure you are at the root of the project
make prepare
make build-contracts
```