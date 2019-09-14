// Uniqos  --  Unique Operating System
// (c) 2019 KATO Takeshi
// Released under the MIT license

/// Misc operations.

/// `align` must be a power of two.
pub fn down_align(adr: usize, align: usize) -> usize {
    adr & !(align - 1)
}
pub fn up_align(adr: usize, align: usize) -> usize {
    (adr + align - 1) & !(align - 1)
}


