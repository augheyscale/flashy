use flashlib::TimeSource;
use std::time::{Duration, Instant};

/// TUI implementation of the `TimeSource` trait.
///
/// This struct provides a real-time implementation using the system clock
/// and standard library sleep functions. For async operations, it uses
/// Tokio's time utilities.
pub struct TuiTimeSource;

impl TuiTimeSource {
    /// Creates a new `TuiTimeSource` instance.
    pub fn new() -> Self {
        TuiTimeSource
    }
}

#[derive(PartialEq, Clone)]
pub struct WrappedInstant(Instant);

impl flashlib::TimeAdd for WrappedInstant {
    fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.0.checked_add(duration).map(WrappedInstant)
    }
}

impl PartialOrd for WrappedInstant {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl TimeSource for TuiTimeSource {
    type AbsoluteTime = WrappedInstant;

    fn now(&self) -> Self::AbsoluteTime {
        WrappedInstant(Instant::now())
    }

    fn sleep(&self, duration: Duration) {
        std::thread::sleep(duration);
    }

    fn sleep_until(&self, time: &Self::AbsoluteTime) {
        std::thread::sleep(time.0.duration_since(Instant::now()));
    }

    async fn async_sleep_until(&self, time: &Self::AbsoluteTime) {
        tokio::time::sleep_until(time.0.into()).await;
    }
}
