use std::ffi::{c_char, c_int, CStr, CString, NulError};

extern "C" {
    fn abs(x: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

pub fn c_abs(x: i32) -> Option<i32> {
    if x == i32::MIN {
        return None;
    }
    // SAFETY: x は i32::MIN ではないので、C の abs の未定義動作の条件に当たらない。
    Some(unsafe { abs(x) })
}

pub fn c_strlen(s: &str) -> Result<usize, NulError> {
    let c = CString::new(s)?;
    // SAFETY: c は \0 終端の有効な C 文字列で、この呼び出しの間 c は生きている。
    Ok(unsafe { strlen(c.as_ptr()) })
}

#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

/// # Safety
///
/// - `ptr` は null であってはならない
/// - `ptr` は、\0 で終わる有効な C 文字列を指していなければならない
/// - その文字列は、この関数を呼んでいる間、解放されたり書き換えられたりしてはならない
pub unsafe fn from_c_str(ptr: *const c_char) -> String {
    // SAFETY: 呼び出し側が上の Safety 節の条件を守っている前提。
    unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned()
}

#[allow(dead_code)]
fn _uses_cstr(_: &CStr) {}
