//! A Rust library for compressed unit vectors.
//!
//! It can be used to efficiently store and transfer things like normal vectors used in computer graphics.
//! You can use it lower the memory footsprint or reduce the size of files on disk.
//! Intead of three 32 bit floats you can represent the unit vector with a single 16 bit unsigned integer.
//!
//! The compression itself is **lossy**, so most input values will be mapped to something slighty different when being unpacked.
//! For many use cases this loss is acceptable. Please make sure this applies to your case as well.
//!
//! ### Getting Started
//!
//! The following example shows how to create and read a compressed unit vector:
//!
//! ```
//! use cuv::CompUnitVec;
//!
//! let input = [1.0, 0.0, 0.0];
//! let packed = CompUnitVec::from_slice(&input);
//! let unpacked = packed.get();
//! assert_eq!(unpacked, input);
//! ```
//! There is also a low level interface which requires the manual creation of a lookup-table for unpacking:
//!
//! ```
//! let packed: u16 = cuv::pack(1.0, 0.0, 0.0);
//! let lut = cuv::create_lut();
//! let unpacked = cuv::unpack(packed, &lut);
//! assert_eq!(unpacked, [1.0, 0.0, 0.0]);
//! ```
//!
//! ### Invalid Input
//!
//! If the input is not a unit vector, it will still work because the input is internally normalized.
//! The unpacked output vectors will be always normalized and true unit vectors.
//!
//! The algorithm can handle invalid inputs and will map them to a valid compressed unit vector.
//! For example, using `[0, 0, 0]` as input will unpack as `[0, 0, 1]`.
//! The same applies for `NaN` or infinity values.

#![forbid(unsafe_code)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value
)]

mod comp_unit_vec;
mod core;

// Low level interface
pub use core::create_lut;
pub use core::pack;
pub use core::unpack;

// High level rustified interface
pub use comp_unit_vec::CompUnitVec;
