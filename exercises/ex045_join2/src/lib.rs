//! Lesson 11-4: asyncを使うべきか
//!
//! join2 は2つの Future を交互に poll し、両方が終わったら結果の組を返す。
//! スレッドは1つも使わないが、2つの処理が並行に進む（並行 ≠ 並列）。
//!
//! フィールドを Pin<Box<F>> で持つので Join2 自体は Unpin になり、
//! self.get_mut() で中身を触れる（unsafe な pin projection が要らない）。

pub mod runtime;
pub use runtime::block_on;

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

pub struct Join2<A: Future, B: Future> {
    a: Pin<Box<A>>,
    b: Pin<Box<B>>,
    a_out: Option<A::Output>,
    b_out: Option<B::Output>,
}

pub fn join2<A: Future, B: Future>(a: A, b: B) -> Join2<A, B> {
    Join2 {
        a: Box::pin(a),
        b: Box::pin(b),
        a_out: None,
        b_out: None,
    }
}

impl<A: Future, B: Future> Future for Join2<A, B>
where
    A::Output: Unpin,
    B::Output: Unpin,
{
    type Output = (A::Output, B::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        let result: Poll<Self::Output> = todo!(
            "まだ終わっていない方をそれぞれ poll し、結果が出たら a_out / b_out に保存してください。両方そろったら Ready、そうでなければ Pending を返します"
        );
        result
    }
}

/// poll されるたびにラベルを記録し、n 回 Pending を返してから Ready になる Future。
pub struct Steps {
    pub n: u32,
    pub label: &'static str,
    pub log: Arc<Mutex<Vec<&'static str>>>,
}

impl Future for Steps {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        self.log.lock().unwrap().push(self.label);
        if self.n == 0 {
            return Poll::Ready(self.n);
        }
        self.n -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
