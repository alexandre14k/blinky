// src/bsp/driver/clock.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 03 oct 2026
// BSD-3-Clause

use crate::bsp::{DEV_HSI_HZ, dev_init};

pub fn clk_init() {
    dev_init();
}

pub fn clk_hz() -> u32 {
    DEV_HSI_HZ
}