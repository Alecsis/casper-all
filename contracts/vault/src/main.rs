#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;
use alloc::string::ToString;

use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{contracts::NamedKeys, CLValue, Key, URef, U512};

mod constants;
use constants::*;

// mod errors;
// use errors::VaultError;

mod entry_points;

/// # Purpose
/// * Returns how much CSPR the given `address` owns.
/// # Arguments
/// * `address` - `Key` -> Address that we are looking for it's token balance.
/// # Returns
/// * `balance` - `U512` -> The given `address`'s balance.
#[no_mangle]
pub extern "C" fn get_balance() {
    // 1. Read the address from the runtime args.
    let address: Key = runtime::get_named_arg(ARG_ADDRESS);

    // 2. Get the Uref of the balances storage.
    let balances_seed_uref = runtime::get_key(BALANCES)
        .unwrap_or_revert()
        .into_uref()
        .unwrap_or_revert();

    // 3. Read the balance of the given address from the balances storage.
    let balance: U512 = storage::dictionary_get(balances_seed_uref, &address.to_string())
        .unwrap_or_default()
        .unwrap_or_default();

    // 4. Return the balance.
    let return_value = CLValue::from_t(balance).unwrap_or_revert();
    runtime::ret(return_value)
}

#[no_mangle]
pub extern "C" fn get_total_balance() {
    // Get the URef of the purse.
    let purse_uref = runtime::get_key(PURSE)
        .unwrap_or_revert()
        .into_uref()
        .unwrap_or_revert();

    // Get the balance of the purse.
    let balance = system::get_purse_balance(purse_uref).unwrap_or_revert();
    let return_value = CLValue::from_t(balance).unwrap_or_revert();
    runtime::ret(return_value)
}

/// # Purpose
/// * Deposits the given amount of CSPR to the given `address`'s purse.
/// # Arguments
/// * `address` - `Key` -> Address that we are depositing to.
/// * `amount` - `U512` -> Amount of CSPR to deposit.
#[no_mangle]
pub extern "C" fn deposit() {
    // Read the depositing purse from the runtime args.
    let source: URef = runtime::get_named_arg(ARG_DEPOSITING_PURSE);
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);

    // Get caller address
    let caller: Key = runtime::get_caller().into();

    // Get the Uref of the balances storage.
    let balances_seed_uref = runtime::get_key(BALANCES)
        .unwrap_or_revert()
        .into_uref()
        .unwrap_or_revert();

    // Get the deposited balance of the caller
    let balance_before: U512 = storage::dictionary_get(balances_seed_uref, &caller.to_string())
        .unwrap_or_default()
        .unwrap_or_default();

    // Get the Uref of the purse storage.
    let target = runtime::get_key(PURSE)
        .unwrap_or_revert()
        .into_uref()
        .unwrap_or_revert();

    // Transfer from caller's purse to the contract's purse.
    system::transfer_from_purse_to_purse(source, target, amount, None).unwrap_or_revert();

    // Update the balances storage.
    let balance_after = balance_before.checked_add(amount).unwrap_or_revert();
    storage::dictionary_put(balances_seed_uref, &caller.to_string(), balance_after);
}

/// # Purpose
/// * Withdraws the given amount of CSPR from the given `address`'s purse.
/// # Arguments
/// * `address` - `Key` -> Address that we are withdrawing from.
/// * `amount` - `U512` -> Amount of CSPR to withdraw.
#[no_mangle]
pub extern "C" fn withdraw() {
    // Read the withdrawing purse from the runtime args.
    let target: URef = runtime::get_named_arg(ARG_WITHDRAWING_PURSE);
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);

    // Get caller address
    let caller: Key = runtime::get_caller().into();

    // Get the Uref of the balances storage.
    let balances_seed_uref = runtime::get_key(BALANCES)
        .unwrap_or_revert()
        .into_uref()
        .unwrap_or_revert();

    // Get the deposited balance of the caller
    let balance_before: U512 = storage::dictionary_get(balances_seed_uref, &caller.to_string())
        .unwrap_or_default()
        .unwrap_or_default();

    // Make sure that the new balance doesn't go below 0.
    let balance_after = balance_before.checked_sub(amount).unwrap_or_revert();

    // Get the Uref of the purse storage.
    let source = runtime::get_key(PURSE)
        .unwrap_or_revert()
        .into_uref()
        .unwrap_or_revert();

    // Transfer from contract's purse to the caller's purse.
    system::transfer_from_purse_to_purse(source, target, balance_before, None).unwrap_or_revert();

    // Update the balances storage.
    storage::dictionary_put(balances_seed_uref, &caller.to_string(), balance_after);
}

#[no_mangle]
pub extern "C" fn call() {
    // Create the contract's named keys.
    let mut named_keys = NamedKeys::new();

    // There is a deposit purse on which the users can deposit funds.
    let purse: URef = system::create_purse();
    named_keys.insert(PURSE.to_string(), purse.into());
    runtime::remove_key(PURSE);

    // There is a ledger to keep track of the deposits.
    let balances_seed_uref = storage::new_dictionary(BALANCES).unwrap_or_revert();
    named_keys.insert(BALANCES.to_string(), balances_seed_uref.into());
    runtime::remove_key(BALANCES);

    // The entry points are defined in a separate file.
    let entry_points = entry_points::default();

    // Create the contract
    let hash_name = Some(VAULT_PACKAGE_NAME.to_string());
    let uref_name = Some(VAULT_ACCESS_TOKEN_NAME.to_string());

    let (contract_hash, contract_version) =
        storage::new_locked_contract(entry_points, Some(named_keys), hash_name, uref_name);

    runtime::put_key(VAULT_HASH_KEY, contract_hash.into());

    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(VAULT_VERSION_KEY, version_uref.into());
}
