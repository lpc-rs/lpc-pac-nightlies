//! Peripheral access API for LPC13XX-PAC microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.25.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.25.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [lpc-pac](https://github.com/lpc-rs/lpc-pac)
//!
//! This crate supports all LPC13XX-PAC devices; for the complete list please
//! see:
//! [lpc13xx-pac](https://crates.io/crates/lpc13xx-pac)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
mod generic;
pub use self::generic::*;
#[cfg(feature = "lpc1311")]
pub mod lpc1311;

#[cfg(feature = "lpc1313")]
pub mod lpc1313;

#[cfg(feature = "lpc1342")]
pub mod lpc1342;

#[cfg(feature = "lpc1343")]
pub mod lpc1343;

