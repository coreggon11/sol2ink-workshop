// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use crate::{impls, traits::*};
pub use openbrush::storage::Mapping;
pub use openbrush::traits::AccountId;
pub use openbrush::traits::AccountIdExt;
use openbrush::traits::Storage;
pub use openbrush::traits::String;
pub use openbrush::traits::ZERO_ADDRESS;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub balances: Mapping<AccountId, u128>,
    pub allowances: Mapping<(AccountId, AccountId), u128>,
    pub total_supply: u128,
    pub name: String,
    pub symbol: String,
    pub _reserved: Option<()>,
}

impl<T: Storage<Data>> ERC20 for T {
    /// @dev Returns the name of the token.
    fn name(&self) -> Result<String, Error> {
        return Ok(self.data().name.clone());
    }

    /// @dev Returns the symbol of the token, usually a shorter version of the
    /// name.
    fn symbol(&self) -> Result<String, Error> {
        return Ok(self.data().symbol.clone());
    }

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
    fn decimals(&self) -> Result<u8, Error> {
        return Ok(18);
    }

    /// @dev See {IERC20-totalSupply}.
    fn total_supply(&self) -> Result<u128, Error> {
        return Ok(self.data().total_supply);
    }

    /// @dev See {IERC20-balanceOf}.
    fn balance_of(&self, account: AccountId) -> Result<u128, Error> {
        return Ok(self.data().balances.get(&account).unwrap_or_default());
    }

    /// @dev See {IERC20-transfer}.
    ///
    /// Requirements:
    ///
    /// - `to` cannot be the zero address.
    /// - the caller must have a balance of at least `amount`.
    fn transfer(&mut self, to: AccountId, amount: u128) -> Result<bool, Error> {
        let mut owner: AccountId = Self::env().caller();
        self._transfer(owner, to, amount)?;
        return Ok(true);
    }

    /// @dev See {IERC20-allowance}.
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Result<u128, Error> {
        return Ok(self
            .data()
            .allowances
            .get(&(owner, spender))
            .unwrap_or_default());
    }

    /// @dev See {IERC20-approve}.
    ///
    /// NOTE: If `amount` is the maximum `uint256`, the allowance is not updated on
    /// `transferFrom`. This is semantically equivalent to an infinite approval.
    ///
    /// Requirements:
    ///
    /// - `spender` cannot be the zero address.
    fn approve(&mut self, spender: AccountId, amount: u128) -> Result<bool, Error> {
        let mut owner: AccountId = Self::env().caller();
        self._approve(owner, spender, amount)?;
        return Ok(true);
    }

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
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<bool, Error> {
        let mut spender: AccountId = Self::env().caller();
        self._spend_allowance(from, spender, amount)?;
        self._transfer(from, to, amount)?;
        return Ok(true);
    }

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
    fn increase_allowance(&mut self, spender: AccountId, added_value: u128) -> Result<bool, Error> {
        let mut owner: AccountId = Self::env().caller();
        self._approve(
            owner,
            spender,
            self.allowance(owner, spender)? + added_value,
        )?;
        return Ok(true);
    }

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
    fn decrease_allowance(
        &mut self,
        spender: AccountId,
        subtracted_value: u128,
    ) -> Result<bool, Error> {
        let mut owner: AccountId = Self::env().caller();
        let mut current_allowance: u128 = self.allowance(owner, spender)?;
        if !(current_allowance >= subtracted_value) {
            return Err(Error::Custom(String::from(
                "ERC20: decreased allowance below zero",
            )));
        };
        self._approve(owner, spender, current_allowance - subtracted_value)?;
        return Ok(true);
    }
}

pub trait Internal {
    /// @dev Moves `amount` of tokens from `from` to `to`.
    ///
    /// This internal function is equivalent to {transfer}, and can be used to
    /// e.g. implement automatic token fees, slashing mechanisms, etc.
    ///
    /// Emits a {Transfer} event.
    ///
    /// Requirements:
    ///
    /// - `from` cannot be the zero address.
    /// - `to` cannot be the zero address.
    /// - `from` must have a balance of at least `amount`.
    fn _transfer(&mut self, from: AccountId, to: AccountId, amount: u128) -> Result<(), Error>;

