use native_vtable::vtable;

use super::heap::Heap;
use crate::sead::container::ListNode;

#[vtable(path = sead::heap::VftIDisposer)]
pub unsafe trait VftIDisposer {
    #[vtable(raw_this)]
    unsafe fn destructor(this: *mut Self);

    #[vtable(raw_this)]
    unsafe fn deleting_destructor(this: *mut Self);
}

#[repr(C)]
pub struct IDisposer {
    pub vtable: *const VftIDisposerLayout,

    pub disposer_heap: *mut Heap,
    pub list_node: ListNode,
}

const _: () = assert!(core::mem::size_of::<IDisposer>() == 0x20);
