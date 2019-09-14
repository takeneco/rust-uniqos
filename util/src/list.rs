// Uniqos  --  Unique Operating System
// (c) 2019 KATO Takeshi
// Released under the MIT license
//
/// Original linked list.

#[cfg(not(feature = "nobox"))]
extern crate alloc;

#[cfg(not(feature = "nobox"))]
use alloc::boxed::Box;

use core::convert::Into;
use core::fmt;
use core::marker::PhantomData;
use core::ops::DerefMut;
use core::ops::Deref;
use core::ptr::NonNull;


pub unsafe trait Refer<T: ?Sized>: DerefMut<Target = T> + Sized {
    fn into_ptr(self) -> *const T;
    unsafe fn from_ptr(ptr: *const T) -> Self;
}

unsafe impl<'a, T: ?Sized> Refer<T> for &'a mut T {
    fn into_ptr(self) -> *const T {
        self
    }
    unsafe fn from_ptr(ptr: *const T) -> Self {
        &mut *(ptr as *mut T)
    }
}

#[cfg(not(feature = "nobox"))]
unsafe impl<T: ?Sized> Refer<T> for Box<T> {
    fn into_ptr(self) -> *const T {
        Box::into_raw(self)
    }
    unsafe fn from_ptr(ptr: *const T) -> Self {
        Box::from_raw(ptr as *mut T)
    }
}

pub struct RawRefer<T: ?Sized> {
    ptr: NonNull<T>,
}
impl<T: ?Sized> RawRefer<T> {
    pub fn as_mut<'t>(&mut self) -> &'t mut T {
        unsafe { &mut *self.ptr.as_ptr() }
    }
}

impl<T: ?Sized> Deref for RawRefer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}
impl<T: ?Sized> DerefMut for RawRefer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}
unsafe impl<T: ?Sized> Refer<T> for RawRefer<T> {
    fn into_ptr(self) -> *const T {
        self.ptr.as_ptr()
    }
    unsafe fn from_ptr(ptr: *const T) -> Self {
        Self {
            ptr: NonNull::<T>::new_unchecked(ptr as *mut T)
        }
    }
}


pub struct Link;

// Node trait
pub trait NodeTrait {
    fn get_next(&self) -> *const Link;
    fn set_prev(&mut self, prev: *const Link);
    fn set_next(&mut self, next: *const Link);
}

// ForwardNode

pub struct ForwardNode {
    next: *const Link,
}

impl ForwardNode {
    pub fn new() -> Self {
        Self {
            next: core::ptr::null(),
        }
    }
}

impl Clone for ForwardNode {
    fn clone(&self) -> Self {
        ForwardNode::new()
    }
}

impl Copy for ForwardNode {}

impl fmt::Debug for ForwardNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{{ next: {:p} }}", self.next))
    }
}

impl NodeTrait for ForwardNode {
    fn get_next(&self) -> *const Link {
        self.next
    }
    fn set_prev(&mut self, _: *const Link) {
        // Nothing to do
    }
    fn set_next(&mut self, ent: *const Link) {
        self.next = ent;
    }
}

// Node
/// Bidirectional.

pub struct Node {
    next: *const Link,
    prev: *const Link,
}

impl Node {
    pub fn new() -> Self {
        Self {
            next: core::ptr::null(),
            prev: core::ptr::null(),
        }
    }
    pub fn get_prev(&self) -> *const Link {
        self.prev
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node::new()
    }
}

impl Copy for Node {}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{{ prev: {:p}, next: {:p} }}", self.prev, self.next))
    }
}

impl NodeTrait for Node {
    fn get_next(&self) -> *const Link {
        self.next
    }
    fn set_prev(&mut self, prev: *const Link) {
        self.prev = prev;
    }
    fn set_next(&mut self, next: *const Link) {
        self.next = next;
    }
}

// End trait

pub trait EndTrait {
    fn new() -> Self;
    fn get_head(&self) -> *const Link;
    fn get_tail_or_null(&self) -> *const Link;
    fn set_head(&mut self, ent: *const Link);
    fn set_tail(&mut self, ent: *const Link);
}

// SignelEnd

pub struct SingleEnd {
    head: *const Link,
}

impl EndTrait for SingleEnd {
    fn new() -> Self {
        Self {
            head: core::ptr::null()
        }
    }
    fn get_head(&self) -> *const Link {
        self.head
    }
    fn get_tail_or_null(&self) -> *const Link {
        core::ptr::null()
    }
    fn set_head(&mut self, head: *const Link) {
        self.head = head;
    }
    fn set_tail(&mut self, _: *const Link) {
        // nothing todo
    }
}

// DualEnd

pub struct DualEnd {
    head: *const Link,
    tail: *const Link,
}

