//! SLIP-0010 curve marker traits.

use super::Curve;

/// Marker trait for SLIP-0010 curves.
pub trait Slip10Curve: Curve {}

/// Marker trait for SLIP-0010 curves that only allow hardened derivation.
pub trait Slip10HardenedOnlyCurve: Slip10Curve {}

/// Marker trait for SLIP-0010 curves that allow non-hardened derivation.
pub trait Slip10NonHardenedCurve: Slip10Curve {}
