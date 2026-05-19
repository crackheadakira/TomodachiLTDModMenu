use ruzstd::decoding::StreamingDecoder;
use skyline::nn;
use std::ffi::{c_char, c_void};
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};

unsafe fn read_string(string_ptr: *const u8) -> String {
    if !string_ptr.is_null() {
        let double_ptr = string_ptr as *const *const c_char;

        let path = *double_ptr;

        if !path.is_null() {
            return std::ffi::CStr::from_ptr(path)
                .to_string_lossy()
                .into_owned();
        }
    }

    String::new()
}

#[skyline::hook(offset = 0x639f4c)]
pub fn rstb_parse_hook(param_1: *mut u64, param_2: *const u64) {
    // param_2 points to a struct, *param_2 is the raw RSTB file data pointer
    let rstb_data = unsafe { *param_2 } as *mut u8;

    if rstb_data.is_null() {
        return call_original!(param_1, param_2);
    }

    // check magic - "RESTBL" or "RSTB"
    let magic = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(rstb_data, 6)) };
    println!("[RSTB Parse] magic={magic:}");

    call_original!(param_1, param_2);
}
