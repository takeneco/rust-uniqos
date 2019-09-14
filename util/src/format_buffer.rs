// Uniqos  --  Unique Operating System
// (c) 2019 KATO Takeshi
// Released under the MIT license

use core::mem;

struct FormatBufferProp {
    buf_offset: usize,
}

pub struct FormatBuffer {
    prop: FormatBufferProp,
    buf: [u8; 128 - mem::size_of::<FormatBufferProp>()],
}