    /// Overflow not possible: the sum of all balances is capped by totalSupply, and the sum is preserved by
    /// decrementing then incrementing.
    ///dev Creates `amount` tokens and assigns them to `account`, increasing
    /// the total supply.
    ///
    /// Emits a {Transfer} event with `from` set to the zero address.
    ///
    /// Requirements:
    ///
    /// - `account` cannot be the zero address.
    fn _mint(&mut self, account: AccountId, amount: u128) -> Result<(), Error>;

    /// Overflow not possible: balance + amount is at most totalSupply + amount, which is checked above.
    /// @dev Destroys `amount` tokens from `account`, reducing the
    /// total supply.
    ///
    /// Emits a {Transfer} event with `to` set to the zero address.
    ///
    /// Requirements:
    ///
    /// - `account` cannot be the zero address.
    /// - `account` must have at least `amount` tokens.
    fn _burn(&mut self, account: AccountId, amount: u128) -> Result<(), Error>;

    /// Overflow not possible: amount <= accountBalance <= totalSupply.
    /// @dev Sets `amount` as the allowance of `spender` over the `owner` s tokens.
    ///
    /// This internal function is equivalent to `approve`, and can be used to
    /// e.g. set automatic allowances for certain subsystems, etc.
    ///
    /// Emits an {Approval} event.
    ///
    /// Requirements:
    ///
    /// - `owner` cannot be the zero address.
    /// - `spender` cannot be the zero address.
    fn _approve(&mut self, owner: AccountId, spender: AccountId, amount: u128)
        -> Result<(), Error>;

    /// @dev Updates `owner` s allowance for `spender` based on spent `amount`.
    ///
    /// Does not update the allowance amount in case of infinite allowance.
    /// Revert if not enough allowance is available.
    ///
    /// Might emit an {Approval} event.
    fn _spend_allowance(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: u128,
    ) -> Result<(), Error>;

    /// @dev Hook that is called before any transfer of tokens. This includes
    /// minting and burning.
    ///
    /// Calling conditions:
    ///
    /// - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
    /// will be transferred to `to`.
    /// - when `from` is zero, `amount` tokens will be minted for `to`.
    /// - when `to` is zero, `amount` of ``from``'s tokens will be burned.
    /// - `from` and `to` are never both zero.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    fn _before_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error>;

    /// @dev Hook that is called after any transfer of tokens. This includes
    /// minting and burning.
    ///
    /// Calling conditions:
    ///
    /// - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
    /// has been transferred to `to`.
    /// - when `from` is zero, `amount` tokens have been minted for `to`.
    /// - when `to` is zero, `amount` of ``from``'s tokens have been burned.
    /// - `from` and `to` are never both zero.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    fn _after_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error>;

    fn _emit_transfer(&self, from: AccountId, to: AccountId, value: u128);

    fn _emit_approval(&self, owner: AccountId, spender: AccountId, value: u128);
}

