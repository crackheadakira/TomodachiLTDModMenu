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
