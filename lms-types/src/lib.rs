// Licensed under the Apache-2.0 license

#![cfg_attr(all(not(test), not(fuzzing)), no_std)]

use core::mem::size_of;

use zerocopy::{AsBytes, BigEndian, FromBytes, LittleEndian, U32};
use zeroize::Zeroize;

pub type LmsIdentifier = [u8; 16];

macro_rules! static_assert {
    ($expression:expr) => {
        const _: () = assert!($expression);
    };
}

#[repr(transparent)]
#[derive(AsBytes, FromBytes, Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct LmsAlgorithmType(pub U32<BigEndian>);
impl LmsAlgorithmType {
    #![allow(non_upper_case_globals)]

    pub const fn new(val: u32) -> Self {
        Self(U32::from_bytes(val.to_be_bytes()))
    }
    pub const LmsReserved: Self = Self::new(0);
    pub const LmsSha256N32H5: Self = Self::new(5);
    pub const LmsSha256N32H10: Self = Self::new(6);
    pub const LmsSha256N32H15: Self = Self::new(7);
    pub const LmsSha256N32H20: Self = Self::new(8);
    pub const LmsSha256N32H25: Self = Self::new(9);
    pub const LmsSha256N24H5: Self = Self::new(10);
    pub const LmsSha256N24H10: Self = Self::new(11);
    pub const LmsSha256N24H15: Self = Self::new(12);
    pub const LmsSha256N24H20: Self = Self::new(13);
    pub const LmsSha256N24H25: Self = Self::new(14);
}

#[repr(transparent)]
#[derive(AsBytes, FromBytes, Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct LmotsAlgorithmType(pub U32<BigEndian>);

impl LmotsAlgorithmType {
    #![allow(non_upper_case_globals)]

    pub const fn new(val: u32) -> Self {
        Self(U32::from_bytes(val.to_be_bytes()))
    }
    pub const LmotsReserved: Self = Self::new(0);
    pub const LmotsSha256N32W1: Self = Self::new(1);
    pub const LmotsSha256N32W2: Self = Self::new(2);
    pub const LmotsSha256N32W4: Self = Self::new(3);
    pub const LmotsSha256N32W8: Self = Self::new(4);
    pub const LmotsSha256N24W1: Self = Self::new(5);
    pub const LmotsSha256N24W2: Self = Self::new(6);
    pub const LmotsSha256N24W4: Self = Self::new(7);
    pub const LmotsSha256N24W8: Self = Self::new(8);
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[repr(C)]
pub struct LmsPublicKey<const N: usize> {
    pub tree_type: LmsAlgorithmType,
    pub otstype: LmotsAlgorithmType,
    pub id: [u8; 16],
    pub digest: [U32<LittleEndian>; N],
}
impl<const N: usize> Default for LmsPublicKey<N> {
    fn default() -> Self {
        Self {
            tree_type: Default::default(),
            otstype: Default::default(),
            id: Default::default(),
            digest: [Default::default(); N],
        }
    }
}

impl<const N: usize> PartialEq for LmsPublicKey<N> {
    // TODO: we whould make a Rust version of the OpenTitan hardened comparisons https://github.com/lowRISC/opentitan/blob/7a61300cf7c409fa68fd892942c1d7b58a7cd4c0/sw/device/lib/base/hardened_memory.c.
    fn eq(&self, other: &Self) -> bool {
        if self.tree_type != other.tree_type {
            return false;
        }
        if self.otstype != other.otstype {
            return false;
        }
        if constant_time_eq::constant_time_eq(&self.id, &other.id) {
            return false;
        }

        let a = self.digest.as_ptr() as *const u8;
        let aslice = unsafe { core::slice::from_raw_parts(a, N * 4) };
        let b = other.digest.as_ptr() as *const u8;
        let bslice = unsafe { core::slice::from_raw_parts(b, N * 4) };
        constant_time_eq::constant_time_eq(aslice, bslice)
    }
}

// Ensure there is no padding (required for AsBytes safety)
static_assert!(
    size_of::<LmsPublicKey<1>>()
        == (size_of::<LmsAlgorithmType>()
            + size_of::<LmotsAlgorithmType>()
            + size_of::<[u8; 16]>()
            + size_of::<[U32<LittleEndian>; 1]>())
);
// Derive doesn't support const generic arrays
unsafe impl<const N: usize> AsBytes for LmsPublicKey<N> {
    fn only_derive_is_allowed_to_implement_this_trait() {}
}
unsafe impl<const N: usize> FromBytes for LmsPublicKey<N> {
    fn only_derive_is_allowed_to_implement_this_trait() {}
}

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Copy, Clone, Debug, Zeroize)]
#[repr(C)]
pub struct LmotsSignature<const N: usize, const P: usize> {
    #[zeroize(skip)]
    pub ots_type: LmotsAlgorithmType,

