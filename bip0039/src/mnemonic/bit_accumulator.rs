//! MSB-first bit accumulator utility.
//!
//! This module provides a tiny helper for streaming fixed-width values in MSB-first order,
//! which is used by BIP-0039 encoding/decoding.
//!
//! Convention:
//! - Bits are shifted in MSB-first (the earliest bit ends up at higher positions while
//!   accumulating).
//! - When draining, we always take the highest available bits first to preserve MSB-first order.

/// A tiny MSB-first bit accumulator for streaming fixed-width values.
///
/// BIP-0039 streams:
/// - entropy bits (bytes, MSB-first)
/// - checksum bits (MSB-first)
/// - word indices (11-bit, MSB-first)
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct BitAccumulator {
    acc: u64,
    bits: usize,
}

impl BitAccumulator {
    /// Create a new empty accumulator.
    #[inline]
    pub const fn new() -> Self {
        Self { acc: 0, bits: 0 }
    }

    /// Push the lowest `n` bits of `v` into the accumulator, MSB-first within that `n`-bit value.
    ///
    /// # Panics (debug)
    /// In debug builds, this asserts `n <= 56` to keep the implementation simple and ensure
    /// shifting never overflows `u64` for the intended BIP-0039 use cases.
    #[inline]
    pub const fn push_bits(&mut self, v: u64, n: usize) {
        debug_assert!(n <= 56, "push_bits: too many bits");

        // BIP-0039 only uses small `n` (8 for entropy bytes; 4..=8 for checksum bits).
        // Since `n <= 56` here, we never need to handle the `n == 64` special case.
        let mask = (1u64 << n) - 1;
        self.acc = (self.acc << n) | (v & mask);
        self.bits += n;
    }

    /// Drain the next `n` bits (MSB-first) as a value in `0..(1<<n)`.
    #[inline]
    pub const fn take_bits(&mut self, n: usize) -> u64 {
        debug_assert!(n <= self.bits, "take_bits: not enough bits");

        // Earliest bits are stored at higher positions; drain from the top.
        let shift = self.bits - n;
        let v = (self.acc >> shift) & ((1u64 << n) - 1);

        // Keep only the remaining low `shift` bits and update the buffered bit count.
        // This avoids an extra branch on the common hot path.
        self.acc &= (1u64 << shift) - 1;
        self.bits = shift;

        v
    }

    /// Whether at least `n` bits can be drained.
    #[inline]
    pub const fn can_take(&self, n: usize) -> bool {
        self.bits >= n
    }

    /// Current number of bits buffered.
    #[inline]
    pub const fn bits(&self) -> usize {
        self.bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Keep a lightweight sanity test that our MSB-first bit conventions behave as expected.
    #[test]
    fn test_bitacc_msb_first_roundtrip_bytes() {
        let mut acc = BitAccumulator::new();
        acc.push_bits(0b1111_0111, 8);
        assert_eq!(acc.take_bits(1), 1);
        assert_eq!(acc.take_bits(1), 1);
        assert_eq!(acc.take_bits(1), 1);
        assert_eq!(acc.take_bits(1), 1);
        assert_eq!(acc.take_bits(1), 0);
        assert_eq!(acc.take_bits(3), 0b111);
        assert_eq!(acc.bits(), 0);
    }
}
