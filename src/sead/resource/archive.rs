use crate::sead::resource::resource::DirectResource;

#[repr(C, align(4))]
pub struct ArchiveRes {
    pub base: DirectResource,
    pub enable: bool,
}

const _: () = assert!(core::mem::size_of::<ArchiveRes>() == 0x20);
