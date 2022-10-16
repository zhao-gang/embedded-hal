//! An asynchronous Hardware Abstraction Layer (HAL) for embedded systems
//!
//! **NOTE** These traits are still experimental. At least one breaking
//! change to this crate is expected in the future (changing from GATs to
//! `async fn`), but there might be more.
//!
//! **NOTE** The traits and modules in this crate should follow the same structure as in
//! `embedded-hal` to ease merging and migration.

#![warn(missing_docs)]
#![no_std]
#![cfg_attr(feature = "nightly", feature(type_alias_impl_trait))]

#[cfg(feature = "nightly")]
pub mod delay;

#[cfg(feature = "nightly")]
pub mod digital;

#[cfg(feature = "nightly")]
pub mod i2c;

#[cfg(feature = "nightly")]
pub mod spi;
