//! SLIP-10 curve marker traits.

use super::Curve;

/// Marker trait for SLIP-10 compatible curves.
pub trait Slip10Curve: Curve {}

/// Marker trait for SLIP-10 curves that only allow hardened derivation.
pub trait Slip10HardenedOnlyCurve: Slip10Curve {}

/// Marker trait for SLIP-10 curves that allow non-hardened derivation.
pub trait Slip10NonHardenedCurve: Slip10Curve {}
