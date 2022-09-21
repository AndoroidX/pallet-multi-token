use super::*;
use frame_support::ensure;
use frame_support::pallet_prelude::DispatchResult;
use sp_runtime::traits::{Saturating, Zero};

impl<T: Config> Mintable<T> for Pallet<T> {
    fn create_token(admin: T::AccountId) -> DispatchResult {
        let id = match AssetIdNonce::<T>::get() {
            Some(mut nonce) => {
                nonce.saturating_inc();
                AssetIdNonce::<T>::put(nonce);
                nonce
            }
            None => {
                let zero: T::AssetId = Zero::zero();
                AssetIdNonce::<T>::put(zero);
                zero
            }
        };
        Assets::<T>::insert(&id, &admin);
        Self::deposit_event(Event::<T>::Created { admin, id });
        Ok(())
    }

    fn mint_tokens(minter: T::AccountId, id: T::AssetId, amount: T::Balance) -> DispatchResult {
        let admin = Assets::<T>::get(&id).ok_or(Error::<T>::UndefinedAsset)?;
        ensure!(admin == minter, Error::<T>::NoPermission);
        Self::credit(&minter, &id, amount)?;
        Self::deposit_event(Event::<T>::Minted {
            origin: minter,
            id,
            amount,
        });
        Ok(())
    }
}

pub trait Mintable<T: Config> {
    fn create_token(admin: T::AccountId) -> DispatchResult;
    fn mint_tokens(minter: T::AccountId, id: T::AssetId, amount: T::Balance) -> DispatchResult;
}