impl EndTrait for DualEnd {
    fn new() -> Self {
        Self {
            head: core::ptr::null(),
            tail: core::ptr::null(),
        }
    }
    fn get_head(&self) -> *const Link {
        self.head
    }
    fn get_tail_or_null(&self) -> *const Link {
        self.tail
    }
    fn set_head(&mut self, head: *const Link) {
        self.head = head;
    }
    fn set_tail(&mut self, tail: *const Link) {
        self.tail = tail;
    }
}

pub trait ListAdapter {
    fn ref_node(&self) -> &Node;
}

// ListImpl

pub struct ListImpl<Type, Ref, Nod, End, Adp> 
    where
        Ref: Refer<Type>,
        Nod: NodeTrait,
        End: EndTrait,
        Adp: ListAdapter<NodeType = Nod> {
    end: End,

    _phantom1: PhantomData<Type>,
    _phantom2: PhantomData<Ref>,
    _phantom3: PhantomData<Nod>,
}
/*
impl<Type, Ref, Nod, End, Adp> Clone for ListImpl<Type, Ref, Nod, End, Adp>
    where
        Ref: Refer<Type>,
        Nod: NodeTrait,
        End: EndTrait,
        Adp: ListAdapter<NodeType = Nod> {

    fn clone(&self) -> Self {
        new()
    }
}
*/

/// implement for any ListImpl includes SingleForwardList.
impl<Type, Ref, Nod, End, Adp> ListImpl<Type, Ref, Nod, End, Adp>
    where
        Ref: Refer<Type>,
        Nod: NodeTrait,
        End: EndTrait,
        Adp: ListAdapter<NodeType = Nod> {

    pub fn new() -> Self {
        Self {
            end: End::new(),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
            _phantom3: PhantomData,
        }
    }

    fn _ref_ent<'t>(ent: *const Link) -> &'t Type {
        unsafe { &*(ent as *const Type) }
    }

    fn _mut_ent<'t>(ent: *const Link) -> &'t mut Type {
        unsafe { &mut *(ent as *mut Type) }
    }

    fn _ref_ent_or_none<'t>(ent: *const Link) -> Option<&'t Type> {
        unsafe { (ent as *const Type).as_ref() }
    }

    fn _mut_ent_or_none<'t>(ent: *const Link) -> Option<&'t mut Type> {
        unsafe { (ent as *mut Type).as_mut() }
    }

    pub fn ref_front(&self) -> Option<&Type> {
        Self::_ref_ent_or_none(self.end.get_head())
    }

    pub fn mut_front(&mut self) -> Option<&mut Type> {
        Self::_mut_ent_or_none(self.end.get_head())
    }

    pub fn ref_next(&self, base: &Type) -> Option<&Type> {
        Self::_ref_ent_or_none(Adp::ref_node(base).get_next())
    }

    pub fn push_front(&mut self, mut ent: Ref) {
        let old_head = self.end.get_head();
        Adp::ref_node(&ent).set_prev(core::ptr::null());
        Adp::ref_node(&ent).set_next(old_head);
        let new_head = ent.into_ptr() as *const Link;

        self.end.set_head(new_head);
        match Self::_mut_ent_or_null(old_head) {
            Some(old_head) => Adp::ref_node(old_head).set_prev(new_head),
            None => self.end.set_tail(new_head),
        }
    }

/*
    pub fn pop_front(&mut self) -> Option<Ref> {
        let old_head = self.end.get_head();
        if !old_head.is_null() {
            let new_head = Self::_mut_ent(old_head).get_next();
            self.end.set_head(new_head);
            if !new_head.is_null() {
                Self::_mut_ent(new_head).set_prev(core::ptr::null());
            } else {
                self.end.set_tail(core::ptr::null());
            }
            Some(unsafe { Refer::from_ptr(old_head as *const Entry) })
        } else {
            None
        }
    }

    pub fn insert_after(&mut self, base: &mut Entry, mut new_ent: Ref) {
        let prev = base;
        let next = Self::_ref_ent_or_none(prev.get_next());

        (*new_ent).set_prev(prev as *const Entry as *const Link);
        (*new_ent).set_next(prev.get_next());
        let new_ent = new_ent.into_ptr() as *const Link;
        prev.set_next(new_ent);
        match next {
            Some(n) => n.set_prev(new_ent),
            None => self.end.set_tail(new_ent),
        }
    }

    pub fn remove_next(&mut self, base: &mut Entry) -> Option<Ref>
    {
        match Self::_ref_ent_or_none(base.get_next()) {
            Some(next1) => {
                let next2 = next1.get_next();
                base.set_next(next2);
                let base_ptr = base as *const Entry as *const Link;
                match Self::_ref_ent_or_none(next2) {
                    Some(next2) => {
                        next2.set_prev(base_ptr);
                    }, 
                    None => {
                        self.end.set_tail(base_ptr)
                    },
                };
                Some(unsafe { Refer::from_ptr(next1) })
            },
            None => { None },
        }
    }
*/

/*
    pub fn iter(&mut self) -> ListIter<'e, Type, Ref, End, Entry> {
        ListIter::<Type, Ref, End, Entry> {
            next: self.end.get_head(),
            next_back: self.end.get_tail_or_null(),

            _phantom1: PhantomData,
            _phantom2: PhantomData,
            _phantom3: PhantomData,
            _phantom4: PhantomData,
        }
    }
*/
}

