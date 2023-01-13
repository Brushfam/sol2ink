// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

use scale::{
    Decode,
    Encode,
};

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Custom(String),
}

pub enum Enum {
    First,
    Second,
}


#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Struct {
    field_1: u128,
    field_2: u128,
}


#[openbrush::wrapper]
pub type ERC20Ref = dyn ERC20;

#[openbrush::trait_definition]
pub trait ERC20 {
    #[ink(message)]
    fn balances(&self) -> Mapping<AccountId, u128>;

    #[ink(message)]
    fn allowances(&self) -> Mapping<(AccountId, AccountId), u128>;

    #[ink(message)]
    fn total_supply(&self) -> u128;

    #[ink(message)]
    fn name(&self) -> String;

    #[ink(message)]
    fn symbol(&self) -> String;

}
