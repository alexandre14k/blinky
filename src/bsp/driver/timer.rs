// src/bsp/driver/timer.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 03 oct 2026
// BSD-3-Clause

use crate::bsp::*;

const TMR_SYST_CSR: u32 = 0xE000_E010;
const TMR_SYST_RVR: u32 = 0xE000_E014;
const TMR_SYST_CVR: u32 = 0xE000_E018;

const TMR_SYST_ENABLE: u32 = 1 << 0;
const TMR_SYST_TICKINT: u32 = 1 << 1;
const TMR_SYST_CLKSOURCE: u32 = 1 << 2;

static mut TMR_MS: u32 = 0;

pub fn tmr_init() {
    rt_reg_w(TMR_SYST_RVR, clk_hz() / 1_000 - 1);
    rt_reg_w(TMR_SYST_CVR, 0);
    rt_reg_w(TMR_SYST_CSR,
        TMR_SYST_ENABLE | TMR_SYST_TICKINT | TMR_SYST_CLKSOURCE);
    rt_irq_enable();
}

pub fn tmr_tick() {
    unsafe {
        let p = core::ptr::addr_of_mut!(TMR_MS);
        let v = core::ptr::read_volatile(p);
        core::ptr::write_volatile(p, v.wrapping_add(1));
    }
}

const TIM_CR1_OFF: u32 = 0x00;
const TIM_DIER_OFF: u32 = 0x0C;
const TIM_SR_OFF: u32 = 0x10;
const TIM_PSC_OFF: u32 = 0x28;
const TIM_ARR_OFF: u32 = 0x2C;
const TIM_BDTR_OFF: u32 = 0x44;
const TIM_EGR_OFF: u32 = 0x14;

const TIM_EGR_UG: u32 = 1 << 0;
const TIM_CR1_CEN: u32 = 1 << 0;
const TIM_CR1_ARPE: u32 = 1 << 7;
const TIM_DIER_UIE: u32 = 1 << 0;
const TIM_SR_UIF: u32 = 1 << 0;
const TIM_BDTR_MOE: u32 = 1 << 15;

const TIM_NVIC_ISER0: u32 = 0xE000_E100;
const TIM_NVIC_ISER1: u32 = 0xE000_E104;

pub const TIM_MATRIX: [[bool; 4]; 3] = [
    [true, false, false, false],
    [false, false, false, false],
    [true, true, true, true],
];

pub struct Timer {
    base: u32,
    adv: bool,
    idx: usize,
    irq: u32,
    clk_en: fn(),
    pins_cfg: fn(&[bool; 4]),
}

impl Timer {
    pub const fn make(
        base: u32,
        adv: bool,
        idx: usize,
        irq: u32,
        clk_en: fn(),
        pins_cfg: fn(&[bool; 4]),
    ) -> Timer {
        Timer { base, adv, idx, irq, clk_en, pins_cfg }
    }

    fn reg_w(&self, off: u32, val: u32) {
        rt_reg_w(self.base + off, val);
    }

    fn reg_r(&self, off: u32) -> u32 {
        rt_reg_r(self.base + off)
    }

    fn nvic_enable(&self) {
        if self.irq < 32 {
            rt_reg_w(TIM_NVIC_ISER0, 1 << self.irq);
        } else {
            rt_reg_w(TIM_NVIC_ISER1, 1 << (self.irq % 32));
        }
    }

    pub fn init_ms(&self, ms: u32) {
        (self.clk_en)();
        (self.pins_cfg)(&TIM_MATRIX[self.idx]);
        self.reg_w(TIM_CR1_OFF, TIM_CR1_ARPE);
        self.reg_w(TIM_PSC_OFF,
            crate::bsp::clk_hz() / 1_000 - 1);
        self.reg_w(TIM_ARR_OFF, ms - 1);
        self.reg_w(TIM_EGR_OFF, TIM_EGR_UG);
        self.reg_w(TIM_SR_OFF, 0);
        if self.adv {
            self.reg_w(TIM_BDTR_OFF, TIM_BDTR_MOE);
        }
    }

    pub fn start(&self) {
        let v = self.reg_r(TIM_CR1_OFF);
        self.reg_w(TIM_CR1_OFF, v | TIM_CR1_CEN);
    }

    pub fn stop(&self) {
        let v = self.reg_r(TIM_CR1_OFF);
        self.reg_w(TIM_CR1_OFF, v & !TIM_CR1_CEN);
    }

    pub fn irq_arm(&self, on: bool) {
        let v = self.reg_r(TIM_DIER_OFF);
        if on {
            self.reg_w(TIM_DIER_OFF, v | TIM_DIER_UIE);
        } else {
            self.reg_w(TIM_DIER_OFF, v & !TIM_DIER_UIE);
        }
        if on {
            self.nvic_enable();
        }
    }

    pub fn irq_clear(&self) {
        let v = self.reg_r(TIM_SR_OFF);
        self.reg_w(TIM_SR_OFF, v & !TIM_SR_UIF);
    }
}

pub static TIM2: Timer = Timer::make(
    DEV_TIM2_BASE, false, 1, 28, dev_tim2_clk, dev_tim2_pins);