/*
/// implement for ForwardList or List.
impl<'e, Type, Ref, Entry> ListImpl<Type, Ref, DualEnd, Entry>
    where
        Ref: Refer<Entry>,
        Entry: EntBase<Type> + 'e {

    pub fn get_back(&self) -> Option<&'e mut Entry> {
        let ret_raw = self.end.get_tail_or_null() as *mut Entry;
        unsafe { ret_raw.as_mut() }
    }

    pub fn push_back(&mut self, mut ent: Ref) {
        let old_tail = self.end.get_tail_or_null();
        (*ent).set_prev(old_tail);
        (*ent).set_next(core::ptr::null());
        let new_tail = ent.into_ptr() as *const Link;

        self.end.set_tail(new_tail);
        if !old_tail.is_null() {
            Self::_mut_ent(old_tail).set_next(new_tail);
        } else {
            self.end.set_head(new_tail);
        }
    }
}
*/

/*
/// implement for SingleList or List.
impl<'e, Type, Ref, End> ListImpl<Type, Ref, End, Ent<Type>>
    where
        Ref: Refer<Ent<Type>>,
        End: EndBase {

    pub fn get_prev(base: &Ent<Type>) -> Option<&'e mut Ent<Type>> {
        let ret_raw = base.get_prev() as *mut Ent<Type>;
        unsafe { ret_raw.as_mut() }
    }

    pub fn insert_before(&mut self, base: &mut Ent<Type>, mut new_ent: Ref) {
        let prev = Self::_ref_ent_or_none(base.get_prev());
        let next = base;

        (*new_ent).set_prev(next.get_prev());
        (*new_ent).set_next(next as *const Ent<Type> as *const Link);
        let new_ent = new_ent.into_ptr() as *const Link;
        next.set_prev(new_ent);
        match prev {
            Some(p) => p.set_next(new_ent),
            None => self.end.set_head(new_ent),
        }
    }

    pub fn remove(&mut self, target: &Ent<Type>) -> Ref {
        let prev = target.get_prev();
        let next = target.get_next();

        match Self::_ref_ent_or_none(prev) {
            Some(x) => x.set_next(next),
            None => self.end.set_head(next),
        };

        match Self::_ref_ent_or_none(next) {
            Some(x) => x.set_prev(prev),
            None => self.end.set_tail(prev),
        }

        unsafe { Refer::from_ptr(target) }
    }
}
*/

/*
/// implement for List.
impl<'e, Type, Ref> ListImpl<Type, Ref, DualEnd, Ent<Type>>
    where Ref: Refer<Ent<Type>> {
    pub fn pop_back(&mut self) -> Option<Ref> {
        let old_tail = self.end.get_tail_or_null();
        if !old_tail.is_null() {
            let new_tail = Self::_mut_ent(old_tail).get_prev();
            self.end.set_tail(new_tail);
            if !new_tail.is_null() {
                Self::_mut_ent(new_tail).set_next(core::ptr::null());
            } else {
                self.end.set_head(core::ptr::null());
            }
            Some(unsafe { Refer::from_ptr(old_tail as *const Ent<Type>) })
        } else {
            None
        }
    }
}
*/

pub type List<Type, Ref, Adapter> =
    ListImpl<Type, Ref, Node, DualEnd, Adapter>;
pub type SingleList<Type, Ref, Adapter> =
    ListImpl<Type, Ref, Node, SingleEnd, Adapter>;
pub type ForwardList<Type, Ref, Adapter> =
    ListImpl<Type, Ref, ForwardNode, DualEnd, Adapter>;
pub type SingleForwardList<Type, Ref, Adapter> =
    ListImpl<Type, Ref, ForwardNode, SingleEnd, Adapter>;
