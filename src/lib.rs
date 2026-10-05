#![no_std]

use bobcat_maths::{U, wrapping_mul_const};

const THOUSAND: U = U::from_u16(1_000);

/// No ether (`0 wei`).
pub const NOETHER: U = U::ZERO;
/// Wei, the smallest EVM native-currency denomination (`10^0 wei`).
pub const WEI: U = U::ONE;
/// Kwei (`10^3 wei`).
pub const KWEI: U = wrapping_mul_const(&WEI, &THOUSAND);
/// Mwei (`10^6 wei`).
pub const MWEI: U = wrapping_mul_const(&KWEI, &THOUSAND);
/// Gwei (`10^9 wei`).
pub const GWEI: U = wrapping_mul_const(&MWEI, &THOUSAND);
/// Microether (`10^12 wei`).
pub const MICROETHER: U = wrapping_mul_const(&GWEI, &THOUSAND);
/// Milliether (`10^15 wei`).
pub const MILLIETHER: U = wrapping_mul_const(&MICROETHER, &THOUSAND);
/// Ether (`10^18 wei`).
pub const ETHER: U = wrapping_mul_const(&MILLIETHER, &THOUSAND);
/// Kether (`10^21 wei`).
pub const KETHER: U = wrapping_mul_const(&ETHER, &THOUSAND);
/// Mether (`10^24 wei`).
pub const METHER: U = wrapping_mul_const(&KETHER, &THOUSAND);
/// Gether (`10^27 wei`).
pub const GETHER: U = wrapping_mul_const(&METHER, &THOUSAND);
/// Tether (`10^30 wei`).
pub const TETHER: U = wrapping_mul_const(&GETHER, &THOUSAND);

/// Alias for [`KWEI`].
pub const BABBAGE: U = KWEI;
/// Alias for [`KWEI`].
pub const FEMTOETHER: U = KWEI;
/// Alias for [`MWEI`].
pub const LOVELACE: U = MWEI;
/// Alias for [`MWEI`].
pub const PICOETHER: U = MWEI;
/// Alias for [`GWEI`].
pub const SHANNON: U = GWEI;
/// Alias for [`GWEI`].
pub const NANOETHER: U = GWEI;
/// Alias for [`GWEI`].
pub const NANO: U = GWEI;
/// Alias for [`MICROETHER`].
pub const SZABO: U = MICROETHER;
/// Alias for [`MICROETHER`].
pub const MICRO: U = MICROETHER;
/// Alias for [`MILLIETHER`].
pub const FINNEY: U = MILLIETHER;
/// Alias for [`MILLIETHER`].
pub const MILLI: U = MILLIETHER;
/// Alias for [`KETHER`].
pub const GRAND: U = KETHER;
