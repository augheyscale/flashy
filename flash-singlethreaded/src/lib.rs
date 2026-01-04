pub mod many_poll;
pub use many_poll::ManyPoll;
use std::time::Duration;

use flashlib::{LightId, Lights, OnOff, TimeAdd, TimeSource};

/// A polled flasher that toggles a light at regular intervals.
///
/// This struct implements a polling-based approach to flashing lights.
/// It tracks when the next toggle should occur and can be polled repeatedly
/// to check if it's time to toggle the light state.
pub struct FlashPolled<AbsoluteTime: TimeAdd> {
    light_id: LightId,
    next_toggle_time: AbsoluteTime,
    sleep_duration: Duration,
    next_flash_state: OnOff,
}

impl<AbsoluteTime: TimeAdd> FlashPolled<AbsoluteTime> {
    /// Creates a new `FlashPolled` instance.
    pub fn new(light_id: LightId, now: &AbsoluteTime, sleep_duration: Duration) -> Option<Self> {
        Some(Self {
            light_id,
            next_toggle_time: now.checked_add(sleep_duration)?,
            sleep_duration,
            next_flash_state: OnOff::On,
        })
    }

    /// Polls the flasher and toggles the light if it's time.
    ///
    /// This method should be called repeatedly in a loop. It checks if the
    /// current time has reached the next toggle time, and if so, toggles
    /// the light state and schedules the next toggle.
    pub fn poll(&mut self, lights: &impl Lights, now: &AbsoluteTime) -> anyhow::Result<()> {
        if now >= &self.next_toggle_time {
            tracing::debug!("Toggling light state to {:?}", self.next_flash_state);
            lights.set_state(self.light_id, self.next_flash_state);
            self.next_toggle_time = now
                .checked_add(self.sleep_duration)
                .ok_or_else(|| anyhow::anyhow!("Overflow"))?;
            self.next_flash_state = self.next_flash_state.other();
            tracing::info!(
                "Light toggled, next toggle in {:?} seconds",
                self.sleep_duration.as_secs_f32()
            );
        }
        Ok(())
    }
}

/// Runs a flashing loop using sleep-based timing.
///
/// This function implements a blocking flashing loop that alternates a light
/// between on and off states at regular intervals. It uses `TimeSource::sleep_until`
/// to wait for the next toggle time, making it more efficient than polling.
///
/// The function runs indefinitely until an error occurs.
pub fn run_flash_sleep<TS: TimeSource>(
    lights: &impl Lights,
    time_source: &TS,
    start_time: &TS::AbsoluteTime,
    light_to_toggle: LightId,
    sleep_duration: Duration,
) -> anyhow::Result<()> {
    tracing::info!("FlashSleep task started");
    let mut next_wakeup_time = start_time
        .checked_add(sleep_duration)
        .ok_or_else(|| anyhow::anyhow!("Overflow"))?;
    loop {
        next_wakeup_time = sleep_for(&next_wakeup_time, sleep_duration, time_source)?;
        lights.set_state(light_to_toggle, OnOff::On);
        next_wakeup_time = sleep_for(&next_wakeup_time, sleep_duration, time_source)?;
        lights.set_state(light_to_toggle, OnOff::Off);
    }
}

fn sleep_for<TS: TimeSource>(
    next_wakeup_time: &TS::AbsoluteTime,
    duration: Duration,
    time_source: &TS,
) -> anyhow::Result<TS::AbsoluteTime> {
    time_source.sleep_until(next_wakeup_time);
    next_wakeup_time
        .checked_add(duration)
        .ok_or_else(|| anyhow::anyhow!("Overflow"))
}
