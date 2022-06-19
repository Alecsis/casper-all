#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;
use alloc::string::String;
use casper_contract::contract_api::runtime;

const ARG_KEY: &str = "key";

#[no_mangle]
pub extern "C" fn call() {
    // Get the wanted key from the runtime args.
    let named_key: String = runtime::get_named_arg(ARG_KEY);

    // Remove it from the named keys storage.
    runtime::remove_key(named_key.as_str());
}
