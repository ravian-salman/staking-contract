use crate::{
    data::{self, StakedTokens},
    event::CEP47Event
};
use casper_types::RuntimeArgs;
use alloc::{string::String};
use casper_types::{ApiError, Key, U256, runtime_args, ContractPackageHash};
use contract_utils::{ContractContext, ContractStorage};
// use core::convert::TryInto;
use casper_contract::contract_api::runtime;
use casper_types::ContractHash;
use crate::detail;

#[repr(u16)]
pub enum Error {
    PermissionDenied = 1,
    WrongArguments = 2,
    NotRequiredStake = 3,
    BadTiming = 4,
    InvalidContext = 5,
    NegativeReward =6,
    NegativeWithdrawableReward = 7
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait CEP20STK<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self,
        name: String,
        address: String, 
        staking_starts: u64,
        staking_ends: u64,
        withdraw_starts: u64,
        withdraw_ends: u64,
        staking_total: U256
        ) {
        data::set_name(name);
        data::set_address(address);
        data::set_staking_starts(staking_starts);
        data::set_staking_ends(staking_ends);
        data::set_withdraw_starts(withdraw_starts);
        data::set_withdraw_ends(withdraw_ends);
        data::set_staking_total(staking_total);
        StakedTokens::init();
    }

    fn name(&self) -> String {
        data::name()
    }

    fn address(&self) -> String {
        data::address()
    }

    fn staking_starts(&self) -> u64 {
        data::staking_starts()
    }

    fn staking_ends(&self) -> u64 {
        data::staking_ends()
    }

    fn withdraw_starts(&self) -> u64 {
        data::withdraw_starts()
    }

    fn withdraw_ends(&self) -> u64 {
        data::withdraw_ends()
    }

    fn staking_total(&self) -> U256 {
        data::staking_total()
    }

    fn amount_staked(&self, staker: Key) -> U256 {
        StakedTokens::instance().get_amount_staked_by_address(&staker).unwrap()
        }



    fn stake(
        &mut self,
        amount: U256
    ) -> Result<U256, Error> {

        if amount < U256::from(2) {
            return Err(Error::NotRequiredStake);
        } 

        if runtime::get_blocktime() < self.staking_starts() {
            return Err(Error::BadTiming);
        }

        if runtime::get_blocktime() >= self.staking_ends() {
            return Err(Error::BadTiming);
        }

        let stakers_dict = StakedTokens::instance();
        let lower_contracthash =
        "contract-c9a9e704604260416bf908cb6274e5d765b36164cf1fb9597a0df67ec4063bfa".to_lowercase();
        let contract_hash = ContractHash::from_formatted_str(&lower_contracthash).unwrap();
        
        let lower_contractpackagehash = "hash-wasmc4929e7fcb71772c1cb39ebb702a70d036b0ad4f9caf420d3fd377f749dfdb17".to_lowercase();
        let contract_package_hash = ContractPackageHash::from_formatted_str(&lower_contractpackagehash).unwrap(); 

        let args = runtime_args! {
            "owner" => detail::get_immediate_caller_address()?,
            "recipient" => contract_package_hash,
            "amount" => amount
        };
        runtime::call_contract(contract_hash,"transfer_from", args);
        stakers_dict.add_stake(&Key::from(detail::get_immediate_caller_address()?), &amount);

        self.emit(CEP47Event::Stake {
            amount,
        });
        Ok(amount)
    }


    fn withdraw(
        &mut self,
        amount: U256
    ) -> Result<U256, Error> {

        if amount < U256::from(2) {
           return Err(Error::NotRequiredStake);
        } 

        if runtime::get_blocktime() < self.staking_starts() {
            return Err(Error::BadTiming);
        }

        if runtime::get_blocktime() >= self.staking_ends() {
            return Err(Error::BadTiming);
        }

        let stakers_dict = StakedTokens::instance();
        let lower_contracthash =
        "contract-c9a9e704604260416bf908cb6274e5d765b36164cf1fb9597a0df67ec4063bfa".to_lowercase();
        let contract_hash = ContractHash::from_formatted_str(&lower_contracthash).unwrap();

        let lower_contractpackagehash = "hash-4929e7fcb71772c1cb39ebb702a70d036b0ad4f9caf420d3fd377f749dfdb17".to_lowercase();
        let contract_package_hash = ContractPackageHash::from_formatted_str(&lower_contractpackagehash).unwrap();

        let args = runtime_args! {
            "recipient" => detail::get_immediate_caller_address()?,
            "amount" => amount
    
        };
        runtime::call_contract(contract_hash,"transfer", args);
        stakers_dict.withdraw_stake(&Key::from(detail::get_immediate_caller_address()?), &amount);

        self.emit(CEP47Event::Stake {
            amount,
        });
        Ok(amount)
    }

    fn add_reward(
        &mut self,
        reward_amount: U256,
        withdrawable_amount: U256
    ) -> Result<U256, Error> {

        if runtime::get_blocktime() >= self.withdraw_starts() {
            return Err(Error::PermissionDenied)
        }

        if reward_amount <= U256::from(0) {
            return Err(Error::NegativeReward)
        }

        if withdrawable_amount < U256::from(0) {
            return Err(Error::NegativeWithdrawableReward)
        }

        if withdrawable_amount > reward_amount {
            return Err(Error::NegativeWithdrawableReward)
        }

        let lower_contracthash =
        "contract-c9a9e704604260416bf908cb6274e5d765b36164cf1fb9597a0df67ec4063bfa".to_lowercase();
        let contract_hash = ContractHash::from_formatted_str(&lower_contracthash).unwrap();
        
        let lower_contractpackagehash = "hash-wasmc4929e7fcb71772c1cb39ebb702a70d036b0ad4f9caf420d3fd377f749dfdb17".to_lowercase();
        let contract_package_hash = ContractPackageHash::from_formatted_str(&lower_contractpackagehash).unwrap(); 

        let args = runtime_args! {
            "owner" => detail::get_immediate_caller_address()?,
            "recipient" => contract_package_hash,
            "amount" => reward_amount + withdrawable_amount
    
        };
        runtime::call_contract(contract_hash,"transfer_from", args);

        self.emit(CEP47Event::AddReward
             {
            reward_amount,
            withdrawable_amount
        });
        Ok(reward_amount)
    }
    

    fn emit(&mut self, event: CEP47Event) {
        data::emit(&event);
    }
}
