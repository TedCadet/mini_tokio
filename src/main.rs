mod delay;
mod interval;
mod main_future;
mod my_task;

use std::time::{Duration, Instant};

use delay::Delay;
use mini_tokio::MiniTokio;

fn main() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay::new(when);

        let out = future.await;
        assert_eq!(out, "done");
    });

    mini_tokio.run();
}
