use super::*;
use frame_support::ensure;
use sp_runtime::traits::{CheckedAdd, CheckedSub, Zero};
use sp_runtime::DispatchResult;

impl<T: Config> Pallet<T> {
    pub(crate) fn debit(
        target: &T::AccountId,
        id: &T::AssetId,
        value: T::Balance,
    ) -> DispatchResult {
        let account = Accounts::<T>::get(&id, &target).ok_or(Error::<T>::UndefinedAccount)?;
        ensure!(account >= value, Error::<T>::NotEnoughBalance);
        let new_amount = account.checked_sub(&value).ok_or(Error::<T>::Overflow)?;
        if new_amount.is_zero() {
            Accounts::<T>::remove(&id, &target);
        } else {
            Accounts::<T>::set(&id, &target, Some(new_amount));
        }
        Ok(())
    }

    pub(crate) fn credit(
        target: &T::AccountId,
        id: &T::AssetId,
        value: T::Balance,
    ) -> DispatchResult {
        match Accounts::<T>::get(&id, &target) {
            Some(account) => {
                let new_amount = account.checked_add(&value).ok_or(Error::<T>::Overflow)?;
                Accounts::<T>::set(&id, &target, Some(new_amount));
            }
            None => {
                Accounts::<T>::insert(&id, &target, value);
            }
        }
        Ok(())
    }
}
