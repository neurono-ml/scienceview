//! A deterministic generator, implemented in-crate.
//!
//! Jitter and the swarm layout must reproduce the same displacement on every
//! platform and across runs. The core takes no dependency for that, so the
//! generator below is plain 64-bit integer arithmetic — SplitMix64 — and the
//! only floating-point operations it feeds are addition, subtraction,
//! multiplication and division, each of which IEEE 754 specifies exactly.
//!
//! The constants are the reference SplitMix64 ones.

/// The golden-ratio increment SplitMix64 adds on every step.
const SV_SPLIT_MIX_INCREMENT: u64 = 0x9E37_79B9_7F4A_7C15;
/// The first mixing constant of the finalizer.
const SV_SPLIT_MIX_FIRST: u64 = 0xBF58_476D_1CE4_E5B9;
/// The second mixing constant of the finalizer.
const SV_SPLIT_MIX_SECOND: u64 = 0x94D0_49BB_1331_11EB;

/// Advances one SplitMix64 state and returns the mixed word.
///
/// The state is advanced by the golden-ratio increment and the result is the
/// SplitMix64 finalizer, so two calls with the same state always agree.
pub fn sv_split_mix_64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(SV_SPLIT_MIX_INCREMENT);
    let mut word = *state;
    word = (word ^ (word >> 30)).wrapping_mul(SV_SPLIT_MIX_FIRST);
    word = (word ^ (word >> 27)).wrapping_mul(SV_SPLIT_MIX_SECOND);
    word ^ (word >> 31)
}
