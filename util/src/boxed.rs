// Uniqos  --  Unique Operating System
// (c) 2019 KATO Takeshi
// Released under the MIT license

/// Original heap allocation implement instead of alloc::boxed::Box.

use core::ops;


pub struct X<T: ?Sized>(*mut T);

impl<T> ops::Deref for X<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.0 }
    }
}

impl<T> ops::DerefMut for X<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0 }
    }
}

