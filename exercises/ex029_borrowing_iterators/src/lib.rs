//! Lesson 07-4: 借用するiteratorを返す
//!
//! a と b の両方から借用するので、両方に同じlifetime 'a を明示する必要がある
//! （引数が2つあるため、lifetime省略規則だけでは決まらない）。
//! ここでは「両方を連結し、threshold を超える要素だけを残す」処理にしている。

pub fn merged_over<'a>(
    a: &'a [i32],
    b: &'a [i32],
    threshold: i32,
) -> impl Iterator<Item = &'a i32> {
    a.iter()
        .chain(b.iter())
        .filter(move |&&x| todo!("x が threshold を超えるかどうかを返してください"))
}
