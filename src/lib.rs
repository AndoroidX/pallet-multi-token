#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::traits::StaticLookup;
mod functions;
mod multi_token;
mod mintable;
#[cfg(test)]
mod test;
type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::dispatch::HasCompact;
	use sp_runtime::traits::AtLeast32BitUnsigned;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Balance: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo;

		type AssetId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo;
	}

	#[pallet::storage]
	pub type Assets<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		T::AccountId, // Admin
	>;

	#[pallet::storage]
	pub type Accounts<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Blake2_128Concat,
		T::AccountId,
		T::Balance,
	>;

	#[pallet::storage]
	pub type Approvals<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId, // owner
		Blake2_128Concat,
		T::AccountId, // operator
		bool, // is approved for all
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub fn deposit_event)]
	pub enum Event<T: Config> {
		Transferred{origin: T::AccountId, from: T::AccountId, to: T::AccountId, id: T::AssetId, amount: T::Balance},
		Created{admin: T::AccountId, id: T::AssetId},
		Minted{origin: T::AccountId, to: T::AccountId, id: T::AssetId, amount: T::Balance},
		Burned{operator: T::AccountId, from: T::AccountId, id: T::AssetId, amount: T::Balance},
		ApprovedAll{origin: T::AccountId, operator: T::AccountId, is_approved: bool}
	}

	#[pallet::error]
	pub enum Error<T> {
		NoPermission,
		NotApproved,
		NotEnoughBalance,
		UndefinedAccount,
		Overflow
	}

	// transfer
	// approve_all
	// create
	// mint
	// burn
	// set_admin

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// r[1] w[]
		#[pallet::weight(10_000_000)]
		pub fn transfer(
			origin: OriginFor<T>, 
			from: AccountIdLookupOf<T>, 
			to: AccountIdLookupOf<T>, 
			id: T::AssetId, 
			amount: T::Balance
		) -> DispatchResult {
			let operator = ensure_signed(origin)?;
			let from = T::Lookup::lookup(from)?;
			let to = T::Lookup::lookup(to)?;

			unimplemented!()
		}
	}
}



	