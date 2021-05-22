//! Various types for private sale pallet
use super::*;

pub type BalanceReleasePercentage = u8;
pub type BalanceReleaseDuration = u64;

// Balance Release
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct BalanceReleaseSetting<Balance> {
    // List of percentages that is released for each stage
    pub(super) release_percentages: Vec<BalanceReleasePercentage>,
    // List of durations in miliseconds required release for each stage
    pub(super) release_durations: Vec<BalanceReleaseDuration>,
    // Minimum balance to transfer
    pub(super) min_transfer_balance: Option<Balance>,
    // Maximum balance to transfer
    pub(super) max_transfer_balance: Option<Balance>,
}
