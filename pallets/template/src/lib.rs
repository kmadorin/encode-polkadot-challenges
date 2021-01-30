#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use codec::{Encode, Decode};
use frame_support::{decl_module, decl_storage, decl_event, decl_error, StorageValue, StorageDoubleMap,
					traits::Randomness, RuntimeDebug, dispatch::DispatchError};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Zombie(pub [u8; 16]);

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq)]
pub enum ZombieGender {
	Male,
	Female,
}

impl Zombie{
	pub fn gender(&self) -> ZombieGender {
		if self.0[0] % 2 == 0 {
			ZombieGender::Male
		} else {
			ZombieGender::Female
		}
	}
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Randomness: Randomness<Self::Hash>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as Zombies {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		pub Zombies get(fn zombies): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<Zombie>;
		/// Stores the next kitty ID
		pub NextZombieId get(fn next_zombie_id): u32;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		/// A zombie is created. \[owner, zombie_id, zombie\]
		ZombieCreated(AccountId, u32, Zombie),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		ZombiesIdOverflow,
		InvalidZombieId,
		SameGender,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 1000]
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;

			let zombie_id = Self::get_next_zombie_id()?;

			let dna = Self::random_value(&sender);

			let zombie = Zombie(dna);

			Zombies::<T>::insert(&sender, zombie_id, zombie.clone());

			Self::deposit_event(RawEvent::ZombieCreated(sender, zombie_id, zombie));

		}

	}
}

impl<T: Trait> Module<T> {
	fn get_next_zombie_id() -> sp_std::result::Result<u32, DispatchError> {
		NextZombieId::try_mutate(|next_id| -> sp_std::result::Result<u32, DispatchError> {
			let current_id = *next_id;
			*next_id = next_id.checked_add(1).ok_or(Error::<T>::ZombiesIdOverflow)?;
			Ok(current_id)
		})
	}

	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		// TODO: finish this implementation
		let payload = (
			T::Randomness::random_seed(),
			&sender,
			<frame_system::Module<T>>::extrinsic_index(),
		);
		payload.using_encoded(blake2_128)
	}
}
