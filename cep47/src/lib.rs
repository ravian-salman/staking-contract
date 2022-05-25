#![no_std]
#[macro_use]
extern crate alloc;

mod cep47;
pub mod data;
pub mod event;
pub mod detail;
pub mod address;


pub use cep47::{Error, CEP20STK};
pub use contract_utils;

use alloc::{collections::BTreeMap, string::String};
use casper_types::U256;
pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;
