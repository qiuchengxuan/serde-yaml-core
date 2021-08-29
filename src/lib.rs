#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(warnings)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(test)]
#[macro_use]
extern crate std;

pub mod ser;

#[doc(inline)]
pub use self::ser::to_fmt;