    #[zeroize(skip)]
    pub nonce: [U32<LittleEndian>; N],

    #[zeroize(skip)]
    pub y: [[U32<LittleEndian>; N]; P],
}

impl<const N: usize, const P: usize> Default for LmotsSignature<N, P> {
    fn default() -> Self {
        Self {
            ots_type: Default::default(),
            nonce: [Default::default(); N],
            y: [[Default::default(); N]; P],
        }
    }
}

impl<const N: usize, const P: usize> PartialEq for LmotsSignature<N, P> {
    // TODO: we whould make a Rust version of the OpenTitan hardened comparisons https://github.com/lowRISC/opentitan/blob/7a61300cf7c409fa68fd892942c1d7b58a7cd4c0/sw/device/lib/base/hardened_memory.c.

    fn eq(&self, other: &Self) -> bool {
        if self.ots_type != other.ots_type {
            return false;
        }

        let a = self.nonce.as_ptr() as *const u8;
        let aslice = unsafe { core::slice::from_raw_parts(a, N * 4) };
        let b = other.nonce.as_ptr() as *const u8;
        let bslice = unsafe { core::slice::from_raw_parts(b, N * 4) };
        if !constant_time_eq::constant_time_eq(aslice, bslice) {
            return false;
        }

        let a = self.y.as_ptr() as *const u8;
        let aslice = unsafe { core::slice::from_raw_parts(a, N * P * 4) };
        let b = other.y.as_ptr() as *const u8;
        let bslice = unsafe { core::slice::from_raw_parts(b, N * P * 4) };
        constant_time_eq::constant_time_eq(aslice, bslice)
    }
}

// Ensure there is no padding (required for AsBytes safety)
static_assert!(
    size_of::<LmotsSignature<1, 1>>()
        == (size_of::<LmotsAlgorithmType>()
            + size_of::<[U32<LittleEndian>; 1]>()
            + size_of::<[[U32<LittleEndian>; 1]; 1]>())
);
// Derive doesn't support const generic arrays
unsafe impl<const N: usize, const P: usize> AsBytes for LmotsSignature<N, P> {
    fn only_derive_is_allowed_to_implement_this_trait() {}
}
unsafe impl<const N: usize, const P: usize> FromBytes for LmotsSignature<N, P> {
    fn only_derive_is_allowed_to_implement_this_trait() {}
}

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LmsSignature<const N: usize, const P: usize, const H: usize> {
    pub q: U32<BigEndian>,

    pub ots: LmotsSignature<N, P>,

    pub tree_type: LmsAlgorithmType,

    pub tree_path: [[U32<LittleEndian>; N]; H],
}
impl<const N: usize, const P: usize, const H: usize> Default for LmsSignature<N, P, H> {
    fn default() -> Self {
        Self {
            q: Default::default(),
            ots: Default::default(),
            tree_type: Default::default(),
            tree_path: [[Default::default(); N]; H],
        }
    }
}

impl<const N: usize, const P: usize, const H: usize> PartialEq for LmsSignature<N, P, H> {
    // TODO: we whould make a Rust version of the OpenTitan hardened comparisons https://github.com/lowRISC/opentitan/blob/7a61300cf7c409fa68fd892942c1d7b58a7cd4c0/sw/device/lib/base/hardened_memory.c.
    fn eq(&self, other: &Self) -> bool {
        if self.q != other.q {
            return false;
        }

        if self.ots != other.ots {
            return false;
        }

        if self.tree_type != other.tree_type {
            return false;
        }

        let a = self.tree_path.as_ptr() as *const u8;
        let aslice = unsafe { core::slice::from_raw_parts(a, N * H * 4) };
        let b = other.tree_path.as_ptr() as *const u8;
        let bslice = unsafe { core::slice::from_raw_parts(b, N * H * 4) };
        constant_time_eq::constant_time_eq(aslice, bslice)
    }
}

