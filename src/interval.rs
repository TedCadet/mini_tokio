use crate::delay::Delay;
use async_stream::stream;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio_stream::Stream;

struct Interval {
    rem: usize,
    delay: Delay,
}

impl Interval {
    fn new() -> Self {
        Self {
            rem: 3,
            delay: Delay::new(Instant::now()),
        }
    }

    fn create_stream() {
        // equivalent to impl Stream for Interval
        stream! {
            let mut when = Instant::now();
            for _ in 0..3 {
                let delay = Delay::new(when);
                delay.await;
                yield ();
                when += Duration::from_millis(10);
            }
        }
    }
}

impl Stream for Interval {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rem == 0 {
            // No more delays
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when_rc = Arc::clone(&self.delay.when());
                let when_mutex = when_rc.lock().unwrap();
                let when = *when_mutex + Duration::from_millis(10);

                self.delay = Delay::new(when);
                self.rem -= 1;

                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
    // fn size_hint(&self) -> (usize, Option<usize>) {}
}
