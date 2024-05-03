#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod tests;
pub mod types;

use frame_support::{ensure, sp_runtime::traits::Zero};
use types::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + scale_info::TypeInfo {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn unique_asset)]
	pub(super) type UniqueAsset<T: Config> =
		StorageMap<_, Blake2_128Concat, UniqueAssetId, UniqueAssetDetails<T, T::MaxLength>>;

	#[pallet::storage]
	#[pallet::getter(fn account)]
	/// The holdings of a specific account for a specific asset.
	pub(super) type Account<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		UniqueAssetId,
		Blake2_128Concat,
		T::AccountId,
		u128,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	/// Nonce for id of the next created asset
	pub(super) type Nonce<T: Config> = StorageValue<_, UniqueAssetId, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New unique asset created
		Created {
			creator: T::AccountId,
			asset_id: UniqueAssetId,
		},
		/// Some assets have been burned
		Burned {
			asset_id: UniqueAssetId,
			owner: T::AccountId,
			total_supply: u128,
		},
		/// Some assets have been transferred
		Transferred {
			asset_id: UniqueAssetId,
			from: T::AccountId,
			to: T::AccountId,
			amount: u128,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The asset ID is unknown
		UnknownAssetId,
		/// The signing account does not own any amount of this asset
		NotOwned,
		/// Supply must be positive
		NoSupply,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::default())]
		pub fn mint(
			origin: OriginFor<T>,
			metadata: BoundedVec<u8, T::MaxLength>,
			supply: u128,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			ensure!(!supply.is_zero(), Error::<T>::NoSupply);

			let id = Self::nonce();

			let details =
				UniqueAssetDetails::<T, T::MaxLength>::new(origin.clone(), metadata, supply);

			UniqueAsset::<T>::insert(id, details);

			Account::<T>::insert(id, origin.clone(), supply);

			Nonce::<T>::set(id.saturating_add(1));

			Self::deposit_event(Event::Created {
				creator: origin,
				asset_id: id,
			});

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::default())]
		pub fn burn(origin: OriginFor<T>, asset_id: UniqueAssetId, amount: u128) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			let mut total_supply = 0;

			UniqueAsset::<T>::try_mutate(asset_id, |details| -> DispatchResult {
				let details = details.as_mut().ok_or(Error::<T>::UnknownAssetId)?;

				Self::ensure_own_some(asset_id, origin.clone())?;

				let mut burned_amount = 0;

				Account::<T>::mutate(asset_id, origin.clone(), |owned_amount| {
					let old_amount = *owned_amount;
					*owned_amount = owned_amount.saturating_sub(amount);
					burned_amount = old_amount - *owned_amount;
				});

				details.supply -= burned_amount;

				total_supply = details.supply;

				Ok(())
			})?;

			Self::deposit_event(Event::Burned {
				asset_id,
				owner: origin,
				total_supply,
			});

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(Weight::default())]
		pub fn transfer(
			origin: OriginFor<T>,
			asset_id: UniqueAssetId,
			amount: u128,
			to: T::AccountId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			ensure!(
				Self::unique_asset(asset_id).is_some(),
				Error::<T>::UnknownAssetId
			);

			Self::ensure_own_some(asset_id, origin.clone())?;

			let mut transferred_amount = 0;

			Account::<T>::mutate(asset_id, origin.clone(), |owned_amount| {
				let old_amount = *owned_amount;
				*owned_amount = owned_amount.saturating_sub(amount);
				transferred_amount = old_amount - *owned_amount;
			});

			Account::<T>::mutate(asset_id, to.clone(), |owned_amount| {
				*owned_amount = owned_amount.saturating_add(transferred_amount);
			});

			Self::deposit_event(Event::Transferred {
				asset_id,
				from: origin,
				to,
				amount: transferred_amount,
			});

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn ensure_own_some(asset_id: UniqueAssetId, account: T::AccountId) -> Result<(), Error<T>> {
		ensure!(
			!Self::account(asset_id, account).is_zero(),
			Error::<T>::NotOwned
		);
		Ok(())
	}
}
