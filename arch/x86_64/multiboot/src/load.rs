// Uniqos  --  Unique Operating System
// (c) 2019 KATO Takeshi
// Released under the MIT license

use core::fmt::Write;

use util::error::Error;
use super::heap;
use super::log::log;

#[cfg(feature = "boot_multiboot2")]
extern crate multiboot2;


const MULTIBOOT2_BOOTLOADER_MAGIC: u32 = 0x36d76289;

fn load_boot_none(_: u32, _: *const u32) -> Result<(), Error> {
    Err(Error::Fail)
}

/// Detect multiboot2 protocol and load if succeeded.
#[cfg(feature = "boot_multiboot2")]
fn load_mb2(magic: u32, tag: *const u32) -> Result<(), Error> {
    if magic != MULTIBOOT2_BOOTLOADER_MAGIC {
        return Err(Error::Fail);
    }

    let mb2_tags = unsafe { multiboot2::load(tag as usize) };

    if let Some(mmap_tag) = mb2_tags.memory_map_tag() {
        for mm in mmap_tag.memory_areas() {
            write!(log(), "{:?}\n", mm).unwrap();
        }
    }

    Ok(())
}

#[cfg(not(feature = "boot_multiboot2"))]
const load_mb2: fn(u32, *const u32) -> Result<(), Error> = load_boot_none;

const load_mb: fn(u32, *const u32) -> Result<(), Error> = load_boot_none;

fn load_bootprotocol(magic: u32, tag: *const u32) -> Result<(), Error> {
    if cfg!(feature = "boot_multiboot2") && load_mb2(magic, tag).is_ok() {
        return Ok(());
    } else if load_mb(magic, tag).is_ok() {
        return Ok(());
    }
    Err(Error::Fail)
}

#[no_mangle]
pub extern "C" fn load(magic: u32, tag: *const u32) -> u32 {
    heap::init();

    let r = load_bootprotocol(magic, tag);
    match r {
        Ok(()) => {
            0
        },
        Err(e) => {
            write!(log(), "No boot protocols detected.");
            1
        },
    }
}
