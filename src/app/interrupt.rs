// src/app/interrupt.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 03 oct 2026
// BSD-3-Clause

use crate::app::{EV_TIM, EV_TICK, q_push};
use crate::bsp::{TIM2, tmr_tick};

#[no_mangle]
pub extern "C" fn isr_systick() {
    tmr_tick();
    q_push(EV_TICK);
}

#[no_mangle]
pub extern "C" fn isr_tim2() {
    TIM2.irq_clear();
    q_push(EV_TIM);
}