/*
// Iterator

pub struct ListIter<'e, Type, Ref, End, Entry> 
    where
        Ref: Refer<Entry>,
        End: EndBase,
        Entry: EntBase<Type> {
    next: *const Link,
    next_back: *const Link,

    _phantom1: PhantomData<&'e Type>,
    _phantom2: PhantomData<Ref>,
    _phantom3: PhantomData<End>,
    _phantom4: PhantomData<Entry>,
}

impl<'e, Type, Ref, End, Entry> Iterator for ListIter<'e, Type, Ref, End, Entry>
    where
        Ref: Refer<Entry>,
        End: EndBase,
        Entry: EntBase<Type> + 'e {
    type Item = &'e mut Type;

    fn next(&mut self) -> Option<&'e mut Type> {
        if self.next.is_null() {
            None
        } else {
            let ret_ent =
                ListImpl::<Type, Ref, End, Entry>::_mut_ent(self.next);
            if self.next == self.next_back {
                self.next = core::ptr::null();
                self.next_back = core::ptr::null();
            } else {
                self.next = ret_ent.get_next();
            }
            Some(ret_ent.ref_elem_mut())
        }
    }
}

impl<'e, Type, Ref> DoubleEndedIterator
    for ListIter<'e, Type, Ref, DualEnd, Ent<Type>>
    where
        Ref: Refer<Ent<Type>>,
        Ent<Type>: 'e {

    fn next_back(&mut self) -> Option<&'e mut Type> {
        if self.next_back.is_null() {
            None
        } else {
            let ret_ent = ListImpl::<Type, Ref, DualEnd, Ent<Type>>::_mut_ent(
                self.next_back);
            if self.next_back == self.next {
                self.next = core::ptr::null();
                self.next_back = core::ptr::null();
            } else {
                self.next_back = ret_ent.get_prev();
            }
            Some(ret_ent.ref_elem_mut())
        }
    }
}
*/


#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Data {
        listnode: Node,
        data: isize,
    }
    impl ListAdapter for Data {
        fn ref_node(&self) -> &Node {
            &self.Node
        }
    }

    #[test]
    fn test() {
        let mut list = SingleForwardList::<Data, &mut Data, Data>::new();
    }

