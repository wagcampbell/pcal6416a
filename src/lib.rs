//! This is a platform-agnostic Rust driver for the NXP PCAL6416A IO Expander
//! based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal
//!
//! For further details of the device architecture and operation, please refer
//! to the official [`Datasheet`].
//!
//! [`Datasheet`]: https://www.nxp.com/docs/en/data-sheet/PCAL6416A.pdf
#![doc = include_str!("../README.md")]
#![no_std]
#![allow(missing_docs)]

pub mod hl;
pub mod ll;

#[derive(Clone, Copy)]
pub enum Address {
    Default0,
    Alternative1,
}

impl Address {
    fn addr(self) -> u8 {
        use Address::*;
        match self {
            Default0 => 0b0100000,
            Alternative1 => 0b0100001,
        }
    }
}

trait SealedMode {}

#[allow(private_bounds)]
pub trait Mode: SealedMode {}

pub struct Async;
pub struct Blocking;

impl SealedMode for Async {}
impl SealedMode for Blocking {}

impl Mode for Async {}
impl Mode for Blocking {}