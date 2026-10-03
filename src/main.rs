// src/main.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 03 oct 2026
// BSD-3-Clause

#![no_std]
#![no_main]
#![allow(static_mut_refs)]

mod app;
mod bsp;

use app::*;
use bsp::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    clk_init();
    tmr_init();
    sm_init();
    loop {
        sm_step();
    }
}