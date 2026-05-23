use std::ffi::c_void;

use crate::sead::container::{ListNode, OffsetList};
use crate::sead::heap::IDisposer;
use crate::sead::prim::INamable;
use crate::sead::thread::CriticalSection;

#[repr(i32)]
pub enum HeapDirection {
    CHeapDirectionForward = 1,
    CHeapDirectionReverse = -1,
}

#[repr(C, packed(4))]
pub struct Heap {
    idisposer_base: IDisposer,
    inamable_base: INamable,
    // hostio::reflexible is behind SEAD_DEBUG
    start: *const c_void,
    size: usize,
    parent: *mut Heap,
    children: OffsetList<Heap>,
    list_node: ListNode,
    disposer_list: OffsetList<IDisposer>,
    direction: HeapDirection,
    pad_84: u32,
    cs: CriticalSection,
    flag: u16, // BitFlag<u16>
    heap_check_tag: u16,
}

const _: () = assert!(core::mem::size_of::<Heap>() == 0xCC);
