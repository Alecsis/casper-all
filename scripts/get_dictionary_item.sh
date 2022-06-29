NODE_ADDRESS="http://16.162.124.124:7777"
STATE_ROOT_HASH="bb83f2326a0bedb723cd223e8065a41d9c1531a4c01b284179beca0605c96da9"
DICITONARY_NAME="global"
DICTIONARY_ITEM_KEY="WCSPR"
SEED_UREF="uref-7f64c2911048b9563ff9336820db8f0751ca6f0227d0321272e086e8312180ce-007"

casper-client get-dictionary-item \
    --state-root-hash $STATE_ROOT_HASH \
    --dictionary-item-key $DICTIONARY_ITEM_KEY \
    --dictionary-name $DICITONARY_NAME \
    --seed-uref $SEED_UREF \
    --node-address $NODE_ADDRESS | jq