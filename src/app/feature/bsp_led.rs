// src/app/feature/bsp_led.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 03 oct 2026
// BSD-3-Clause

use crate::bsp::{TIM2, dev_led_off, dev_led_on};

pub const LED_CMD_OFF: u8 = 0x00;
pub const LED_CMD_ON: u8 = 0x01;
pub const LED_CMD_BLINK: u8 = 0x02;

pub static mut LED_BLINKING: bool = false;
pub static mut LED_STATE: bool = false;

pub fn bsp_led_exec(opt: u8, arg: u16, has_arg: bool) {
    match (opt, has_arg) {
        (LED_CMD_OFF, false) => {
            unsafe {
                LED_BLINKING = false;
                LED_STATE = false;
            }
            TIM2.stop();
            dev_led_off();
        }
        (LED_CMD_ON, false) => {
            unsafe {
                LED_BLINKING = false;
                LED_STATE = true;
            }
            TIM2.stop();
            dev_led_on();
        }
        (LED_CMD_BLINK, true) => {
            if arg == 0 || arg > 10_000 {
                return;
            }
            unsafe {
                LED_BLINKING = true;
                LED_STATE = true;
            }
            dev_led_on();
            TIM2.init_ms(arg as u32);
            TIM2.irq_arm(true);
            TIM2.start();
        }
        _ => {},
    }
}

pub fn bsp_led_on_tim() {
    unsafe {
        if !LED_BLINKING {
            return;
        }
        if LED_STATE {
            dev_led_off();
            LED_STATE = false;
        } else {
            dev_led_on();
            LED_STATE = true;
        }
    }
}