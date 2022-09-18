use frame_support::{ensure};
use frame_support::pallet_prelude::{DispatchResult};
use sp_runtime::traits::{Zero, AtLeast32BitUnsigned};
use frame_support::pallet_prelude::*;
use frame_support::dispatch::HasCompact;
use super::*;

impl<T: Config> MultiTokenTrait<T, T::AssetId, T::Balance> for Pallet<T> {
    fn safe_transfer(operator: T::AccountId, from: T::AccountId, to: T::AccountId, id: T::AssetId, value: T::Balance) -> DispatchResult {
        // Permission check
        if operator != from {
            let is_approved = match Approvals::<T>::get(&from, &operator) {
                Some(approved) => approved,
                None => false
            };
            ensure!(is_approved, Error::<T>::NotApproved);
        }
        if value.is_zero() {
            return Ok(());
        }
        if from == to {
            return Ok(());
        }
        Self::debit(&from, &id, value.clone())?;
        Self::credit(&to, &id, value.clone())?;
        Self::deposit_event(Event::<T>::Transferred {
            origin: operator,
            from,
            to,
            id,
            amount: value
        });
        return Ok(())
    }

    fn set_approve_all(owner: T::AccountId, operator: T::AccountId, approved: bool) -> DispatchResult {
        if owner == operator {
            return Ok(());
        }
        match Approvals::<T>::get(&owner, &operator) {
            Some(is_approved) => {
                if is_approved == approved {
                    return Ok(());
                }
                Approvals::<T>::set(&owner, &operator, Some(approved));
            },
            None => {
                Approvals::<T>::insert(&owner, &operator, approved);
            }
        }
        Self::deposit_event(Event::<T>::ApprovedAll { origin: owner, operator: operator, is_approved: approved });
        Ok(())
    }
}

pub trait MultiTokenTrait<T, AssetId, Balance> 
where 
    T: frame_system::Config,
    Balance: Member
        + Parameter
        + AtLeast32BitUnsigned
        + Default
        + Copy
        + MaybeSerializeDeserialize
        + MaxEncodedLen
        + TypeInfo,
    AssetId: Member
        + Parameter
        + AtLeast32BitUnsigned
        + Default
        + Copy
        + HasCompact
        + MaybeSerializeDeserialize
        + MaxEncodedLen
        + TypeInfo
        + Zero
    {
    fn safe_transfer(operator: T::AccountId, from: T::AccountId, to: T::AccountId, id: AssetId, value: Balance) -> DispatchResult;
    fn set_approve_all(owner: T::AccountId, operator: T::AccountId, approved: bool) -> DispatchResult;
}