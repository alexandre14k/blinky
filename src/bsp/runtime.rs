// src/bsp/runtime.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 03 oct 2026
// BSD-3-Clause

use core::panic::PanicInfo;
use crate::bsp::device::DEV_STACK_TOP;

pub type RtHandler = unsafe extern "C" fn();

pub union RtVector {
    pub handler: RtHandler,
    pub word: usize,
}

const RT_DEFAULT: RtVector = RtVector { handler: rt_isr_default };

extern "C" {
    static _sidata: u32;
    static mut _data: u32;
    static mut _edata: u32;
    static mut _bss: u32;
    static mut _ebss: u32;
    fn main() -> !;
    fn isr_systick();
    fn isr_tim2();
}

#[no_mangle]
pub extern "C" fn _init() {}

#[no_mangle]
pub extern "C" fn _fini() {}

#[no_mangle]
pub extern "C" fn rt_isr_default() {
    loop {
        rt_wait();
    }
}

#[link_section = ".after_reset.rt_reset"]
#[no_mangle]
pub unsafe extern "C" fn rt_reset() {
    rt_data_init();
    rt_vtor_set();
    main()
}

const fn rt_table() -> [RtVector; 55] {
    let mut t = [RT_DEFAULT; 55];
    t[0] = RtVector { word: DEV_STACK_TOP };
    t[1] = RtVector { handler: rt_reset };
    t[15] = RtVector { handler: isr_systick };
    t[44] = RtVector { handler: isr_tim2 };
    t
}

#[link_section = ".init"]
#[used]
pub static RT_VECTORS: [RtVector; 55] = rt_table();

pub fn rt_data_init() {
    unsafe {
        let mut s = core::ptr::addr_of!(_sidata) as *const u32;
        let mut d = core::ptr::addr_of_mut!(_data);
        let de = core::ptr::addr_of!(_edata) as usize;
        while (d as usize) < de {
            d.write_volatile(s.read_volatile());
            s = s.add(1);
            d = d.add(1);
        }
        let mut b = core::ptr::addr_of_mut!(_bss);
        let be = core::ptr::addr_of!(_ebss) as usize;
        while (b as usize) < be {
            b.write_volatile(0);
            b = b.add(1);
        }
    }
}

pub const RT_FLASH_BASE: u32 = 0x0800_0000;
const RT_SCB_VTOR: u32 = 0xE000_ED08;

pub fn rt_vtor_set() {
    rt_reg_w(RT_SCB_VTOR, RT_FLASH_BASE);
}

pub fn rt_reg_w(addr: u32, val: u32) {
    unsafe { core::ptr::write_volatile(addr as *mut u32, val) }
}

pub fn rt_reg_r(addr: u32) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}

pub fn rt_wait() {
    unsafe { core::arch::asm!("wfi") }
}

pub fn rt_irq_enable() {
    unsafe { core::arch::asm!("cpsie i") }
}

pub fn rt_irq_disable() {
    unsafe { core::arch::asm!("cpsid i") }
}

pub fn rt_panic_report() {
    let msg: &[u8] = b"\r\nsm panic\r\n";
    for b in msg {
        while rt_reg_r(0x4000_4400) & (1 << 7) == 0 {}
        rt_reg_w(0x4000_4404, *b as u32);
    }
}

#[panic_handler]
fn rt_panic(_i: &PanicInfo) -> ! {
    rt_irq_disable();
    rt_panic_report();
    rt_panic_blink_arm();
    loop {
        rt_panic_wait_edge();
        rt_reg_w(0x4002_0018, 1 << 5);
        rt_panic_wait_edge();
        rt_reg_w(0x4002_0018, 1 << (5 + 16));
    }
}

fn rt_panic_blink_arm() {
    rt_reg_w(0x4000_0000 + 0x28, 15);
    rt_reg_w(0x4000_0000 + 0x2C, 49_999);
    rt_reg_w(0x4000_0000 + 0x10, 0);
    rt_reg_w(0x4000_0000 + 0x14, 1);
    rt_reg_w(0x4000_0000 + 0x00, 0x81);
}

fn rt_panic_wait_edge() {
    let sr = 0x4000_0000 + 0x10;
    while rt_reg_r(sr) & 1 == 0 {}
    rt_reg_w(sr, 0);
}