//! Lesson 15-3: FFI
//!
//! C の標準ライブラリの関数を、依存 crate なしで呼ぶ。
//! 外に見せるのは safe な関数だけにし、C の作法（\0 終端など）は内側に閉じ込める。

use std::ffi::{c_char, c_int, CStr, CString, NulError};

extern "C" {
    fn abs(x: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

/// C の abs を呼ぶ。
/// C の abs は i32::MIN を渡すと未定義動作になるので、その場合は C に渡さず None を返す。
pub fn c_abs(x: i32) -> Option<i32> {
    todo!("i32::MIN なら None、そうでなければ SAFETY コメントを書いて abs を呼び Some で返してください")
}

/// C の strlen で、文字列のバイト数を数える。途中に \0 があれば Err を返す。
pub fn c_strlen(s: &str) -> Result<usize, NulError> {
    todo!("CString::new で変換し（? で Err を返す）、変数に束縛したまま strlen を呼んでください")
}

/// C から rust_add という名前で呼べる関数。
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    // ここだけは todo!() を使っていない。extern "C" の関数の中で panic すると、
    // panic が C の境界を越えられず、プログラム全体がその場で止まる（abort）ため（本文 15-3）。
    // かわりに「まだ書いていない」ことを表す仮の値を返している。a と b の和を返すように書き換えること
    let _ = (a, b);
    i32::MIN
}

/// C の文字列（\0 終端）を Rust の String に変換する。UTF-8 でない部分は置き換え文字にする。
///
/// # Safety
///
/// - `ptr` は null であってはならない
/// - `ptr` は、\0 で終わる有効な C 文字列を指していなければならない
/// - その文字列は、この関数を呼んでいる間、解放されたり書き換えられたりしてはならない
pub unsafe fn from_c_str(ptr: *const c_char) -> String {
    todo!("SAFETY コメントを書いて CStr::from_ptr で読み、to_string_lossy で String にしてください")
}

// CStr を使っていることを骨組みの段階でも示すため（未使用警告を避ける）。
#[allow(dead_code)]
fn _uses_cstr(_: &CStr) {}
