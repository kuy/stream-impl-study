use chrono::Local;
use futures::{task::Poll, Stream, StreamExt};
use std::{error::Error, result::Result, thread, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut timer = CountDownTimer::new();
    while let Some(remain) = timer.next().await {
        println!("{}", remain);
    }

    Ok(())
}

struct CountDownTimer {
    count: usize,
    timestamp: i64,
}

impl CountDownTimer {
    fn new() -> Self {
        CountDownTimer {
            count: 10,
            timestamp: 0,
        }
    }
}

impl Stream for CountDownTimer {
    type Item = usize;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.count == 0 {
            println!("poll_next: done");
            Poll::Ready(None)
        } else {
            let current = Local::now().timestamp_millis();
            if self.timestamp == 0 {
                self.timestamp = current;
            }

            if current - self.timestamp < 1000 {
                let waker = cx.waker().clone();
                thread::spawn(move || {
                    println!("poll_next: sleep");
                    thread::sleep(Duration::from_millis(1000));

                    println!("poll_next: wake");
                    waker.wake();
                });

                println!("poll_next: pending");
                Poll::Pending
            } else {
                self.count -= 1;
                self.timestamp = current;

                println!("poll_next: ready");
                Poll::Ready(Some(self.count))
            }
        }
    }
}
