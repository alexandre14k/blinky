// src/app/queue.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 03 oct 2026
// BSD-3-Clause

use core::ptr::{read_volatile, write_volatile};

pub const Q_SIZE: usize = 16;

pub const EV_TICK: u8 = 1;
pub const EV_TIM: u8 = 2;

pub struct EvQueue {
    buf: [u8; Q_SIZE],
    head: u32,
    tail: u32,
}

impl EvQueue {
    pub const fn make() -> EvQueue {
        EvQueue { buf: [0; Q_SIZE], head: 0, tail: 0 }
    }

    pub fn push(&mut self, ev: u8) -> bool {
        unsafe {
            let head = read_volatile(
                core::ptr::addr_of!(self.head));
            let tail = read_volatile(
                core::ptr::addr_of!(self.tail));
            let next = (head + 1) % Q_SIZE as u32;
            if next == tail {
                return false;
            }
            let bp = core::ptr::addr_of_mut!(self.buf)
                .cast::<u8>();
            write_volatile(bp.add(head as usize), ev);
            write_volatile(
                core::ptr::addr_of_mut!(self.head), next);
            true
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        unsafe {
            let head = read_volatile(
                core::ptr::addr_of!(self.head));
            let tail = read_volatile(
                core::ptr::addr_of!(self.tail));
            if tail == head {
                return None;
            }
            let bp = core::ptr::addr_of!(self.buf)
                .cast::<u8>();
            let ev = read_volatile(bp.add(tail as usize));
            write_volatile(
                core::ptr::addr_of_mut!(self.tail),
                (tail + 1) % Q_SIZE as u32);
            Some(ev)
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            let head = read_volatile(
                core::ptr::addr_of!(self.head));
            write_volatile(
                core::ptr::addr_of_mut!(self.tail), head);
        }
    }
}

pub static mut Q: EvQueue = EvQueue::make();

pub fn q_push(ev: u8) -> bool {
    unsafe { Q.push(ev) }
}

pub fn q_pop() -> Option<u8> {
    unsafe { Q.pop() }
}

pub fn q_flush() {
    unsafe { Q.flush() }
}