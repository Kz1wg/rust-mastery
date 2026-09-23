//! 最小のランタイム（Lesson 11-1 と同じもの。依存crateなし）。
use std::future::Future;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Wake, Waker};

struct Signal {
    woken: Mutex<bool>,
    cv: Condvar,
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        *self.woken.lock().unwrap() = true;
        self.cv.notify_one();
    }
}

/// Future を完了まで実行する。
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = Box::pin(future);
    let signal = Arc::new(Signal {
        woken: Mutex::new(false),
        cv: Condvar::new(),
    });
    let waker = Waker::from(Arc::clone(&signal));
    let mut cx = Context::from_waker(&waker);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => {
                let mut woken = signal.woken.lock().unwrap();
                while !*woken {
                    woken = signal.cv.wait(woken).unwrap();
                }
                *woken = false;
            }
        }
    }
}
