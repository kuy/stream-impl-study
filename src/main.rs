use chrono::Local;
use futures::task::Poll;
use futures::{Stream, StreamExt};
use std::{error::Error, result::Result};

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
            Poll::Ready(None)
        } else {
            let current = Local::now().timestamp_millis();
            if current - self.timestamp < 1000 {
                if self.timestamp == 0 {
                    self.timestamp = current;
                }
                cx.waker().wake_by_ref();
                Poll::Pending
            } else {
                self.count -= 1;
                self.timestamp = current;
                Poll::Ready(Some(self.count))
            }
        }
    }
}
