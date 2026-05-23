use skyline::nn;

use crate::sead::heap::IDisposer;

#[repr(C)]
pub struct CriticalSection {
    pub base: IDisposer,
    pub critical_section_inner: nn::os::MutexType,
}

const _: () = assert!(core::mem::size_of::<CriticalSection>() == 0x40);
