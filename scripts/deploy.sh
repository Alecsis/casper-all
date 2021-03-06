CHAIN_NAME="casper-test"
NODE_ADDRESS="http://16.162.124.124:7777"
PATH_TO_YOUR_KEY="./wallet"
CONTRACT_NAME="create_writable_purse"

# Get the account hash
ACCOUNT_HASH=$(casper-client account-address -p $PATH_TO_YOUR_KEY/public_key.pem)
echo "Account hash: $ACCOUNT_HASH"

# Get the global state
STATE_ROOT_HASH=$(casper-client get-state-root-hash --node-address $NODE_ADDRESS | jq -r '.result.state_root_hash')
echo "Global state root hash: $STATE_ROOT_HASH"

GLOBAL_STATE=$(casper-client query-global-state \
    --node-address $NODE_ADDRESS \
    --state-root-hash $STATE_ROOT_HASH \
    --key $ACCOUNT_HASH)
echo "Global state: $GLOBAL_STATE"

DEPLOY_HASH=$(casper-client put-deploy \
    --node-address $NODE_ADDRESS \
    --chain-name casper-test \
    --secret-key $PATH_TO_YOUR_KEY/secret_key.pem \
    --payment-amount 50000000000 \
    --session-path ./target/wasm32-unknown-unknown/release/$CONTRACT_NAME.wasm | jq -r '.result.deploy_hash')
echo "Deploy result: $DEPLOY_HASH"

DEPLOY_RESULT=$(casper-client get-deploy \
    --node-address $NODE_ADDRESS \
    $DEPLOY_HASH)
echo "Deploy result: $DEPLOY_RESULT"