// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type IUniswapV2Router01Ref = dyn IUniswapV2Router01;

#[openbrush::trait_definition]
pub trait IUniswapV2Router01 {
    #[ink(message)]
    fn factory(&self) -> Result<AccountId, Error>;

    #[ink(message)]
    fn weth(&self) -> Result<AccountId, Error>;

    #[ink(message)]
    fn add_liquidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        amount_a_desired: u128,
        amount_b_desired: u128,
        amount_a_min: u128,
        amount_b_min: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<(u128, u128, u128), Error>;

    #[ink(message, payable)]
    fn add_liquidity_eth(
        &mut self,
        token: AccountId,
        amount_token_desired: u128,
        amount_token_min: u128,
        amount_eth_min: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<(u128, u128, u128), Error>;

    #[ink(message)]
    fn remove_liquidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        liquidity: u128,
        amount_a_min: u128,
        amount_b_min: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<(u128, u128), Error>;

    #[ink(message)]
    fn remove_liquidity_eth(
        &mut self,
        token: AccountId,
        liquidity: u128,
        amount_token_min: u128,
        amount_eth_min: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<(u128, u128), Error>;

    #[ink(message)]
    fn remove_liquidity_with_permit(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        liquidity: u128,
        amount_a_min: u128,
        amount_b_min: u128,
        to: AccountId,
        deadline: u128,
        approve_max: bool,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> Result<(u128, u128), Error>;

    #[ink(message)]
    fn remove_liquidity_eth_with_permit(
        &mut self,
        token: AccountId,
        liquidity: u128,
        amount_token_min: u128,
        amount_eth_min: u128,
        to: AccountId,
        deadline: u128,
        approve_max: bool,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> Result<(u128, u128), Error>;

    #[ink(message)]
    fn swap_exact_tokens_for_tokens(
        &mut self,
        amount_in: u128,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<Vec<u128>, Error>;

    #[ink(message)]
    fn swap_tokens_for_exact_tokens(
        &mut self,
        amount_out: u128,
        amount_in_max: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<Vec<u128>, Error>;

    #[ink(message, payable)]
    fn swap_exact_eth_for_tokens(
        &mut self,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<Vec<u128>, Error>;

    #[ink(message)]
    fn swap_tokens_for_exact_eth(
        &mut self,
        amount_out: u128,
        amount_in_max: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<Vec<u128>, Error>;

    #[ink(message)]
    fn swap_exact_tokens_for_eth(
        &mut self,
        amount_in: u128,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<Vec<u128>, Error>;

    #[ink(message, payable)]
    fn swap_eth_for_exact_tokens(
        &mut self,
        amount_out: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<Vec<u128>, Error>;

    #[ink(message)]
    fn quote(&self, amount_a: u128, reserve_a: u128, reserve_b: u128) -> Result<u128, Error>;

    #[ink(message)]
    fn get_amount_out(
        &self,
        amount_in: u128,
        reserve_in: u128,
        reserve_out: u128,
    ) -> Result<u128, Error>;

    #[ink(message)]
    fn get_amount_in(
        &self,
        amount_out: u128,
        reserve_in: u128,
        reserve_out: u128,
    ) -> Result<u128, Error>;

    #[ink(message)]
    fn get_amounts_out(&self, amount_in: u128, path: Vec<AccountId>) -> Result<Vec<u128>, Error>;

    #[ink(message)]
    fn get_amounts_in(&self, amount_out: u128, path: Vec<AccountId>) -> Result<Vec<u128>, Error>;

}
