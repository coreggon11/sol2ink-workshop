#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod contract {
    use openbrush::contracts::psp22::*;
    use openbrush::traits::Storage;
    use workshop::impls::staking::*;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Storage, Default)]
    pub struct Contract {
        #[storage_field]
        data: psp22::Data,
        #[storage_field]
        staking_data: StakingData,
    }

    impl PSP22 for Contract {}

    impl Staking for Contract {}

    impl Contract {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            instance
                ._mint_to(Self::env().caller(), total_supply)
                .expect("Unable to mint tokens to caller");
            instance
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use openbrush::test_utils::{accounts, change_caller};

        #[ink::test]
        fn new_works() {
            let total_supply = 1_000_000;
            let accounts = accounts();

            let contract = Contract::new(total_supply);

            assert_eq!(contract.total_supply(), total_supply);
            assert_eq!(contract.balance_of(accounts.alice), total_supply);
        }

        #[ink::test]
        fn can_not_transfer_without_allowance() {
            let transfer_amount = 10_000;
            let total_supply = 10_000_000;
            let accounts = accounts();

            let mut contract = Contract::new(total_supply);

            change_caller(accounts.bob);

            assert_eq!(contract.balance_of(accounts.alice), total_supply);

            let tx = contract.transfer_from(accounts.alice, accounts.bob, transfer_amount, vec![]);
            assert!(tx.is_err());
        }
    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = ContractRef::default();

            // When
            let contract_account_id = client
                .instantiate("contract", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<ContractRef>(contract_account_id.clone())
                .call(|contract| contract.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works_2(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = ContractRef::new(false);
            let contract_account_id = client
                .instantiate("contract", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<ContractRef>(contract_account_id.clone())
                .call(|contract| contract.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<ContractRef>(contract_account_id.clone())
                .call(|contract| contract.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<ContractRef>(contract_account_id.clone())
                .call(|contract| contract.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
