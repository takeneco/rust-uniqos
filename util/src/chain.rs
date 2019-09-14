
use core::cell::Cell;
use core::marker::PhantomData;
use core::ops::Deref;

pub unsafe trait Pointer<T: ?Sized>: Deref<Target = T> + Sized {
    fn into_ptr(self) -> *const T;
    unsafe fn from_ptr(ptr: *const T) -> Self;
}

unsafe impl<'a, T: ?Sized> Pointer<T> for &'a T {
    fn into_ptr(self) -> *const T {
        self
    }
    unsafe fn from_ptr(ptr: *const T) -> Self {
        &*ptr
    }
}

unsafe impl<T: ?Sized> Pointer<T> for Box<T> {
    fn into_ptr(self) -> *const T {
        Box::into_raw(self)
    }
    unsafe fn from_ptr(ptr: *const T) -> Self {
        Box::from_raw(ptr as *mut T)
    }
}

pub struct Link();

trait ListNode {
    fn set_next(e: *const Link);
}

/// Linked list pointer inserted into a object.
pub struct ForwardNode {
    next: Cell<*const Link>,
}

impl ForwardNode {
    pub fn new() -> Self {
        Self { next: Cell::new(core::ptr::null()) }
    }
}

impl Clone for ForwardNode {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Default for ForwardNode {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Node {
    next: Cell<*const Link>,
    prev: Cell<*const Link>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            next: Cell::new(core::ptr::null()),
            prev: Cell::new(core::ptr::null()),
        }
    }
}

pub trait AdapterBase {}
pub trait ForwardAdapter<Type, Ref: Deref<Target=Type>> : AdapterBase =
    Fn(&Ref) -> &ForwardNode;
pub trait Adapter<Type, Ref: Deref<Target=Type>> : AdapterBase =
    Fn(&Ref) -> *const Node;

pub trait End {
    fn new() -> Self;
    fn get_head(&self) -> *const Link;
    fn set_head(&mut self, e: *const Link);
    fn set_tail(&mut self, e: *const Link);
}

/// List end inserted into ListImpl.
pub struct SingleEnd {
    pub head: *const Link,
}

impl End for SingleEnd {
    fn new() -> Self {
        Self {
            head: core::ptr::null(),
        }
    }
    fn get_head(&self) -> *const Link {
        self.head
    }
    fn set_head(&mut self, e: *const Link) {
        self.head = e;
    }
    fn set_tail(&mut self, _: *const Link) {
        // nothing to do.
    }
}

/// List end inserterd into ListImpl for double ended list.
pub struct DualEnd {
    h: SingleEnd,
    tail: *const Link,
}

impl End for DualEnd {
    fn new() -> Self {
        Self {
            h: SingleEnd::new(),
            tail: core::ptr::null(),
        }
    }
    fn get_head(&self) -> *const Link {
        self.h.head
    }
    fn set_head(&mut self, e: *const Link) {
        self.h.head = e;
    }
    fn set_tail(&mut self, e: *const Link) {
        self.tail = e;
    }
}

impl DualEnd {
    fn get_tail(&self) -> *const Link {
        self.tail
    }
}

pub trait Impl<Type, Ref, E: End, H>
    where Ref: Deref<Target=Type>,
          H: ForwardAdapter<Type, Ref> {
    fn iter(&self) -> ListIter<Type, Ref, E, H>;
    fn next(&self, e: *const Link) -> *const Link;
}

/// List implementation.
pub struct ListImpl<Type, Ref, E, Hook> {
    pub end: E,
    hook: Hook,

    _phantom1: PhantomData<Type>,
    _phantom2: PhantomData<Ref>,
}

impl<Type, Ref, E: End, H>
    ListImpl<Type, Ref, E, H>
    where
        Ref: Pointer<Type>,
        H: ForwardAdapter<Type, Ref> {

    pub fn new(hook: H) -> Self {
        Self {
            end: E::new(),
            hook: hook,

            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn push_front(&mut self, e: Ref) {
        {
            let node = (self.hook)(&e) as &ForwardNode;
            node.next.set(self.end.get_head());
        }
        let ep = e.into_ptr() as *const Link;

        if self.end.get_head().is_null() {
            self.end.set_tail(ep);
        }
        self.end.set_head(ep);
    }
    pub fn pop_front(&mut self) -> Ref {
        let ret_raw = self.end.get_head();
        let ret;
        unsafe {
            ret = Pointer::from_ptr(ret_raw as *const Type);
            let ret_node = (self.hook)(&ret);
            self.end.set_head((*ret_node).next.get());
        }
        ret
    }
}

impl<Type, Ref, H>
    ListImpl<Type, Ref, DualEnd, H>
    where
        Ref: Pointer<Type>,
        H: ForwardAdapter<Type, Ref> {
}

impl<'a, Type, Ref, E: End, H>
    IntoIterator
    for &'a mut ListImpl<Type, Ref, E, H>
    where Ref: Pointer<Type> + 'a,
          H: ForwardAdapter<Type, Ref> {

    type Item = &'a Type;
    type IntoIter = ListIter<'a, Type, Ref, E, H>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            list: self,
            cur: self.end.get_head(),
        }
    }
}

impl<'t, Type, Ref, E: End, H: ForwardAdapter<Type, Ref>>
    Impl<Type, Ref, E, H>
    for ListImpl<Type, Ref, E, H>
    where Ref: Pointer<Type> + 't {

    fn iter(&self) -> ListIter<Type, Ref, E, H> {
        ListIter {
            list: self,
            cur: self.end.get_head(),
        }
    }

    fn next(&self, e: *const Link) -> *const Link {
//      if e.is_null() {
//          e
//      }
//      else {
            let hook = &self.hook;
            let ret;
            unsafe {
                let elem_ref = Pointer::from_ptr(e as *const Type);
                {
                let node = hook(&elem_ref);
                ret = (*node).next.get();
                }
                Pointer::into_ptr(elem_ref);
            }
            ret
//      }
    }
}

/// List iterator.
pub struct ListIter<'l, Type, Ref: Deref, End, Adapter> {
    list: &'l ListImpl<Type, Ref, End, Adapter>,
    cur: *const Link,
}

impl<'t, Type, Ref, E: End, H>
    core::iter::Iterator
    for ListIter<'t, Type, Ref, E, H>
    where
        Ref: Pointer<Type>,
        H: ForwardAdapter<Type, Ref> {

    type Item = &'t Type;

    fn next(&mut self) -> Option<&'t Type> {
        let ret_raw = self.cur as *const Type;;
        if !ret_raw.is_null() {
            self.cur = self.list.next(self.cur);
            unsafe { Option::Some(&*ret_raw) }
        } else {
            Option::None
        }
    }
}

pub type SingleForwardList<Type, Ref, NodeHook> =
    ListImpl<Type, Ref, SingleEnd, NodeHook>;

pub type ForwardList<Type, Ref, NodeHook> =
    ListImpl<Type, Ref, DualEnd, NodeHook>;

pub type SingleList<Type, Ref, NodeHook> =
    ListImpl<Type, Ref, SingleEnd, NodeHook>;

pub type List<Type, Ref, NodeHook> =
    ListImpl<Type, Ref, DualEnd, NodeHook>;

