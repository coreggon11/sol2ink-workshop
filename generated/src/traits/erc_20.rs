// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use openbrush::storage::Mapping;
pub use openbrush::traits::AccountId;
pub use openbrush::traits::AccountIdExt;
pub use openbrush::traits::String;
pub use openbrush::traits::ZERO_ADDRESS;
use scale::{Decode, Encode};

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Custom(String),
}



#[openbrush::wrapper]
pub type ERC20Ref = dyn ERC20;

#[openbrush::trait_definition]
pub trait ERC20 {
    /// @dev Returns the name of the token.
    #[ink(message)]
    fn name(&self) -> Result<String, Error>;

    /// @dev Returns the symbol of the token, usually a shorter version of the
    /// name.
    #[ink(message)]
    fn symbol(&self) -> Result<String, Error>;

    /// @dev Returns the number of decimals used to get its user representation.
    /// For example, if `decimals` equals `2`, a balance of `505` tokens should
    /// be displayed to a user as `5.05` (`505 / 10 ** 2`).
    ///
    /// Tokens usually opt for a value of 18, imitating the relationship between
    /// Ether and Wei. This is the default value returned by this function, unless
    /// it's overridden.
    ///
    /// NOTE: This information is only used for _display_ purposes: it in
    /// no way affects any of the arithmetic of the contract, including
    /// {IERC20-balanceOf} and {IERC20-transfer}.
    #[ink(message)]
    fn decimals(&self) -> Result<u8, Error>;

    /// @dev See {IERC20-totalSupply}.
    #[ink(message)]
    fn total_supply(&self) -> Result<u128, Error>;

    /// @dev See {IERC20-balanceOf}.
    #[ink(message)]
    fn balance_of(&self, account: AccountId) -> Result<u128, Error>;

    /// @dev See {IERC20-transfer}.
    ///
    /// Requirements:
    ///
    /// - `to` cannot be the zero address.
    /// - the caller must have a balance of at least `amount`.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, amount: u128) -> Result<bool, Error>;

    /// @dev See {IERC20-allowance}.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Result<u128, Error>;

    /// @dev See {IERC20-approve}.
    ///
    /// NOTE: If `amount` is the maximum `uint256`, the allowance is not updated on
    /// `transferFrom`. This is semantically equivalent to an infinite approval.
    ///
    /// Requirements:
    ///
    /// - `spender` cannot be the zero address.
    #[ink(message)]
    fn approve(&mut self, spender: AccountId, amount: u128) -> Result<bool, Error>;

    /// @dev See {IERC20-transferFrom}.
    ///
    /// Emits an {Approval} event indicating the updated allowance. This is not
    /// required by the EIP. See the note at the beginning of {ERC20}.
    ///
    /// NOTE: Does not update the allowance if the current allowance
    /// is the maximum `uint256`.
    ///
    /// Requirements:
    ///
    /// - `from` and `to` cannot be the zero address.
    /// - `from` must have a balance of at least `amount`.
    /// - the caller must have allowance for ``from``'s tokens of at least
    /// `amount`.
    #[ink(message)]
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<bool, Error>;

    /// @dev Atomically increases the allowance granted to `spender` by the caller.
    ///
    /// This is an alternative to {approve} that can be used as a mitigation for
    /// problems described in {IERC20-approve}.
    ///
    /// Emits an {Approval} event indicating the updated allowance.
    ///
    /// Requirements:
    ///
    /// - `spender` cannot be the zero address.
    #[ink(message)]
    fn increase_allowance(&mut self, spender: AccountId, added_value: u128) -> Result<bool, Error>;

    /// @dev Atomically decreases the allowance granted to `spender` by the caller.
    ///
    /// This is an alternative to {approve} that can be used as a mitigation for
    /// problems described in {IERC20-approve}.
    ///
    /// Emits an {Approval} event indicating the updated allowance.
    ///
    /// Requirements:
    ///
    /// - `spender` cannot be the zero address.
    /// - `spender` must have allowance for the caller of at least
    /// `subtractedValue`.
    #[ink(message)]
    fn decrease_allowance(
        &mut self,
        spender: AccountId,
        subtracted_value: u128,
    ) -> Result<bool, Error>;

}
