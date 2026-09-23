//! ```compile_fail
//! let x = 5;
//! let p = &x as *const i32;
//! let _ = *p; // unsafe ブロックの外では参照外しできない
//! ```

pub fn first_via_ptr(slice: &[i32]) -> Option<i32> {
    if slice.is_empty() {
        return None;
    }
    let ptr = slice.as_ptr();
    // SAFETY: slice は空でないことを確認済みなので、ptr は有効な先頭要素を指している。
    // slice はこの関数の間ずっと借用されており、解放も書き換えもされない。
    Some(unsafe { *ptr })
}

/// # Safety
///
/// - `p` は null であってはならない
/// - `p` は、有効な（初期化済みで、まだ生きている）`i32` を指していなければならない
pub unsafe fn read_value(p: *const i32) -> i32 {
    // SAFETY: 呼び出し側が上の Safety 節の条件を守っている前提。
    unsafe { *p }
}

pub fn swap_values(a: &mut i32, b: &mut i32) {
    std::mem::swap(a, b);
}
