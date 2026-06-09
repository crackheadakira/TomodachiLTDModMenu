use crate::{save::player::TimedCatalogInfo, sead::container::FreeList};

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
