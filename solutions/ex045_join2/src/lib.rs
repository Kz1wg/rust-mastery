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

        if me.a_out.is_none() {
            if let Poll::Ready(value) = me.a.as_mut().poll(cx) {
                me.a_out = Some(value);
            }
        }
        if me.b_out.is_none() {
            if let Poll::Ready(value) = me.b.as_mut().poll(cx) {
                me.b_out = Some(value);
            }
        }

        if me.a_out.is_some() && me.b_out.is_some() {
            Poll::Ready((me.a_out.take().unwrap(), me.b_out.take().unwrap()))
        } else {
            Poll::Pending
        }
    }
}

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
