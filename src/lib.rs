//! Utility functions for generating random numbers with a fixed number of set bits (ones).
//!
//! # Example
//!
//! ```rust
//! use rand::thread_rng;
//! use rand_bits::RngBits;
//!
//! let mut rng = thread_rng();
//! let x: u8 = rng.gen_bits(4); // generates a u8 with 4 set bits
//! assert_eq!(x.count_ones(), 4);
//! let y: u16 = rng.gen_bits(15); // generates a u16 with 15 set bits
//! assert_eq!(y.count_ones(), 15);
//! let z: u64 = rng.gen_bits(1); // generates a u64 with 1 set bits
//! assert_eq!(z.count_ones(), 1);
//! ```
//!
//! # License
//!
//! This crate is licensed under the MIT License.

#![forbid(unsafe_code)]

use std::cmp::min;

use phf::{phf_map, Map};
use rand::Rng;

const MAPPING: Map<u8, &'static [u8]> = phf_map! {
    0_u8 => &[0x00],
    1_u8 => &[0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80],
    2_u8 => &[0x03, 0x05, 0x06, 0x09, 0x0A, 0x0C, 0x11, 0x12, 0x14, 0x18, 0x21, 0x22, 0x24, 0x28, 0x30, 0x41, 0x42, 0x44, 0x48, 0x50, 0x60, 0x81, 0x82, 0x84, 0x88, 0x90, 0xA0, 0xC0],
    3_u8 => &[0x07, 0x0B, 0x0D, 0x0E, 0x13, 0x15, 0x16, 0x19, 0x1A, 0x1C, 0x23, 0x25, 0x26, 0x29, 0x2A, 0x2C, 0x31, 0x32, 0x34, 0x38, 0x43, 0x45, 0x46, 0x49, 0x4A, 0x4C, 0x51, 0x52, 0x54, 0x58, 0x61, 0x62, 0x64, 0x68, 0x70, 0x83, 0x85, 0x86, 0x89, 0x8A, 0x8C, 0x91, 0x92, 0x94, 0x98, 0xA1, 0xA2, 0xA4, 0xA8, 0xB0, 0xC1, 0xC2, 0xC4, 0xC8, 0xD0, 0xE0],
    4_u8 => &[0x0F, 0x17, 0x1B, 0x1D, 0x1E, 0x27, 0x2B, 0x2D, 0x2E, 0x33, 0x35, 0x36, 0x39, 0x3A, 0x3C, 0x47, 0x4B, 0x4D, 0x4E, 0x53, 0x55, 0x56, 0x59, 0x5A, 0x5C, 0x63, 0x65, 0x66, 0x69, 0x6A, 0x6C, 0x71, 0x72, 0x74, 0x78, 0x87, 0x8B, 0x8D, 0x8E, 0x93, 0x95, 0x96, 0x99, 0x9A, 0x9C, 0xA3, 0xA5, 0xA6, 0xA9, 0xAA, 0xAC, 0xB1, 0xB2, 0xB4, 0xB8, 0xC3, 0xC5, 0xC6, 0xC9, 0xCA, 0xCC, 0xD1, 0xD2, 0xD4, 0xD8, 0xE1, 0xE2, 0xE4, 0xE8, 0xF0],
    5_u8 => &[0x1F, 0x2F, 0x37, 0x3B, 0x3D, 0x3E, 0x4F, 0x57, 0x5B, 0x5D, 0x5E, 0x67, 0x6B, 0x6D, 0x6E, 0x73, 0x75, 0x76, 0x79, 0x7A, 0x7C, 0x8F, 0x97, 0x9B, 0x9D, 0x9E, 0xA7, 0xAB, 0xAD, 0xAE, 0xB3, 0xB5, 0xB6, 0xB9, 0xBA, 0xBC, 0xC7, 0xCB, 0xCD, 0xCE, 0xD3, 0xD5, 0xD6, 0xD9, 0xDA, 0xDC, 0xE3, 0xE5, 0xE6, 0xE9, 0xEA, 0xEC, 0xF1, 0xF2, 0xF4, 0xF8],
    6_u8 => &[0x3F, 0x5F, 0x6F, 0x77, 0x7B, 0x7D, 0x7E, 0x9F, 0xAF, 0xB7, 0xBB, 0xBD, 0xBE, 0xCF, 0xD7, 0xDB, 0xDD, 0xDE, 0xE7, 0xEB, 0xED, 0xEE, 0xF3, 0xF5, 0xF6, 0xF9, 0xFA, 0xFC],
    7_u8 => &[0x7F, 0xBF, 0xDF, 0xEF, 0xF7, 0xFB, 0xFD, 0xFE],
    8_u8 => &[0xFF],
};

/// A generic random value distribution, implemented for many primitive types.
/// Usually generates values with a numerically uniform distribution, and with a
/// range appropriate to the type.
///
/// Based on [`rand::distributions::Standard`].
pub struct Standard;

