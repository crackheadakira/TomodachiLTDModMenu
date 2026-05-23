use std::ffi::c_void;

use super::heap::Heap;
use crate::sead::container::ListNode;

#[repr(C)]
pub struct IDisposer<V = c_void> {
    pub vtable: *const V,

    pub disposer_heap: *mut Heap,
    pub list_node: ListNode,
}

const _: () = assert!(core::mem::size_of::<IDisposer>() == 0x20);
