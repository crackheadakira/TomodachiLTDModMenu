use std::ffi::c_void;

use crate::sead::{container::Buffer, resource::archive::ArchiveRes};

#[repr(C)]
pub struct SharcArchive {
    pub archive: *const SharcArchiveRes,
}

#[repr(C)]
pub struct SharcArchiveRes {
    pub base: ArchiveRes,
    pub archive_block_header: *const SarcArchiveBlockHeader,
    pub fat_block_header: *const c_void,
    pub fnt_block: *const c_void,
    pub fat_entries: Buffer<SarcFatEntry>,
    pub data_block: *const u8,
    pub endian_type: i32,
}

#[repr(C)]
pub struct SarcArchiveBlockHeader {
    pub signature: [u8; 4],
    pub header_size: u16,
    pub byte_order: u16,
    pub file_size: u32,
    pub data_block_offset: u32,
    pub version: u16,
    pub reserved: u16,
}

#[repr(C)]
pub struct SarcFatEntry {
    pub hash: u32,
    pub name_offset: u32,
    pub data_start_offset: u32,
    pub data_end_offset: u32,
}

const _: () = assert!(core::mem::size_of::<SharcArchive>() == 0x8);
const _: () = assert!(core::mem::size_of::<SharcArchiveRes>() == 0x58);
const _: () = assert!(core::mem::size_of::<SarcArchiveBlockHeader>() == 0x14);
const _: () = assert!(core::mem::size_of::<SarcFatEntry>() == 0x10);
