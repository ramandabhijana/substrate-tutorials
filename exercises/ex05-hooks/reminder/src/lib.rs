#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn event_counter)]
	pub type EventCounter<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn reminders)]
	pub type Reminders<T: Config> =
		StorageMap<_, Blake2_256, BlockNumberFor<T>, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ReminderSet(BlockNumberFor<T>, Vec<u8>),
		Reminder(Vec<u8>),
		RemindersExecuteds(u32),
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// on_initialize() will be called at the beginning of each new block, before anything
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			let mut used_weight: Weight = 0.into();

			let reminders = Self::reminders(n);

			// this is an example of how do we get system weights for read and writes.
			// you only have to mesure read and writes for this exercice !
			//
			// try to do this hook in one read and two writes !
			used_weight += T::DbWeight::get().reads(1);

			let event_count = reminders.len() as u32;

			EventCounter::<T>::mutate(|value| *value = event_count);
			used_weight += T::DbWeight::get().writes(1);

			for reminder in reminders {
				Self::deposit_event(Event::Reminder(reminder));
			}

			Reminders::<T>::remove(n);
			used_weight += T::DbWeight::get().writes(1);

			used_weight
		}

		fn on_finalize(n: BlockNumberFor<T>) {
			let count = Self::event_counter();
			Self::deposit_event(Event::RemindersExecuteds(count));
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(Weight::from(10_000) + T::DbWeight::get().reads(1))]
		pub fn schedule_reminder(
			origin: OriginFor<T>,
			at: BlockNumberFor<T>,
			message: Vec<u8>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			<Reminders<T>>::mutate(at, |reminders| reminders.push(message.clone()));
			Self::deposit_event(Event::ReminderSet(at, message));

			Ok(())
		}
	}
}
