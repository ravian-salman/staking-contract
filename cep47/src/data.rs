use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime::get_call_stack, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{system::CallStackElement, ContractPackageHash, Key, URef, U256};
use contract_utils::{get_key, key_and_value_to_str, key_to_str, set_key, Dict};
use crate::detail;
use crate::{event::CEP47Event};

const STAKERS_DICT: &str = "stakers";
const AMOUNT_STAKED_BY_ADDRESS_DICT: &str = "amount_staked_by_addresses_dict";
const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

pub const NAME: &str = "address";
pub const ADDRESS: &str = "address";
pub const STAKING_STARTS: &str = "staking_starts";
pub const STAKING_ENDS: &str = "staking_ends";
pub const WITHDRAW_STARTS: &str = "withdraw_starts";
pub const WITHDRAW_ENDS: &str = "withdraw_ends";
pub const STAKING_TOTAL: &str = "staking_total";





pub struct StakedTokens {
    addresses_staked_dict: Dict,
}

impl StakedTokens {
    pub fn instance() -> StakedTokens {
        StakedTokens {
            addresses_staked_dict: Dict::instance(AMOUNT_STAKED_BY_ADDRESS_DICT),
        }
    }

    pub fn init() {
        Dict::init(AMOUNT_STAKED_BY_ADDRESS_DICT);
    }

    pub fn get_amount_staked_by_address(&self, address: &Key) -> Option<U256> {
        self.addresses_staked_dict.get(&key_to_str(address))
    }


    pub fn add_stake(&self, owner: &Key, amount: &U256) {
        let staked_amount = self.get_amount_staked_by_address(owner).unwrap();
        let new_amount = staked_amount + amount;
        self.addresses_staked_dict
            .set(&key_to_str(owner),new_amount);
    }

    pub fn withdraw_stake(&self, owner: &Key, amount: &U256) {
        let staked_amount = self.get_amount_staked_by_address(owner).unwrap();
        let new_amount = staked_amount - amount;
        self.addresses_staked_dict
            .set(&key_to_str(owner),new_amount);
    }
}

pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn address() -> String {
    get_key(ADDRESS).unwrap_or_revert()
}

pub fn set_address(address: String) {
    set_key(ADDRESS, address);
}

pub fn staking_starts() -> U256 {
    get_key(STAKING_STARTS).unwrap_or_revert()
}

pub fn set_staking_starts(staking_starts: U256) {
    set_key(STAKING_STARTS, staking_starts);
}

pub fn staking_ends() -> U256 {
    get_key(STAKING_ENDS).unwrap_or_revert()
}

pub fn set_staking_ends(staking_ends: U256) {
    set_key(STAKING_ENDS, staking_ends);
}

pub fn withdraw_starts() -> U256 {
    get_key(WITHDRAW_STARTS).unwrap_or_default()
}

pub fn set_withdraw_starts(withdraw_starts: U256) {
    set_key(WITHDRAW_STARTS, withdraw_starts);
}

pub fn withdraw_ends() -> U256 {
    get_key(WITHDRAW_ENDS).unwrap_or_default()
}

pub fn set_withdraw_ends(withdraw_ends: U256) {
    set_key(WITHDRAW_STARTS, withdraw_ends);
}

pub fn staking_total() -> U256 {
    get_key(STAKING_TOTAL).unwrap_or_default()
}

pub fn set_staking_total(staking_total: U256) {
    set_key(STAKING_TOTAL, staking_total);
}

pub fn contract_package_hash() -> ContractPackageHash {
    let call_stacks = get_call_stack();
    let last_entry = call_stacks.last().unwrap_or_revert();
    let package_hash: Option<ContractPackageHash> = match last_entry {
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => Some(*contract_package_hash),
        _ => None,
    };
    package_hash.unwrap_or_revert()
}

pub fn emit(event: &CEP47Event) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    match event {
        CEP47Event::Stake {
            amount
        } => {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "stake".to_string());
                param.insert("staker",Key::from(detail::get_immediate_caller_address().ok().unwrap()).to_formatted_string());
                param.insert("stake_amount", amount.to_string());
                events.push(param);
        }
        CEP47Event::Withdraw { amount} => {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "withdraw".to_string());
                param.insert("staker", Key::from(detail::get_immediate_caller_address().ok().unwrap()).to_formatted_string());
                param.insert("withdrawn_amount", amount.to_string());
                events.push(param);
            
        }
        CEP47Event::AddReward {
            reward_amount,
            withdrawable_amount
        } => {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "add_reward".to_string());
                param.insert("reward_amount", reward_amount.to_string());
                param.insert("withdrawable_amount", withdrawable_amount.to_string());
                events.push(param);
        }
    };
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
