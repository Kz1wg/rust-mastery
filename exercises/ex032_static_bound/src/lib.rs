//! Lesson 08-3: elisionと 'static
//!
//! `T: 'static` は「T が 'static でない参照を含まない」という意味。
//! String や i32 のような所有型は満たすが、ローカル変数への参照は満たさない。

/// item を後で文字列化するクロージャを返す。
///
/// 返すクロージャは、いつまで保持されるか分からない（`Box<dyn Fn>` は既定で 'static）。
/// そのため item は、どこかの一時的な借用に縛られていてはいけない:
///
/// ```compile_fail
/// use ex032_static_bound::describe_later;
///
/// let local = String::from("x");
/// let f = describe_later(&local); // &local は 'static を満たさない
/// ```
pub fn describe_later<T: std::fmt::Display + 'static>(item: T) -> Box<dyn Fn() -> String> {
    todo!("item を move で取り込み、呼ばれるたびに item.to_string() を返すクロージャを Box に入れて返してください")
}
