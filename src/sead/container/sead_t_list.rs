use std::marker::PhantomData;
use std::ptr;

use super::sead_list_impl::{ListImpl, ListNode};

#[repr(C)]
pub struct TList<T> {
    list_impl: ListImpl,
    _marker: PhantomData<T>,
}

impl<T> TList<T> {
    pub fn iter(&self) -> TListIterator<'_, T> {
        let first_node =
            unsafe { ptr::read_unaligned(ptr::addr_of!(self.list_impl.start_end.next)) };

        TListIterator {
            current: first_node,
            list: &self.list_impl,
            _marker: PhantomData,
        }
    }
}

pub struct TListIterator<'a, T> {
    current: *mut ListNode,
    list: &'a ListImpl,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for TListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let sentinel_ptr = ptr::addr_of!(self.list.start_end) as *mut ListNode;

        if self.current == sentinel_ptr {
            None
        } else {
            unsafe {
                let t_node = self.current as *mut TListNode<T>;

                self.current = (*self.current).next();

                Some(&(*t_node).data)
            }
        }
    }
}

#[repr(C)]
pub struct TListNode<T> {
    list_node: ListNode,
    data: T,
    list: *mut TList<T>,
}
