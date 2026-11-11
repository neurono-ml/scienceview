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

/// Mixes the mark key — seed, group code and point index — into one word.
///
/// The key never contains the frame index, which is what keeps a jittered point
/// in place while its value changes (design D4).
pub(crate) fn sv_mix_mark_key(seed: Option<u64>, group_key: u32, index: usize) -> u64 {
    let mut state = seed.unwrap_or(0) ^ (u64::from(group_key) << 32) ^ (index as u64);
    sv_split_mix_64(&mut state)
}

/// Returns a uniform value in `0.0..1.0` from one 64-bit word.
///
/// The upper 53 bits are used, which is exactly the precision an `f64` mantissa
/// can represent, so the mapping is exact and platform-independent.
pub(crate) fn sv_unit_interval(word: u64) -> f64 {
    ((word >> 11) as f64) * (1.0 / 9_007_199_254_740_992.0)
}

/// Returns a uniform value in `-1.0..1.0` from one 64-bit word.
pub(crate) fn sv_signed_unit_interval(word: u64) -> f64 {
    sv_unit_interval(word).mul_add(2.0, -1.0)
}
