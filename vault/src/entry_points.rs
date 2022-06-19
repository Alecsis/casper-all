use crate::constants::*;
use alloc::{string::ToString, vec};
use casper_types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter};

pub fn default() -> EntryPoints {
    let deposit_entrypoint = EntryPoint::new(
        ENTRY_POINT_DEPOSIT,
        vec![
            Parameter::new(ARG_DEPOSITING_PURSE.to_string(), CLType::URef),
            Parameter::new(ARG_AMOUNT.to_string(), CLType::U512),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let get_balance_entrypoint = EntryPoint::new(
        ENTRY_POINT_GET_BALANCE,
        vec![Parameter::new(ARG_ADDRESS.to_string(), CLType::Key)],
        CLType::U512,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let get_total_balance_entrypoint = EntryPoint::new(
        ENTRY_POINT_GET_TOTAL_BALANCE,
        vec![],
        CLType::U512,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let withdraw_entrypoint = EntryPoint::new(
        ENTRY_POINT_WITHDRAW,
        vec![
            Parameter::new(ARG_DEPOSITING_PURSE.to_string(), CLType::URef),
            Parameter::new(ARG_AMOUNT.to_string(), CLType::U512),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    // Add the entry points to the contract.
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(deposit_entrypoint);
    entry_points.add_entry_point(get_balance_entrypoint);
    entry_points.add_entry_point(get_total_balance_entrypoint);
    entry_points.add_entry_point(withdraw_entrypoint);

    entry_points
}