/*
    fn test_pop_front<Ref>(
        list: &mut List<Data, Ref>,
        val1: Ref, val2: Ref, val3: Ref, val4: Ref)
    where
        Ref: Refer<Ent<Data>> + AsRef<Ent<Data>>,
    {
        let ref1 = unsafe { &*(val1.as_ref() as *const Ent<Data>) };
        let ref2 = unsafe { &*(val2.as_ref() as *const Ent<Data>) };
        let ref3 = unsafe { &*(val3.as_ref() as *const Ent<Data>) };
        let ref4 = unsafe { &*(val4.as_ref() as *const Ent<Data>) };

        assert!(list.get_front().is_none());
        assert!(list.get_back().is_none());
        assert!(list.pop_front().is_none());
        list.push_front(val1);
        // list: 1
        assert_eq!(list.get_front().unwrap(), ref1);
        assert_eq!(list.get_back().unwrap(), ref1);
        list.push_front(val2);
        // list: 2, 1
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref1);
        list.push_back(val3);
        // list: 2, 1, 3
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref3);
        list.push_back(val4);
        // list: 2, 1, 3, 4
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref4);

        let tmp = list.pop_front().unwrap();
        // list: 1, 3, 4, tmp: 2
        assert_eq!(tmp.as_ref(), ref2);
        assert_eq!(list.get_front().unwrap(), ref1);
        assert_eq!(list.get_back().unwrap(), ref4);
        list.insert_after(list.get_front().unwrap(), tmp);
        // list: 1, 2, 3, 4
        assert_eq!(list.get_front().unwrap(), ref1);
        assert_eq!(list.get_back().unwrap(), ref4);
        for (x, y) in (1..).zip(list.iter()) {
            assert_eq!(x, y.data);
        }
        let tmp = list.pop_front().unwrap();
        // list: 2, 3, 4, tmp: 1
        assert_eq!(tmp.as_ref(), ref1);
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref4);
        list.insert_after(list.get_back().unwrap(), tmp);
        // list: 2, 3, 4, 1
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref1);
        let mut iter = list.iter().rev();
        for x in [1, 4, 3, 2].iter() {
            assert_eq!(iter.next(), Some(&mut Data { data: *x }));
        }
        let tmp = list.pop_front().unwrap();
        // list: 3, 4, 1, tmp: 2
        assert_eq!(tmp.as_ref(), ref2);
        assert_eq!(list.get_front().unwrap(), ref3);
        assert_eq!(list.get_back().unwrap(), ref1);
        list.insert_before(list.get_front().unwrap(), tmp);
        // list: 2, 3, 4, 1
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref1);
        let tmp = list.pop_front().unwrap();
        // list: 3, 4, 1, tmp: 2
        assert_eq!(tmp.as_ref(), ref2);
        assert_eq!(list.get_front().unwrap(), ref3);
        assert_eq!(list.get_back().unwrap(), ref1);
        list.insert_before(list.get_back().unwrap(), tmp);
        // list: 3, 4, 2, 1
        assert_eq!(list.get_front().unwrap(), ref3);
        assert_eq!(list.get_back().unwrap(), ref1);
        let tmp = list.pop_front().unwrap();
        // list: 4, 2, 1, tmp: 3
        assert_eq!(tmp.as_ref(), ref3);
        assert_eq!(list.get_front().unwrap(), ref4);
        assert_eq!(list.get_back().unwrap(), ref1);
        let tmp = list.pop_front().unwrap();
        // list: 2, 1, tmp: 4
        assert_eq!(tmp.as_ref(), ref4);
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref1);
        let tmp = list.pop_front().unwrap();
        // list: 1, tmp: 2
        assert_eq!(tmp.as_ref(), ref2);
        assert_eq!(list.get_front().unwrap(), ref1);
        assert_eq!(list.get_back().unwrap(), ref1);
        let tmp = list.pop_front().unwrap();
        // list is empty, tmp: 1
        assert_eq!(tmp.as_ref(), ref1);
        assert!(list.get_front().is_none());
        assert!(list.get_back().is_none());
        assert!(list.pop_front().is_none());
    }

    fn test_s_pop_front<Ref>(
        list: &mut SingleList<Data, Ref>,
        val1: Ref, val2: Ref)
    where
        Ref: Refer<Ent<Data>> + AsRef<Ent<Data>>,
    {
        let ref1 = unsafe { &*(val1.as_ref() as *const Ent<Data>) };
        let ref2 = unsafe { &*(val2.as_ref() as *const Ent<Data>) };

        assert!(list.get_front().is_none());
        assert!(list.pop_front().is_none());
        list.push_front(val1);
        // list: 1
        assert_eq!(list.get_front().unwrap(), ref1);
        list.push_front(val2);
        // list: 2, 1
        assert_eq!(list.get_front().unwrap(), ref2);
        let tmp = list.pop_front().unwrap();
        // list: 1, tmp: 2
        assert_eq!(tmp.as_ref(), ref2);
        assert_eq!(list.get_front().unwrap(), ref1);
        list.insert_after(list.get_front().unwrap(), tmp);
        // list: 1, 2
        assert_eq!(list.get_front().unwrap(), ref1);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&mut Data { data: 1 }));
        assert_eq!(iter.next(), Some(&mut Data { data: 2 }));
        assert_eq!(iter.next(), None);
        let tmp = list.pop_front().unwrap();
        // list: 2, tmp: 1
        assert_eq!(tmp.as_ref(), ref1);
        assert_eq!(list.get_front().unwrap(), ref2);
        list.insert_before(list.get_front().unwrap(), tmp);
        // list: 1, 2
        assert_eq!(list.get_front().unwrap(), ref1);
        let tmp = list.pop_front().unwrap();
        // list: 2, tmp: 1
        assert_eq!(tmp.as_ref(), ref1);
        assert_eq!(list.get_front().unwrap(), ref2);
        let tmp = list.pop_front().unwrap();
        // list is empty: tmp: 2
        assert_eq!(tmp.as_ref(), ref2);
        assert!(list.get_front().is_none());
        assert!(list.pop_front().is_none());
    }

    fn test_f_pop_front<Ref>(
        list: &mut ForwardList<Data, Ref>,
        val1: Ref, val2: Ref, val3: Ref, val4: Ref)
    where
        Ref: Refer<ForwardEnt<Data>> + AsRef<ForwardEnt<Data>>,
    {
        let ref1 = unsafe { &*(val1.as_ref() as *const ForwardEnt<Data>) };
        let ref2 = unsafe { &*(val2.as_ref() as *const ForwardEnt<Data>) };
        let ref3 = unsafe { &*(val3.as_ref() as *const ForwardEnt<Data>) };
        let ref4 = unsafe { &*(val4.as_ref() as *const ForwardEnt<Data>) };

        assert!(list.get_front().is_none());
        assert!(list.get_back().is_none());
        assert!(list.pop_front().is_none());
        list.push_front(val1);
        // list: 1
        assert_eq!(list.get_front().unwrap(), ref1);
        assert_eq!(list.get_back().unwrap(), ref1);
        list.push_front(val2);
        // list: 2, 1
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref1);
        list.push_back(val3);
        // list: 2, 1, 3
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref3);
        list.push_back(val4);
        // list: 2, 1, 3, 4
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref4);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&mut Data { data: 2 }));
        assert_eq!(iter.next(), Some(&mut Data { data: 1 }));
        assert_eq!(iter.next(), Some(&mut Data { data: 3 }));
        assert_eq!(iter.next(), Some(&mut Data { data: 4 }));
        assert_eq!(iter.next(), None);

        let tmp = list.pop_front().unwrap();
        // list: 1, 3, 4, tmp: 2
        assert_eq!(tmp.as_ref(), ref2);
        assert_eq!(list.get_front().unwrap(), ref1);
        assert_eq!(list.get_back().unwrap(), ref4);
        list.insert_after(list.get_front().unwrap(), tmp);
        // list: 1, 2, 3, 4
        assert_eq!(list.get_front().unwrap(), ref1);
        assert_eq!(list.get_back().unwrap(), ref4);
        let tmp = list.pop_front().unwrap();
        // list: 2, 3, 4, tmp: 1
        assert_eq!(tmp.as_ref(), ref1);
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref4);
        list.insert_after(list.get_back().unwrap(), tmp);
        // list: 2, 3, 4, 1
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref1);
        let tmp = list.pop_front().unwrap();
        // list: 3, 4, 1, tmp: 2
        assert_eq!(tmp.as_ref(), ref2);
        assert_eq!(list.get_front().unwrap(), ref3);
        assert_eq!(list.get_back().unwrap(), ref1);
        let mut iter = list.iter();
        for x in [3, 4, 1].iter() {
            assert_eq!(iter.next(), Some(&mut Data { data: *x }));
        }
        let tmp = list.pop_front().unwrap();
        // list: 4, 1, tmp: 3
        assert_eq!(tmp.as_ref(), ref3);
        assert_eq!(list.get_front().unwrap(), ref4);
        assert_eq!(list.get_back().unwrap(), ref1);
        let tmp = list.pop_front().unwrap();
        // list: 1, tmp: 4
        assert_eq!(tmp.as_ref(), ref4);
        assert_eq!(list.get_front().unwrap(), ref1);
        assert_eq!(list.get_back().unwrap(), ref1);
        let tmp = list.pop_front().unwrap();
        // list is empty, tmp: 1
        assert_eq!(tmp.as_ref(), ref1);
        assert!(list.get_front().is_none());
        assert!(list.get_back().is_none());
        assert!(list.pop_front().is_none());
    }

    fn test_sf_pop_front<Ref>(
        list: &mut SingleForwardList<Data, Ref>,
        val1: Ref, val2: Ref)
    where
        Ref: Refer<ForwardEnt<Data>> + AsRef<ForwardEnt<Data>>,
    {
        let ref1 = unsafe { &*(val1.as_ref() as *const ForwardEnt<Data>) };
        let ref2 = unsafe { &*(val2.as_ref() as *const ForwardEnt<Data>) };

        assert!(list.get_front().is_none());
        assert!(list.pop_front().is_none());
        list.push_front(val1);
        // list: 1
        assert_eq!(list.get_front().unwrap(), ref1);
        list.push_front(val2);
        // list: 2, 1
        assert_eq!(list.get_front().unwrap(), ref2);

        let tmp = list.pop_front().unwrap();
        // list: 1, tmp: 2
        assert_eq!(tmp.as_ref(), ref2);
        assert_eq!(list.get_front().unwrap(), ref1);
        list.insert_after(list.get_front().unwrap(), tmp);
        // list: 1, 2
        assert_eq!(list.get_front().unwrap(), ref1);
        let tmp = list.pop_front().unwrap();
        // list: 2, tmp: 1
        assert_eq!(tmp.as_ref(), ref1);
        assert_eq!(list.get_front().unwrap(), ref2);
        let tmp = list.pop_front().unwrap();
        // list is empty, tmp: 2
        assert_eq!(tmp.as_ref(), ref2);
        assert!(list.get_front().is_none());
        assert!(list.pop_front().is_none());
    }

    fn test_pop_back<Ref>(
        list: &mut List<Data, Ref>,
        val1: Ref, val2: Ref, val3: Ref, val4: Ref)
    where
        Ref: Refer<Ent<Data>> + AsRef<Ent<Data>>,
    {
        let ref1 = unsafe { &*(val1.as_ref() as *const Ent<Data>) };
        let ref2 = unsafe { &*(val2.as_ref() as *const Ent<Data>) };
        let ref3 = unsafe { &*(val3.as_ref() as *const Ent<Data>) };
        let ref4 = unsafe { &*(val4.as_ref() as *const Ent<Data>) };

        assert!(list.get_front().is_none());
        assert!(list.get_back().is_none());
        assert!(list.pop_back().is_none());
        list.push_front(val1);
        // list: 1
        assert_eq!(list.get_front().unwrap(), ref1);
        assert_eq!(list.get_back().unwrap(), ref1);
        list.push_front(val2);
        // list: 2, 1
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref1);
        list.push_back(val3);
        // list: 2, 1, 3
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref3);
        list.push_back(val4);
        // list: 2, 1, 3, 4
        for (x, y) in [2, 1, 3, 4, -1].iter().zip(list.iter()) {
            assert_eq!(*x, y.data);
        }
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref4);

        let tmp = list.pop_back().unwrap();
        // list: 2, 1, 3, tmp: 4
        assert_eq!(tmp.as_ref(), ref4);
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref3);
        list.insert_after(list.get_front().unwrap(), tmp);
        // list: 2, 4, 1, 3
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref3);
        let tmp = list.pop_back().unwrap();
        // list: 2, 4, 1, tmp: 3
        assert_eq!(tmp.as_ref(), ref3);
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref1);
        list.insert_before(list.get_back().unwrap(), tmp);
        // list: 2, 4, 3, 1
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref1);
        let tmp = list.pop_back().unwrap();
        // list: 2, 4, 3, tmp: 1
        assert_eq!(tmp.as_ref(), ref1);
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref3);
        let tmp = list.pop_back().unwrap();
        // list: 2, 4, tmp: 3
        assert_eq!(tmp.as_ref(), ref3);
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref4);
        let tmp = list.pop_back().unwrap();
        // list: 2, tmp: 4
        assert_eq!(tmp.as_ref(), ref4);
        assert_eq!(list.get_front().unwrap(), ref2);
        assert_eq!(list.get_back().unwrap(), ref2);
        let tmp = list.pop_back().unwrap();
        // list is empty, tmp: 2
        assert_eq!(tmp.as_ref(), ref2);
        assert!(list.get_front().is_none());
        assert!(list.get_back().is_none());
        assert!(list.pop_back().is_none());
    }

    fn test_remove<Ref>(
        list: &mut List<Data, Ref>,
        val1: Ref, val2: Ref, val3: Ref, val4: Ref)
    where
        Ref: Refer<Ent<Data>> + AsRef<Ent<Data>>,
    {
        let ref1 = unsafe { &*(val1.as_ref() as *const Ent<Data>) };
        let ref2 = unsafe { &*(val2.as_ref() as *const Ent<Data>) };
        let ref3 = unsafe { &*(val3.as_ref() as *const Ent<Data>) };
        let ref4 = unsafe { &*(val4.as_ref() as *const Ent<Data>) };

        list.push_front(val4);
        list.push_front(val3);
        list.push_front(val2);
        list.push_front(val1);
        for (x, y) in [1, 2, 3, 4, -1].iter().zip(list.iter()) {
            assert_eq!(*x, y.data);
        }

        let tmp = list.remove(ref2);
        assert_eq!(tmp.as_ref(), ref2);
        for (x, y) in [1, 3, 4, -1].iter().zip(list.iter()) {
            assert_eq!(*x, y.data);
        }

        let tmp = list.remove(ref1);
        assert_eq!(tmp.as_ref(), ref1);
        for (x, y) in [3, 4, -1].iter().zip(list.iter()) {
            assert_eq!(*x, y.data);
        }

        let tmp = list.remove(ref4);
        assert_eq!(tmp.as_ref(), ref4);
        for (x, y) in [3, -1].iter().zip(list.iter()) {
            assert_eq!(*x, y.data);
        }

        let tmp = list.remove(ref3);
        assert_eq!(tmp.as_ref(), ref3);
        assert_eq!(list.get_front(), None);
    }

    #[test]
    fn test_ref() {
        let mut list = List::<Data, &mut Ent<Data>>::new();

        let mut val1 = Ent::new(Data { data: 1 });
        let mut val2 = Ent::new(Data { data: 2 });
        let mut val3 = Ent::new(Data { data: 3 });
        let mut val4 = Ent::new(Data { data: 4 });
        test_pop_front(
            &mut list, &mut val1, &mut val2, &mut val3, &mut val4);

        let mut val1 = Ent::new(Data { data: 1 });
        let mut val2 = Ent::new(Data { data: 2 });
        let mut val3 = Ent::new(Data { data: 3 });
        let mut val4 = Ent::new(Data { data: 4 });
        test_pop_back(
            &mut list, &mut val1, &mut val2, &mut val3, &mut val4);

        let mut val1 = Ent::new(Data { data: 1 });
        let mut val2 = Ent::new(Data { data: 2 });
        let mut val3 = Ent::new(Data { data: 3 });
        let mut val4 = Ent::new(Data { data: 4 });
        test_remove(
            &mut list, &mut val1, &mut val2, &mut val3, &mut val4);
    }

    #[test]
    fn test_box() {
        let mut list = List::<Data, Box<Ent<Data>>>::new();

        let val1 = Box::new(Ent::new(Data { data: 1 }));
        let val2 = Box::new(Ent::new(Data { data: 2 }));
        let val3 = Box::new(Ent::new(Data { data: 3 }));
        let val4 = Box::new(Ent::new(Data { data: 4 }));
        test_pop_front(&mut list, val1, val2, val3, val4);

        let val1 = Box::new(Ent::new(Data { data: 1 }));
        let val2 = Box::new(Ent::new(Data { data: 2 }));
        let val3 = Box::new(Ent::new(Data { data: 3 }));
        let val4 = Box::new(Ent::new(Data { data: 4 }));
        test_pop_back(&mut list, val1, val2, val3, val4);

        let val1 = Box::new(Ent::new(Data { data: 1 }));
        let val2 = Box::new(Ent::new(Data { data: 2 }));
        let val3 = Box::new(Ent::new(Data { data: 3 }));
        let val4 = Box::new(Ent::new(Data { data: 4 }));
        test_remove(&mut list, val1, val2, val3, val4);
    }

    #[test]
    fn test_raw() {
        let mut list = List::<Data, RawRefer<Ent<Data>>>::new();

        let mut val = [
            Ent::new(Data { data: 1 }),
            Ent::new(Data { data: 2 }),
            Ent::new(Data { data: 3 }),
            Ent::new(Data { data: 4 }),
        ];

        test_pop_front(
            &mut list,
            (&mut val[0]).into(),
            (&mut val[1]).into(),
            (&mut val[2]).into(),
            (&mut val[3]).into());

        test_pop_back(
            &mut list,
            (&mut val[0]).into(),
            (&mut val[1]).into(),
            (&mut val[2]).into(),
            (&mut val[3]).into());

        test_remove(
            &mut list,
            (&mut val[0]).into(),
            (&mut val[1]).into(),
            (&mut val[2]).into(),
            (&mut val[3]).into());
    }

    #[test]
    fn test_s_ref() {
        let mut list = SingleList::<Data, &mut Ent<Data>>::new();
        let mut val1 = Ent::new(Data { data: 1 });
        let mut val2 = Ent::new(Data { data: 2 });

        test_s_pop_front(&mut list, &mut val1, &mut val2);
    }

    #[test]
    fn test_s_box() {
        let mut list = SingleList::<Data, Box<Ent<Data>>>::new();
        let val1 = Box::new(Ent::new(Data { data: 1 }));
        let val2 = Box::new(Ent::new(Data { data: 2 }));

        test_s_pop_front(&mut list, val1, val2);
    }

    #[test]
    fn test_s_raw() {
        let mut list = SingleList::<Data, RawRefer<Ent<Data>>>::new();
        let mut val = [
            Ent::new(Data { data: 1 }),
            Ent::new(Data { data: 2})
        ];

        test_s_pop_front(&mut list, (&mut val[0]).into(), (&mut val[1]).into());
    }

    #[test]
    fn test_f_ref() {
        let mut list = ForwardList::<Data, &mut ForwardEnt<Data>>::new();
        let mut val1 = ForwardEnt::new(Data { data: 1 });
        let mut val2 = ForwardEnt::new(Data { data: 2 });
        let mut val3 = ForwardEnt::new(Data { data: 3 });
        let mut val4 = ForwardEnt::new(Data { data: 4 });

        test_f_pop_front(&mut list, &mut val1, &mut val2, &mut val3, &mut val4);
    }

    #[test]
    fn test_f_box() {
        let mut list = ForwardList::<Data, Box<ForwardEnt<Data>>>::new();
        let val1 = Box::new(ForwardEnt::new(Data { data: 1 }));
        let val2 = Box::new(ForwardEnt::new(Data { data: 2 }));
        let val3 = Box::new(ForwardEnt::new(Data { data: 3 }));
        let val4 = Box::new(ForwardEnt::new(Data { data: 4 }));

        test_f_pop_front(&mut list, val1, val2, val3, val4);
    }

    #[test]
    fn test_f_raw() {
        let mut list = ForwardList::<Data, RawRefer<ForwardEnt<Data>>>::new();
        let mut val = [
            ForwardEnt::new(Data { data: 1 }),
            ForwardEnt::new(Data { data: 2 }),
            ForwardEnt::new(Data { data: 3 }),
            ForwardEnt::new(Data { data: 4 })
        ];

        test_f_pop_front(
            &mut list,
            (&mut val[0]).into(),
            (&mut val[1]).into(),
            (&mut val[2]).into(),
            (&mut val[3]).into());
    }

    #[test]
    fn test_sf_ref() {
        let mut list = SingleForwardList::<Data, &mut ForwardEnt<Data>>::new();
        let mut val1 = ForwardEnt::new(Data { data: 1 });
        let mut val2 = ForwardEnt::new(Data { data: 2 });

        test_sf_pop_front(&mut list, &mut val1, &mut val2);
    }

    #[test]
    fn test_sf_box() {
        let mut list = SingleForwardList::<Data, Box<ForwardEnt<Data>>>::new();
        let val1 = Box::new(ForwardEnt::new(Data { data: 1 }));
        let val2 = Box::new(ForwardEnt::new(Data { data: 2 }));

        test_sf_pop_front(&mut list, val1, val2);
    }

    #[test]
    fn test_sf_raw() {
        let mut list =
            SingleForwardList::<Data, RawRefer<ForwardEnt<Data>>>::new();
        let mut val = [
            ForwardEnt::new(Data { data: 1 }),
            ForwardEnt::new(Data { data: 2 })
        ];

        test_sf_pop_front(
            &mut list, (&mut val[0]).into(), (&mut val[1]).into());
    }
*/
}
