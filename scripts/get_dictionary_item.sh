NODE_ADDRESS="http://16.162.124.124:7777"
DICTIONARY_NAME="global"
DICTIONARY_ITEM_KEY="WCSPR"
SEED_UREF="uref-7f64c2911048b9563ff9336820db8f0751ca6f0227d0321272e086e8312180ce-007"

STATE_ROOT_HASH=$(casper-client get-state-root-hash --node-address $NODE_ADDRESS | jq -r '.result.state_root_hash')

casper-client get-dictionary-item \
    --state-root-hash $STATE_ROOT_HASH \
    --dictionary-item-key $DICTIONARY_ITEM_KEY \
    --dictionary-name $DICTIONARY_NAME \
    --seed-uref $SEED_UREF \
    --node-address $NODE_ADDRESS | jq