// Ensure there is no padding (required for AsBytes safety)
static_assert!(
    size_of::<LmsSignature<1, 1, 1>>()
        == (size_of::<U32<BigEndian>>()
            + size_of::<LmotsSignature<1, 1>>()
            + size_of::<LmsAlgorithmType>()
            + size_of::<[[U32<LittleEndian>; 1]; 1]>())
);
// Derive doesn't support const generic arrays
unsafe impl<const N: usize, const P: usize, const H: usize> AsBytes for LmsSignature<N, P, H> {
    fn only_derive_is_allowed_to_implement_this_trait() {}
}
unsafe impl<const N: usize, const P: usize, const H: usize> FromBytes for LmsSignature<N, P, H> {
    fn only_derive_is_allowed_to_implement_this_trait() {}
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct LmsPrivateKey<const N: usize> {
    pub tree_type: LmsAlgorithmType,

    pub otstype: LmotsAlgorithmType,

    pub id: LmsIdentifier,

    pub seed: [U32<LittleEndian>; N],
}
impl<const N: usize> Default for LmsPrivateKey<N> {
    fn default() -> Self {
        Self {
            tree_type: Default::default(),
            otstype: Default::default(),
            id: Default::default(),
            seed: [Default::default(); N],
        }
    }
}
static_assert!(
    size_of::<LmsPrivateKey<1>>()
        == (size_of::<LmsAlgorithmType>()
            + size_of::<LmotsAlgorithmType>()
            + size_of::<LmsIdentifier>()
            + size_of::<[U32<LittleEndian>; 1]>())
);
// Derive doesn't support const generic arrays
unsafe impl<const N: usize> AsBytes for LmsPrivateKey<N> {
    fn only_derive_is_allowed_to_implement_this_trait() {}
}
unsafe impl<const N: usize> FromBytes for LmsPrivateKey<N> {
    fn only_derive_is_allowed_to_implement_this_trait() {}
}

/// Converts a byte array to word arrays as used in the LMS types. Intended for
/// use at compile-time or in tests / host utilities; not optimized for use in
/// firmware at runtime.
pub const fn bytes_to_words_6(bytes: [u8; 24]) -> [U32<LittleEndian>; 6] {
    let mut result = [U32::ZERO; 6];
    let mut i = 0;
    while i < result.len() {
        result[i] = U32::from_bytes([
            bytes[i * 4],
            bytes[i * 4 + 1],
            bytes[i * 4 + 2],
            bytes[i * 4 + 3],
        ]);
        i += 1;
    }
    result
}

/// Converts a byte array to word arrays as used in the LMS types. Intended for
/// use at compile-time or in tests / host utilities; not optimized for use in
/// firmware at runtime.
pub const fn bytes_to_words_8(bytes: [u8; 32]) -> [U32<LittleEndian>; 8] {
    let mut result = [U32::ZERO; 8];
    let mut i = 0;
    while i < result.len() {
        result[i] = U32::from_bytes([
            bytes[i * 4],
            bytes[i * 4 + 1],
            bytes[i * 4 + 2],
            bytes[i * 4 + 3],
        ]);
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use zerocopy::{LittleEndian, U32};

    #[test]
    fn test_bytes_to_words_6() {
        assert_eq!(
            bytes_to_words_6([
                0x7e, 0x40, 0xc3, 0xed, 0x23, 0x13, 0x9f, 0x1b, 0xa0, 0xad, 0x31, 0x02, 0x4d, 0x15,
                0xe0, 0x39, 0xe8, 0x71, 0xd4, 0x79, 0xfc, 0x53, 0xca, 0xf0
            ]),
            [
                <U32<LittleEndian>>::from(0xedc3407e),
                0x1b9f1323.into(),
                0x0231ada0.into(),
                0x39e0154d.into(),
                0x79d471e8.into(),
                0xf0ca53fc.into()
            ]
        )
    }

    #[test]
    fn test_bytes_to_words_8() {
        assert_eq!(
            bytes_to_words_8([
                0x7e, 0x40, 0xc3, 0xed, 0x23, 0x13, 0x9f, 0x1b, 0xa0, 0xad, 0x31, 0x02, 0x4d, 0x15,
                0xe0, 0x39, 0xe8, 0x71, 0xd4, 0x79, 0xfc, 0x53, 0xca, 0xf0, 0x9a, 0x3c, 0x4b, 0xb8,
                0x1b, 0xde, 0x77, 0x9f
            ]),
            [
                <U32<LittleEndian>>::from(0xedc3407e),
                0x1b9f1323.into(),
                0x0231ada0.into(),
                0x39e0154d.into(),
                0x79d471e8.into(),
                0xf0ca53fc.into(),
                0xb84b3c9a.into(),
                0x9f77de1b.into()
            ]
        )
    }
}
