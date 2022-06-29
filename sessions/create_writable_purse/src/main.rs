#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;
use casper_contract::contract_api::{runtime, system};
use casper_types::URef;

// const SRC_PURSE: URef =
// "uref-c9733355d61aa2a36721d9d1081eebcfe5dde94f82386b3d75163fee894d292a-007"

#[no_mangle]
pub extern "C" fn call() {
    let default_purse: URef = system::create_purse();
    let writable_purse: URef = system::create_purse().into_read_add_write();

    // Add it to my named keys
    runtime::put_key("default_purse", default_purse.into());
    runtime::put_key("writable_purse", writable_purse.into());
}