/// Types (distributions) that can be used to create a random instance of `T`.
///
/// Based on [`rand::distributions::Distribution`].
pub trait Distribution<T> {
    /// Generate a random value of `T`, using `rng` as the source of randomness.
    fn sample<R>(&self, rng: &mut R, bits: u8) -> T
    where
        R: Rng + ?Sized;
}

impl Distribution<u8> for Standard {
    fn sample<R>(&self, rng: &mut R, bits: u8) -> u8
    where
        R: Rng + ?Sized,
    {
        let values = MAPPING.get(&bits).expect("bits count out of range");
        let index = {
            let index: usize = rng.gen();
            index % values.len()
        };
        values[index]
    }
}

impl Distribution<u16> for Standard {
    fn sample<R>(&self, rng: &mut R, bits: u8) -> u16
    where
        R: Rng + ?Sized,
    {
        assert!((0..=(u16::BITS as u8)).contains(&bits), "bits count out of range");
        let mut value = 0;
        let mut cnt = 0;
        for k in (0..(u16::BITS / u8::BITS)).rev() {
            let minbits = (bits - cnt).checked_sub((k * u8::BITS) as u8).unwrap_or_default();
            let maxbits = min(bits - cnt, u8::BITS as u8);
            let bits = rng.gen_range(minbits..=maxbits);
            let values = MAPPING.get(&bits).expect("bits count out of range");
            let index = {
                let index: usize = rng.gen();
                index % values.len()
            };
            value = (value << 8) | (values[index] as u16);
            cnt += bits;
        }
        value
    }
}

impl Distribution<u32> for Standard {
    fn sample<R>(&self, rng: &mut R, bits: u8) -> u32
    where
        R: Rng + ?Sized,
    {
        assert!((0..=(u32::BITS as u8)).contains(&bits), "bits count out of range");
        let mut value = 0;
        let mut cnt = 0;
        for k in (0..(u32::BITS / u8::BITS)).rev() {
            let minbits = (bits - cnt).checked_sub((k * u8::BITS) as u8).unwrap_or_default();
            let maxbits = min(bits - cnt, u8::BITS as u8);
            let bits = rng.gen_range(minbits..=maxbits);
            let values = MAPPING.get(&bits).expect("bits count out of range");
            let index = {
                let index: usize = rng.gen();
                index % values.len()
            };
            value = (value << 8) | (values[index] as u32);
            cnt += bits;
        }
        value
    }
}

impl Distribution<u64> for Standard {
    fn sample<R>(&self, rng: &mut R, bits: u8) -> u64
    where
        R: Rng + ?Sized,
    {
        assert!((0..=(u64::BITS as u8)).contains(&bits), "bits count out of range");
        let mut value = 0;
        let mut cnt = 0;
        for k in (0..(u64::BITS / u8::BITS)).rev() {
            let minbits = (bits - cnt).checked_sub((k * u8::BITS) as u8).unwrap_or_default();
            let maxbits = min(bits - cnt, u8::BITS as u8);
            let bits = rng.gen_range(minbits..=maxbits);
            let values = MAPPING.get(&bits).expect("bits count out of range");
            let index = {
                let index: usize = rng.gen();
                index % values.len()
            };
            value = (value << 8) | (values[index] as u64);
            cnt += bits;
        }
        value
    }
}

/// An automatically-implemented extension trait on [`rand::Rng`].
///
/// # Example:
///
/// ```rust
/// # use rand::thread_rng;
/// use rand_bits::RngBits;
///
/// fn foo<R>(rng: &mut R) -> u16
/// where
///     R: RngBits + ?Sized,
/// {
///     rng.gen_bits(16)
/// }
///
/// # let v = foo(&mut thread_rng());
/// ```
pub trait RngBits: Rng {
    /// Return a random value supporting the [`Standard`] distribution with a chosen
    /// number of bits set to active.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rand::thread_rng;
    /// use rand_bits::RngBits;
    ///
    /// let mut rng = thread_rng();
    /// let x: u32 = rng.gen_bits(11);
    /// println!("{}", x);
    /// ```
    fn gen_bits<T>(&mut self, bits: u8) -> T
    where
        Standard: Distribution<T>,
    {
        Standard.sample(self, bits)
    }
}

impl<R> RngBits for R where R: Rng {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8() {
        let mut rng = rand::thread_rng();
        for i in 0..=(u8::BITS as u8) {
            let n: u8 = rng.gen_bits(i);
            assert_eq!(n.count_ones() as u8, i);
        }
    }

    #[test]
    fn u16() {
        let mut rng = rand::thread_rng();
        for i in 0..=(u16::BITS as u8) {
            let n: u16 = rng.gen_bits(i);
            assert_eq!(n.count_ones() as u8, i);
        }
    }

    #[test]
    fn u32() {
        let mut rng = rand::thread_rng();
        for i in 0..=(u32::BITS as u8) {
            let n: u32 = rng.gen_bits(i);
            assert_eq!(n.count_ones() as u8, i);
        }
    }

    #[test]
    fn u64() {
        let mut rng = rand::thread_rng();
        for i in 0..=(u64::BITS as u8) {
            let n: u64 = rng.gen_bits(i);
            assert_eq!(n.count_ones() as u8, i);
        }
    }
}
