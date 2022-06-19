CHAIN_NAME="casper-test"
NODE_ADDRESS="http://16.162.124.124:7777"
PATH_TO_YOUR_KEY="/home/ubuntu/casper/vault/wallet"

# Get the account hash
ACCOUNT_HASH=$(casper-client account-address -p $PATH_TO_YOUR_KEY/public_key.pem)
echo "Account hash: $ACCOUNT_HASH"

# Get the global state
STATE_ROOT_HASH=$(casper-client get-state-root-hash --node-address $NODE_ADDRESS | jq -r '.result.state_root_hash')
echo "Root hash: $STATE_ROOT_HASH"

GLOBAL_STATE=$(casper-client query-global-state \
    --node-address $NODE_ADDRESS \
    --state-root-hash $STATE_ROOT_HASH \
    --key $ACCOUNT_HASH | jq -r '.result')
echo "Account state: $GLOBAL_STATE"