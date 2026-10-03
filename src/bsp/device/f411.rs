// src/bsp/device/f411.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 03 oct 2026
// BSD-3-Clause

use crate::bsp::{rt_reg_r, rt_reg_w};

pub const DEV_STACK_TOP: usize = 0x2002_0000;
pub const DEV_HSI_HZ: u32 = 16_000_000;
pub const DEV_RCC_BASE: u32 = 0x4002_3800;
pub const DEV_RCC_AHB1ENR_OFF: u32 = 0x30;
pub const DEV_RCC_APB1ENR_OFF: u32 = 0x40;
pub const DEV_GPIOA_BASE: u32 = 0x4002_0000;
pub const DEV_TIM2_BASE: u32 = 0x4000_0000;

const GPIO_MODER_OFF: u32 = 0x00;
const GPIO_BSRR_OFF: u32 = 0x18;
const GPIO_AFRL_OFF: u32 = 0x20;
const GPIO_AFRH_OFF: u32 = 0x24;
const DEV_LED_PORT: u32 = DEV_GPIOA_BASE;
const DEV_LED_PIN: u32 = 5;

const DEV_GPIOA_EN: u32 = 1 << 0;
const DEV_GPIOB_EN: u32 = 1 << 1;
const DEV_GPIOC_EN: u32 = 1 << 2;
const DEV_TIM2_EN: u32 = 1 << 0;

fn f411_bus_en(off: u32, bit: u32) {
    let v = rt_reg_r(DEV_RCC_BASE + off);
    rt_reg_w(DEV_RCC_BASE + off, v | bit);
    rt_reg_r(DEV_RCC_BASE + off);
}

fn f411_gpio_mode(base: u32, pin: u32, mode: u32) {
    let sh = pin * 2;
    let v = rt_reg_r(base + GPIO_MODER_OFF);
    rt_reg_w(base + GPIO_MODER_OFF, (v & !(3 << sh)) | (mode << sh));
}

fn f411_gpio_af(base: u32, pin: u32, af: u32) {
    let off = if pin < 8 { GPIO_AFRL_OFF } else { GPIO_AFRH_OFF };
    let sh = (pin % 8) * 4;
    let v = rt_reg_r(base + off);
    rt_reg_w(base + off, (v & !(0xF << sh)) | (af << sh));
}

pub fn dev_gpio_out(base: u32, pin: u32) {
    f411_gpio_mode(base, pin, 1);
}

pub fn dev_gpio_af(base: u32, pin: u32, af: u32) {
    f411_gpio_mode(base, pin, 2);
    f411_gpio_af(base, pin, af);
}

pub fn dev_gpio_write(base: u32, pin: u32, on: bool) {
    if on {
        rt_reg_w(base + GPIO_BSRR_OFF, 1 << pin);
    } else {
        rt_reg_w(base + GPIO_BSRR_OFF, 1 << (pin + 16));
    }
}

pub fn dev_led_on() {
    dev_gpio_write(DEV_LED_PORT, DEV_LED_PIN, true);
}

pub fn dev_led_off() {
    dev_gpio_write(DEV_LED_PORT, DEV_LED_PIN, false);
}

pub fn dev_init() {
    f411_bus_en(DEV_RCC_AHB1ENR_OFF,
        DEV_GPIOA_EN | DEV_GPIOB_EN | DEV_GPIOC_EN);
    dev_gpio_out(DEV_LED_PORT, DEV_LED_PIN);
    dev_led_off();
}

pub fn dev_tim2_clk() {
    f411_bus_en(DEV_RCC_APB1ENR_OFF, DEV_TIM2_EN);
}

pub fn dev_tim2_pins(on: &[bool; 4]) {
    if on[0] { dev_gpio_af(DEV_GPIOA_BASE, 0, 1); }
    if on[1] { dev_gpio_af(DEV_GPIOA_BASE, 1, 1); }
    if on[2] { dev_gpio_af(DEV_GPIOA_BASE, 2, 1); }
    if on[3] { dev_gpio_af(DEV_GPIOA_BASE, 3, 1); }
}