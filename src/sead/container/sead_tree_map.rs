use crate::{save::player::TimedCatalogInfo, sead::container::FreeList};
use std::fmt;

#[repr(C)]
pub struct TreeMapNode<K, T> {
    pub left: *mut TreeMapNode<K, T>,
    pub right: *mut TreeMapNode<K, T>,
    pub parent_color: *mut TreeMapNode<K, T>,
    pub key: K,
    pub data: T,
}

#[repr(C)]
pub struct FixedTreeMap<K, T, const S: usize> {
    pub root: *mut TreeMapNode<K, T>,
    pub free_list: FreeList,
    pub capacity: i32,
    pub size: i32,
    pub storage: [TreeMapNode<K, T>; S],
}

const _: () = assert!(core::mem::size_of::<TreeMapNode<u32, TimedCatalogInfo>>() == 0x50);
const _: () = assert!(core::mem::size_of::<FixedTreeMap<u32, TimedCatalogInfo, 0x200>>() == 0xA020);

impl<K: fmt::Debug, T: fmt::Debug> fmt::Debug for TreeMapNode<K, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TreeMapNode")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("parent_color", &self.parent_color)
            .field("key", &self.key)
            .field("data", &self.data)
            .finish()
    }
}
impl<K: fmt::Debug, T: fmt::Debug, const S: usize> fmt::Debug for FixedTreeMap<K, T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FixedTreeMap")
            .field("size", &self.size)
            .field("capacity", &self.capacity)
            .field("root_ptr", &self.root)
            .field("storage_buffer_slots", &S)
            .finish()
    }
}
