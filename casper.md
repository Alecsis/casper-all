# Casper

You need the client: `cargo install casper-client`

## Create some wallet

Use the casper-client to create a wallet.

```sh
casper-client keygen ./wallet
```

It should create a folder `wallet` with the private and public keys:

```txt
.wallet
├── public_key.pem
├── public_key_hex
└── secret_key.pem
```

## Install a contract

To install a contract in global state, you need to send a deploy to the network with the contract Wasm. You can do so by using the put-deploy command. Remember to verify the deploy.

```sh
casper-client put-deploy \
    --node-address [NODE_SERVER_ADDRESS] \
    --chain-name [CHAIN_NAME] \
    --secret-key [KEY_PATH]/secret_key.pem \
    --payment-amount [PAYMENT_AMOUNT_IN_MOTES] \
    --session-path [CONTRACT_PATH]/[CONTRACT_NAME].wasm
```

For example, we deploy a contract here:

```sh
casper-client put-deploy \
 --node-address http://16.162.124.124:7777 \
 --chain-name casper-test \
 --secret-key ./wallet/secret_key.pem \
 --payment-amount 1000000 \
 --session-path ./target/wasm32-unknown-unknown/release/purse_transfer.wasm \
 --session-arg "src_purse:uref='uref-c9733355d61aa2a36721d9d1081eebcfe5dde94f82386b3d75163fee894d292a-007'"
```

The result is:

```json
{
  "id": -7220934104870306962,
  "jsonrpc": "2.0",
  "result": {
    "api_version": "1.4.6",
    "deploy_hash": "5f4ecf256ac5f88b648bf64c5356b16766cc429784e9a57b31de4a59fffd9d8d"
  }
}
```

We analyse the deploy hash:

```sh
casper-client get-deploy --node-address http://16.162.124.124:7777 5f4ecf256ac5f88b648bf64c5356b16766cc429784e9a57b31de4a59fffd9d8d | jq
```

## Find contracts in the explorer ?

We start with the friendly market address that I retrieved from the list of [contracts](https://testnet.cspr.live/contracts) in the testnet explorer. Taking a closer look using it's package hash [dde7472639058717a42e22d297d6cf3e07906bb57bc28efceac3677f8a3dc83b](https://testnet.cspr.live/contract-package/dde7472639058717a42e22d297d6cf3e07906bb57bc28efceac3677f8a3dc83b):

```yaml
- Contract Package Hash: dde7472639058717a42e22d297d6cf3e07906bb57bc28efceac3677f8a3dc83b
- Access Key: uref-8a4133032b84ae73340a70c169219eb2ca51cc067012dee88d026051689c75fa-007
- Name: Friendly Market 
- Description: Gateway contract part of Friendly Market’s suite of DeFi tools 
- Owner Public Key: 016d9e3db0a800aef8d18975b469c77bef042ee909d24cb83d27df97a22bb6d8ad
- Contract Type: DeFi 
- Timestamp: Jan 1, 2022, 9:56:54 AM
```

In the browser, we can see that 11 versions are deployed (as of June 28th). Click on the [most recent version](https://testnet.cspr.live/contract/4f4da49a080efdf3a66ddc279f050c0700618db675507734a46a8a1bb784575f) and we see the following:

```yaml
- Contract Hash: 4f4da49a080efdf3a66ddc279f050c0700618db675507734a46a8a1bb784575f
- Contract Package Hash: dde7472639058717a42e22d297d6cf3e07906bb57bc28efceac3677f8a3dc83b
- Protocol Version: 1.4.4 
- Contract Type: DeFi 
- Timestamp: Jan 6, 2022, 8:26:14 PM
```

There is a list of entry points (eg. `add_liquidity`, `get_amount_in`, etc...), and named keys:

```yaml
- contract_purse: uref-d20483554e87b6f2f59e31d1bb1804a6be8f6ceba2b13dd160631d6e9c6e97c5-007
- global: uref-7f64c2911048b9563ff9336820db8f0751ca6f0227d0321272e086e8312180ce-007
- update_authority: account-hash-d0bc9ca1353597c4004b8f881b397a89c1779004f5e547e04b57c2e7967c6269
```

Let's find the WCSPR address, which is stored in the `global` key. The dictionary explorer allows to retrieve the value of a key. By querying `WCSPR` in the [explorer](https://testnet.cspr.live/dictionary/uref-7f64c2911048b9563ff9336820db8f0751ca6f0227d0321272e086e8312180ce-007), we get: `7616e33946f7835f412a3ab240edbad379e83e7eaab27ba20fb488f03b8a76cb`.

What do we know about it ?

```yaml
- Contract Hash: 7616e33946f7835f412a3ab240edbad379e83e7eaab27ba20fb488f03b8a76cb
- Contract Package Hash: 1ecd0ff313d140de7fb7cc7c5f22c9d767453dc70777272d36e65053b2fed87b
- Protocol Version: 1.4.3 
- Contract Type: Custom ERC-20 
- Timestamp: Jan 1, 2022, 9:19:32 AM
```

We observe several named keys:

```yaml
- allowances: uref-6ce72c828fbc66a9ce7551c1f24cb95d583e8d5dae6dda4192cef258b41a6648-007
- balances: uref-2d1c7c43f92c48be88edf2bfa0f561f0f694f4a8d760b27cbc950140f1222593-007
- contract_package_hash: uref-eb4ea7ca9f9c5450f3d2c4b00f8ad5c9302bf6fcd58fa9fa436266deba930505-007
- contract_purse: uref-c9733355d61aa2a36721d9d1081eebcfe5dde94f82386b3d75163fee894d292a-007
- decimals: uref-0e4275bcc60e92b6abd5fba307bac9683e3e1a798685f64eebc2d1293f68b782-007
- name: uref-67789e1b6ebf5bec2cf8f7c5e390b45e5f7ffe46772194d2aa332cbed78e8fa5-007
- symbol: uref-3d6fb47d6c8e5d8e65430ae40c8469f4dd37979860320e361886f7c7fd52d76b-007
- total_supply: uref-89d1b1aa730691ee80bfe8536ee44ad968cd79edab8872681b6c4b72e55c8355-007
```

What is in that [purse](https://testnet.cspr.live/uref/uref-c9733355d61aa2a36721d9d1081eebcfe5dde94f82386b3d75163fee894d292a-007) ? 1,006,911.17344 CSPR. Can we retrieve some of it ?
