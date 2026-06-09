use std::ffi::{c_char, CStr};
use std::fmt;

pub type SafeString = SafeStringBase<u8>;
pub type WSafeString = SafeStringBase<u16>;

pub type BufferedSafeString = BufferedSafeStringBase<u8>;
pub type WBufferedSafeString = BufferedSafeStringBase<u16>;

pub type FixedSafeString32 = FixedSafeStringBase<u8, 32>;
pub type FixedSafeString64 = FixedSafeStringBase<u8, 64>;
pub type WFixedSafeString32 = FixedSafeStringBase<u16, 32>;
pub type WFixedSafeString64 = FixedSafeStringBase<u16, 64>;

#[repr(C)]
pub struct SafeStringBase<T> {
    string_top: *const T,
}

#[repr(C)]
pub struct BufferedSafeStringBase<T> {
    base: SafeStringBase<T>,
    buffer_size: i32,
}

#[repr(C)]
pub struct FixedSafeStringBase<T, const L: usize> {
    base: BufferedSafeStringBase<T>,
    buffer: [T; L],
}

unsafe fn read_utf8_ptr(ptr: *const u8) -> String {
    if ptr.is_null() {
        return String::from("NULL");
    }

    CStr::from_ptr(ptr as *const c_char)
        .to_string_lossy()
        .into_owned()
}

unsafe fn read_utf16_ptr(ptr: *const u16) -> String {
    if ptr.is_null() {
        return String::from("NULL");
    }

    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    let slice = core::slice::from_raw_parts(ptr, len);
    String::from_utf16_lossy(slice)
}

impl fmt::Debug for SafeStringBase<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = unsafe { read_utf8_ptr(self.string_top) };
        fmt::Debug::fmt(&s, f)
    }
}

impl fmt::Debug for SafeStringBase<u16> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = unsafe { read_utf16_ptr(self.string_top) };
        fmt::Debug::fmt(&s, f)
    }
}

impl<T> fmt::Debug for BufferedSafeStringBase<T>
where
    SafeStringBase<T>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.base, f)
    }
}

impl<T, const L: usize> fmt::Debug for FixedSafeStringBase<T, L>
where
    BufferedSafeStringBase<T>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.base, f)
    }
}
