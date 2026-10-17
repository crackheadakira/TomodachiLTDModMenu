use crate::sead::container::{FreeList, PtrArray};
use std::mem::{ManuallyDrop, MaybeUninit};

#[repr(C)]
pub struct ObjArray<T> {
    pub base: PtrArray<T>,
    pub free_list: FreeList,
}

#[repr(C)]
pub struct FixedObjArray<T, const N: usize> {
    pub base: ObjArray<T>,
    work: ObjArrayWork<T, N>,
}

#[repr(C)]
pub struct ObjArrayWork<T, const N: usize> {
    slots: [MaybeUninit<ObjArraySlot<T>>; N],
    pointers: [MaybeUninit<*mut T>; N],
}

#[repr(C)]
pub union ObjArraySlot<T> {
    next_free: *mut ObjArraySlot<T>,
    value: ManuallyDrop<T>,
}
