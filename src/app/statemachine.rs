// src/app/statemachine.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 03 oct 2026
// BSD-3-Clause

use crate::app::*;
use crate::bsp::*;

pub const SM_IDLE: u8 = 0;

static mut SM_STATE: u8 = SM_IDLE;

pub fn sm_init() {
    unsafe {
        SM_STATE = SM_IDLE;
    }
    q_flush();
    dev_led_off();
    bsp_led_exec(LED_CMD_BLINK, 500, true);
}

pub fn sm_step() {
    while let Some(ev) = q_pop() {
        match ev {
            crate::app::EV_TIM => bsp_led_on_tim(),
            _ => {}
        }
    }
}