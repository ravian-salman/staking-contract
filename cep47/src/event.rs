
use casper_types::U256;


pub enum CEP47Event {
    Stake {
        amount: U256,
    },
    Withdraw {
        amount: U256,
    },
    AddReward {
        reward_amount: U256,
        withdrawable_amount: U256
    }

}
