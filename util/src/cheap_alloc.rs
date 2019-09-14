// Uniqos  --  Unique Operating System
// (c) 2019 KATO Takeshi
// Released under the MIT license
//
/// Cheap allocator implements.

//extern crate alloc;

use core::alloc::Layout;
use core::mem::{transmute, size_of};

use super::boxed::X;
use super::cheap_list::*;
use super::error::Error;
use super::ops;


const SLOT_NUM: usize = 8;

type SlotMask = u8;

// This code cannot to be compile...
/*
use core::marker::Sized;
unsafe fn generate_with0<T: Sized>() -> T
{
    mem::transmute::<[u8; mem::size_of::<T>()], T>([0u8; mem::size_of::<T>()])
}
*/
// So instead to use this:
macro_rules! generate_with0 {
    ($type:ty) => {
        transmute::<[u8; size_of::<$type>()], $type>([0u8; size_of::<$type>()])
    };
}

#[derive(Clone, Copy)]
pub struct _SlotDefEnt {
    start: usize,
    end: usize,
}
pub struct SlotDefs {
    defs: [_SlotDefEnt; SLOT_NUM],
}

impl SlotDefs {
    pub fn new() -> Self {
        Self {
            defs: [_SlotDefEnt { start: 0, end: 0 }; SLOT_NUM],
        }
    }
    pub fn set(&mut self, i: usize, start: usize, end: usize) {
        self.defs[i].start = start;
        self.defs[i].end = end;
    }
}

#[derive(Clone, Copy)]
struct AdrRange {
    adr: usize,
    bytes: usize,  // Unused if bytes == 0.
}

impl AdrRange {
    fn new() -> Self {
        Self { adr: 0, bytes: 0 }
    }
    fn set(&mut self, adr: usize, bytes: usize) {
        self.adr = adr;
        self.bytes = bytes;
    }
}

struct AdrSlot {
    start: usize,
    end: usize,
    free_ranges: SingleForwardList::<AdrRange, RawRefer<ForwardEnt<AdrRange>>>,
    used_ranges: SingleForwardList::<AdrRange, RawRefer<ForwardEnt<AdrRange>>>,
}

impl AdrSlot {
    fn new() -> Self {
        Self {
          start: 0, end: 0,
          free_ranges:
           SingleForwardList::<AdrRange, RawRefer<ForwardEnt<AdrRange>>>::new(),
          used_ranges:
           SingleForwardList::<AdrRange, RawRefer<ForwardEnt<AdrRange>>>::new(),
        }
    }
}

fn is_masked(i: usize, slotmask: SlotMask) -> bool {
    (1 << i) & slotmask != 0
}

pub struct CheapAlloc {
    free_buf_list:
        SingleForwardList::<AdrRange, RawRefer<ForwardEnt<AdrRange>>>,

    slots: [AdrSlot; SLOT_NUM],
    range_buf: [ForwardEnt<AdrRange>; 256],
}

pub const USIZES_IN_CHEAPALLOC: usize = 
    (size_of::<CheapAlloc>() + size_of::<usize>() - 1) / size_of::<usize>();

impl CheapAlloc {

    pub fn from<'buf>(buf: &'buf mut [usize; USIZES_IN_CHEAPALLOC])
        -> &'buf mut Self
    {
        let rawalloc = unsafe { buf.as_mut_ptr() } as *mut Self;
        unsafe { &mut *rawalloc }
    }

    pub fn init_with_slotdefs(&mut self, defs: &SlotDefs) {
        // initialize.
        self.free_buf_list =
            SingleForwardList::<AdrRange, RawRefer<ForwardEnt<AdrRange>>>::
            new();
        for slot in self.slots.iter_mut() {
            *slot = AdrSlot::new();
        }
        for buf in self.range_buf.iter_mut() {
            self.free_buf_list.push_front(buf.into());
        }

        // load.
        for (slot, &def) in defs.defs.iter().enumerate() {
            self.slots[slot].start = def.start;
            self.slots[slot].end = def.end;
        }
    }

    pub fn add_free_range(&mut self, slot: usize, adr: usize, bytes: usize)
        -> Result<(), Error>
    {
        if bytes == 0 {
            return Ok(());
        }
        let r = self.new_adrrange();
        match r {
            Err(e) => Err(e),
            Ok(ent) => {
                ent.ref_elem_mut().set(adr, bytes);
                self.slots[slot].free_ranges.push_front(ent.into());
                Ok(())
            }
        }
    }

    fn _new() -> Self {
        Self {
            free_buf_list:
                SingleForwardList::<AdrRange, RawRefer<ForwardEnt<AdrRange>>>::
                new(),
            slots:
                unsafe { generate_with0!([AdrSlot; SLOT_NUM]) },
            /* or safe represents:
             * [
             *   AdrSlot::new(), AdrSlot::new(), AdrSlot::new(), AdrSlot::new(),
             *   AdrSlot::new(), AdrSlot::new(), AdrSlot::new(), AdrSlot::new()
             * ],
             */
            range_buf:
                unsafe { generate_with0!([ForwardEnt<AdrRange>; 256]) },
        }
    }

    fn _alloc<Type>(
        &mut self,
        slot: &mut AdrSlot,
        layout: Layout,
        forget: bool) -> Option<usize>
    {
        let mut align_gap;
        let mut found: Option<&mut AdrRange> = None;

        for e in slot.free_ranges.iter() {
            if e.bytes < layout.size() {
                continue;
            }
            align_gap = ops::up_align(e.adr, layout.align()) - e.adr;
            if e.bytes - align_gap >= layout.size() {
                found = Some(e);
                break;
            }
        }
        let ent = match found {
            None => return None,
            Some(x) => x,
        };

        if ent.bytes == layout.size() {
            //adr = ent.adr;
            //slot.free_ranges.remove
        }

        //let mut adr;
        //if ent.bytes == layout.size() {
        //    adr = ent.adr;
        //    slot.free_ranges.
        //} else {
        //}

        None
    }

    pub fn alloc<Type>(
        &mut self,
        slotmask: SlotMask,
        layout: Layout,
        forget: bool) -> Result<X<Type>, Error> {

        for (i, slot) in self.slots.iter_mut().enumerate() {
            //write!(log(), "i: {}, start: {}, end: {}", i, slot.start, slot.end);
            if !is_masked(i, slotmask) {
                continue;
            }

            for range in slot.free_ranges.iter() {
                //if bytes < range.bytes
            }

        }

        Err(Error::Fail)

    }

    fn new_adrrange<'s, 't>(&'s mut self)
        -> Result<&'t mut ForwardEnt<AdrRange>, Error>
    {
        let mut r = self.free_buf_list.pop_front();
        match r {
            Some(mut buf) => Ok(buf.as_mut()),
            None => Err(Error::Fail),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2 + 2, 4);
        let mut defs = SlotDefs::new();
        defs.set(0, 0x00000, 0x0ffff);
        defs.set(1, 0x10000, 0x1ffff);

        let mut buf = [0usize; USIZES_IN_CHEAPALLOC];
        let mut ca = CheapAlloc::from(&mut buf);
        ca.init_with_slotdefs(&defs);
        ca.alloc::<[u8; 0x10000]>(0x1 | 0x2, unsafe { Layout::from_size_align_unchecked(0x10000, 8) }, false);
    }
}
