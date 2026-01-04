use std::time::{Duration, Instant};
use flashlib::TimeSource;

pub struct TuiTimeSource;

impl TimeSource for TuiTimeSource {
    fn now(&self) -> Instant {
        Instant::now()
    }

    fn sleep(&self, duration: Duration) {
        std::thread::sleep(duration);
    }

    fn sleep_until(&self, time: Instant) {
        std::thread::sleep(time.duration_since(Instant::now()));
    }

    async fn async_sleep_until(&self, time: Instant) {
        tokio::time::sleep_until(time.into()).await;
    }
}

