pub use crate::traits::staking::*;
use openbrush::storage::Mapping;
use openbrush::traits::AccountId;
use openbrush::traits::Balance;
use openbrush::traits::Storage;
use openbrush::traits::Timestamp;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(StakingData);

#[openbrush::upgradeable_storage(STORAGE_KEY)]
#[derive(Default)]
pub struct StakingData {
    staked_amounts: Mapping<AccountId, Balance>,
    staked_timestamp: Mapping<AccountId, Timestamp>,
}

impl<T> Staking for T
where
    T: Storage<StakingData>,
{
    fn stake(&mut self, amount: Balance) {}

    fn unstake(&mut self, amount: Balance) {}
}
