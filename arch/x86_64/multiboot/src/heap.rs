// Copyright (c) 2019 KATO Takeshi
// Released under the MIT license

/// Heap.

use core::fmt::Write;
use core::mem::size_of;

use bootinfo::*;
use util::cheap_alloc;

use super::log::log;


pub const SLOT_CONVENTIONAL: usize = 0;
pub const SLOT_BOOTHEAP: usize = 1;
pub const SLOT_NORMAL: usize = 2;

const USIZES_IN_ALLOCOBJ: usize = 
    (size_of::<cheap_alloc::CheapAlloc>() + size_of::<usize>() - 1)
    / size_of::<usize>();

static mut allocobj: [usize; cheap_alloc::USIZES_IN_CHEAPALLOC] =
    [0usize; cheap_alloc::USIZES_IN_CHEAPALLOC];

fn _get_alloc() -> &'static mut cheap_alloc::CheapAlloc {
    let rawalloc = unsafe { allocobj.as_mut_ptr() };
    let rawalloc2 = rawalloc as *mut cheap_alloc::CheapAlloc;
    unsafe { &mut *rawalloc2 }
}

pub fn init() {
    let mut slotdefs = cheap_alloc::SlotDefs::new();

    slotdefs.set(SLOT_CONVENTIONAL, 0x00000000,   0x000fffff);
    slotdefs.set(SLOT_BOOTHEAP,     0x00100000,   HEAP_END );
    slotdefs.set(SLOT_NORMAL,       HEAP_END + 1, 0xffffffff);

    _get_alloc().init_with_slotdefs(&slotdefs);
}
