//! Peripheral access API for LPC81X-PAC microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.25.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.25.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [lpc-pac](https://github.com/lpc-rs/lpc-pac)
//!
//! This crate supports all LPC81X-PAC devices; for the complete list please
//! see:
//! [lpc81x-pac](https://crates.io/crates/lpc81x-pac)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
mod generic;
pub use self::generic::*;
#[cfg(feature = "lpc810")]
pub mod lpc810;

#[cfg(feature = "lpc811")]
pub mod lpc811;

#[cfg(feature = "lpc812")]
pub mod lpc812;

