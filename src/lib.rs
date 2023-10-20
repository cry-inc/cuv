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
