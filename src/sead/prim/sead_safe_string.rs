pub type SafeString = SafeStringBase<u8>;
pub type WSafeString = SafeStringBase<u16>;

pub type BufferedSafeString = BufferedSafeStringBase<u8>;
pub type WBufferedSafeString = BufferedSafeStringBase<u16>;

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
