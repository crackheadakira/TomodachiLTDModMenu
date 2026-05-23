use super::sead_safe_string::SafeString;

pub struct INamable {
    inamable_name: SafeString,
}

const _: () = assert!(core::mem::size_of::<INamable>() == 0x8);
