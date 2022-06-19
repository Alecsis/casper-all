CHAIN_NAME="casper-test"
NODE_ADDRESS="http://16.162.124.124:7777"
PATH_TO_YOUR_KEY="./wallet"
DEPLOY_HASH="3840fa9386bce722d7632ed87d6e58702072dc6a4d19d5dc5567cf8684b98b00"

# Get the account hash
ACCOUNT_HASH=$(casper-client account-address -p $PATH_TO_YOUR_KEY/public_key.pem)
echo "Account hash: $ACCOUNT_HASH"

# Get the global state
STATE_ROOT_HASH=$(casper-client get-state-root-hash --node-address $NODE_ADDRESS | jq -r '.result.state_root_hash')
echo "Global state root hash: $STATE_ROOT_HASH"

# GLOBAL_STATE=$(casper-client query-global-state \
#     --node-address $NODE_ADDRESS \
#     --state-root-hash $STATE_ROOT_HASH \
#     --key $ACCOUNT_HASH -key "deploy-$DEPLOY_HASH")
# echo "Global state: $GLOBAL_STATE"

casper-client put-deploy \
    --node-address $NODE_ADDRESS \
    --chain-name $CHAIN_NAME \
    --secret-key $PATH_TO_YOUR_KEY/secret_key.pem \
    --payment-amount 50000000000 \
    --session-name "contract-hash" \
    --session-entry-point "get_total_balances"