impl<T: Storage<Data>> Internal for T {
    /// @dev Moves `amount` of tokens from `from` to `to`.
    ///
    /// This internal function is equivalent to {transfer}, and can be used to
    /// e.g. implement automatic token fees, slashing mechanisms, etc.
    ///
    /// Emits a {Transfer} event.
    ///
    /// Requirements:
    ///
    /// - `from` cannot be the zero address.
    /// - `to` cannot be the zero address.
    /// - `from` must have a balance of at least `amount`.
    default fn _transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        if !(from != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC20: transfer from the zero address",
            )));
        };
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC20: transfer to the zero address",
            )));
        };
        self._before_token_transfer(from, to, amount)?;
        let mut from_balance: u128 = self.data().balances.get(&from).unwrap_or_default();
        if !(from_balance >= amount) {
            return Err(Error::Custom(String::from(
                "ERC20: transfer amount exceeds balance",
            )));
        };
        self.data()
            .balances
            .insert(&(from), &(from_balance - amount));
        let new_value = self.data().balances.get(&(to)).unwrap_or_default() + amount;
        self.data().balances.insert(&(to), &new_value);
        self._emit_transfer(from, to, amount);
        self._after_token_transfer(from, to, amount)?;
        Ok(())
    }

    /// Overflow not possible: the sum of all balances is capped by totalSupply, and the sum is preserved by
    /// decrementing then incrementing.
    ///dev Creates `amount` tokens and assigns them to `account`, increasing
    /// the total supply.
    ///
    /// Emits a {Transfer} event with `from` set to the zero address.
    ///
    /// Requirements:
    ///
    /// - `account` cannot be the zero address.
    default fn _mint(&mut self, account: AccountId, amount: u128) -> Result<(), Error> {
        if !(account != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC20: mint to the zero address",
            )));
        };
        self._before_token_transfer(ZERO_ADDRESS.into(), account, amount)?;
        self.data().total_supply += amount;
        let new_value = self.data().balances.get(&(account)).unwrap_or_default() + amount;
        self.data().balances.insert(&(account), &new_value);
        self._emit_transfer(ZERO_ADDRESS.into(), account, amount);
        self._after_token_transfer(ZERO_ADDRESS.into(), account, amount)?;
        Ok(())
    }

    /// Overflow not possible: balance + amount is at most totalSupply + amount, which is checked above.
    /// @dev Destroys `amount` tokens from `account`, reducing the
    /// total supply.
    ///
    /// Emits a {Transfer} event with `to` set to the zero address.
    ///
    /// Requirements:
    ///
    /// - `account` cannot be the zero address.
    /// - `account` must have at least `amount` tokens.
    default fn _burn(&mut self, account: AccountId, amount: u128) -> Result<(), Error> {
        if !(account != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC20: burn from the zero address",
            )));
        };
        self._before_token_transfer(account, ZERO_ADDRESS.into(), amount)?;
        let mut account_balance: u128 = self.data().balances.get(&account).unwrap_or_default();
        if !(account_balance >= amount) {
            return Err(Error::Custom(String::from(
                "ERC20: burn amount exceeds balance",
            )));
        };
        self.data()
            .balances
            .insert(&(account), &(account_balance - amount));
        self.data().total_supply -= amount;
        self._emit_transfer(account, ZERO_ADDRESS.into(), amount);
        self._after_token_transfer(account, ZERO_ADDRESS.into(), amount)?;
        Ok(())
    }

    /// Overflow not possible: amount <= accountBalance <= totalSupply.
    /// @dev Sets `amount` as the allowance of `spender` over the `owner` s tokens.
    ///
    /// This internal function is equivalent to `approve`, and can be used to
    /// e.g. set automatic allowances for certain subsystems, etc.
    ///
    /// Emits an {Approval} event.
    ///
    /// Requirements:
    ///
    /// - `owner` cannot be the zero address.
    /// - `spender` cannot be the zero address.
    default fn _approve(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        if !(owner != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC20: approve from the zero address",
            )));
        };
        if !(spender != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC20: approve to the zero address",
            )));
        };
        self.data().allowances.insert(&(owner, spender), &(amount));
        self._emit_approval(owner, spender, amount);
        Ok(())
    }

    /// @dev Updates `owner` s allowance for `spender` based on spent `amount`.
    ///
    /// Does not update the allowance amount in case of infinite allowance.
    /// Revert if not enough allowance is available.
    ///
    /// Might emit an {Approval} event.
    default fn _spend_allowance(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        let mut current_allowance: u128 = self.allowance(owner, spender)?;
        if current_allowance != u128::MAX {
            if !(current_allowance >= amount) {
                return Err(Error::Custom(String::from("ERC20: insufficient allowance")));
            };
            self._approve(owner, spender, current_allowance - amount)?;
        }
        Ok(())
    }

    /// @dev Hook that is called before any transfer of tokens. This includes
    /// minting and burning.
    ///
    /// Calling conditions:
    ///
    /// - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
    /// will be transferred to `to`.
    /// - when `from` is zero, `amount` tokens will be minted for `to`.
    /// - when `to` is zero, `amount` of ``from``'s tokens will be burned.
    /// - `from` and `to` are never both zero.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    default fn _before_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        Ok(())
    }

    /// @dev Hook that is called after any transfer of tokens. This includes
    /// minting and burning.
    ///
    /// Calling conditions:
    ///
    /// - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
    /// has been transferred to `to`.
    /// - when `from` is zero, `amount` tokens have been minted for `to`.
    /// - when `to` is zero, `amount` of ``from``'s tokens have been burned.
    /// - `from` and `to` are never both zero.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    default fn _after_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        Ok(())
    }

    default fn _emit_transfer(&self, _: AccountId, _: AccountId, _: u128) {}

    default fn _emit_approval(&self, _: AccountId, _: AccountId, _: u128) {}
}
