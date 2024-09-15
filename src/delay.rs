use futures::task::{self, ArcWake};
use std::{
    future::Future,
    iter::MapWhile,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::{Duration, Instant},
};

pub struct Delay {
    when: Instant,
    // this is Some when we have spawned a thread and None otherwise
    waker: Option<Arc<Mutex<Waker>>>,
}

impl Delay {
    pub fn new(when: Instant) -> Delay {
        Delay { when, waker: None }
    }
}
impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("poll ready");
            Poll::Ready("done")
        } else {
            // Get a handle to the waker for
            // let waker = cx.waker().clone();
            // let when = self.when;

            // The duration has not elapsed. If this is the first time the future
            // is called, spawn the timer thread. If the timer thread is already
            // running, ensure the stored `Waker` matches the current task's waker.
            if let Some(waker) = &self.waker {
                let mut waker = waker.lock().unwrap();

                // Check if the stored waker matches the current task's waker.
                // This is necessary as the `Delay` future instance may move to
                // a different task between calls to `poll`. If this happens
                // the waker contained by the given `Context` will differ and we
                // must update our stored waker to reflect this change.
                if !waker.will_wake(cx.waker()) {
                    *waker = cx.waker().clone();
                }
            } else {
                let when = self.when;
                let waker = Arc::new(Mutex::new(cx.waker().clone()));

                // Spawn a timer thread.
                thread::spawn(move || {
                    let now = Instant::now();

                    if now < when {
                        thread::sleep(when - now);
                    }

                    waker.lock().unwrap().wake_by_ref();
                    // waker.wake();
                });
            }

            Poll::Pending
        }
    }
}
