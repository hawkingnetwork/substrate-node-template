#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod types;
pub use types::*;

use frame_support::codec::{Decode, Encode};
use frame_support::traits::{Currency, EnsureOrigin, LockableCurrency};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::Codec;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::AtLeast32BitUnsigned;
    use sp_std::fmt::Debug;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        // Currency mechanism
        type Currency: Currency<Self::AccountId> + LockableCurrency<Self::AccountId>;

        // Identifier for each unlock setting
        type RoundId: Member + Parameter + Copy;

        // The origin which may call force operations
        type ForceOrigin: EnsureOrigin<Self::Origin>;

        // Balance type
        type Balance: Member
            + Parameter
            + AtLeast32BitUnsigned
            + Codec
            + Copy
            + MaybeSerializeDeserialize
            + Debug;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn something)]
    // Learn more about declaring storage items:
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
    pub type Something<T> = StorageValue<_, u32>;

    // Percentage Unlock Settings
    // Store Vector of Percentages to unlock at stage for each setting
    #[pallet::storage]
    pub type BalanceReleaseSettingsMap<T: Config> =
        StorageMap<_, Blake2_128Concat, T::RoundId, BalanceReleaseSetting<T::Balance>>;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, T::AccountId),
        // Emitted when successfully create a private sale round. [round_id]
        RoundCreated(T::RoundId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        // Total balance release percentage is not equal to 1
        InvalidTotalBalanceReleasePercentage,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let who = ensure_signed(origin)?;

            // Update storage.
            <Something<T>>::put(something);

            // Emit an event.
            Self::deposit_event(Event::SomethingStored(something, who));
            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }

        /// An example dispatchable that may throw a custom error.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;

            // Read a value from storage.
            match <Something<T>>::get() {
                // Return an error if the value has not been set.
                None => Err(Error::<T>::NoneValue)?,
                Some(old) => {
                    // Increment the value read from storage; will error in the event of overflow.
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    // Update the value in storage with the incremented result.
                    <Something<T>>::put(new);
                    Ok(().into())
                }
            }
        }

        // Create Unlock Setting Item
        //
        // This call is for Root user only
        #[pallet::weight(10_000)]
        pub fn force_create_round(
            origin: OriginFor<T>,
            round_id: T::RoundId,
            balance_release_percentages: Vec<BalanceReleasePercentage>,
            balance_release_durations: Vec<BalanceReleaseDuration>,
            min_transfer_balance: Option<T::Balance>,
            max_transfer_balance: Option<T::Balance>,
        ) -> DispatchResultWithPostInfo {
            T::ForceOrigin::ensure_origin(origin)?;

            let total_percentage = balance_release_percentages.iter().fold(0, |acc, x| acc + x);

            ensure!(
                total_percentage != 100,
                Error::<T>::InvalidTotalBalanceReleasePercentage
            );

            let balance_release_setting = BalanceReleaseSetting {
                release_percentages: balance_release_percentages,
                release_durations: balance_release_durations,
                min_transfer_balance: min_transfer_balance,
                max_transfer_balance: max_transfer_balance,
            };
            <BalanceReleaseSettingsMap<T>>::insert(round_id, balance_release_setting);

            Self::deposit_event(Event::RoundCreated(round_id));

            Ok(().into())
        }
    }
}
