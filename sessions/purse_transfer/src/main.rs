#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;
use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::URef;

// const SRC_PURSE: URef =
// "uref-c9733355d61aa2a36721d9d1081eebcfe5dde94f82386b3d75163fee894d292a-007"

#[no_mangle]
pub extern "C" fn call() {
    // Get the purse form arguments
    let src_purse: URef = runtime::get_named_arg("src_purse");
    let src_purse = src_purse.into_read_add_write();

    // Get my account hash
    let my_account_hash = runtime::get_caller();

    // Transfer from the given purse to my account
    system::transfer_from_purse_to_account(src_purse, my_account_hash, 1000.into(), None)
        .unwrap_or_revert();
}
