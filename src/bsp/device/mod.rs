// src/bsp/device/mod.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 03 oct 2026
// BSD-3-Clause

#[cfg(feature = "f411")]
mod f411;
#[cfg(feature = "f411")]
pub use f411::*;