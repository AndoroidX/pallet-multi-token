use frame_support::ensure;
use sp_runtime::{DispatchResult};
use sp_runtime::traits::{CheckedAdd, CheckedSub};
use super::*;

impl<T: Config> Pallet<T> {
    pub(crate) fn debit(target: &T::AccountId, id: &T::AssetId, value: T::Balance) -> DispatchResult {
        let mut account = Accounts::<T>::get(&id, &target).ok_or(Error::<T>::UndefinedAccount)?;
        ensure!(account >= value, Error::<T>::NotEnoughBalance);
        account = account.checked_sub(&value).ok_or(Error::<T>::Overflow)?;
        Ok(())
    }

    pub(crate) fn credit(target: &T::AccountId, id: &T::AssetId, value: T::Balance) -> DispatchResult {
        match Accounts::<T>::get(&id, &target) {
            Some(mut account) => {
                account = account.checked_add(&value).ok_or(Error::<T>::Overflow)?;
            },
            None => {
                Accounts::<T>::insert(&id, &target, value);
            }
        }
        Ok(())
    }
}