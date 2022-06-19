CHAIN_NAME="casper-test"
NODE_ADDRESS="http://16.162.124.124:7777"
PATH_TO_YOUR_KEY="./wallet"


# Get the account hash
ACCOUNT_HASH=$(casper-client account-address -p $PATH_TO_YOUR_KEY/public_key.pem)

# Get the global state
STATE_ROOT_HASH=$(casper-client get-state-root-hash --node-address $NODE_ADDRESS | jq -r '.result.state_root_hash')

NAMED_KEYS=$(casper-client query-global-state \
    --node-address $NODE_ADDRESS \
    --state-root-hash $STATE_ROOT_HASH \
    --key $ACCOUNT_HASH | jq -r '.result.stored_value.Account.named_keys')
echo "Named keys: $NAMED_KEYS"