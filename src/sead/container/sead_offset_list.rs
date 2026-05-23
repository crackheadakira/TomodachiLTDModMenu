use std::marker::PhantomData;

use crate::sead::container::ListImpl;

pub struct OffsetList<T> {
    base: ListImpl,
    offset: i32,
    _marker: PhantomData<T>,
}

const _: () = assert!(core::mem::size_of::<OffsetList<usize>>() == 0x18);
