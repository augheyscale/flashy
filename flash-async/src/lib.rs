use std::time::Duration;

use flashlib::{LightId, Lights, OnOff, TimeAdd as _, TimeSource};

/// Runs an asynchronous flashing loop.
///
/// This function implements a non-blocking flashing loop that alternates a light
/// between on and off states at regular intervals. It uses `TimeSource::async_sleep_until`
/// to wait for the next toggle time, making it suitable for async/await contexts.
///
/// The function runs indefinitely until an error occurs.
pub async fn run_async_flasher<TS: TimeSource>(
    now: &TS::AbsoluteTime,
    light_to_toggle: LightId,
    sleep_duration: Duration,
    lights: &impl Lights,
    time_source: &TS,
) -> anyhow::Result<()> {
    tracing::info!("AsyncFlasher task started");
    let mut next_wakeup_time = now
        .checked_add(sleep_duration)
        .ok_or_else(|| anyhow::anyhow!("Overflow"))?;

    loop {
        next_wakeup_time = sleep_for(&next_wakeup_time, sleep_duration, time_source).await?;
        lights.set_state(light_to_toggle, OnOff::On);

        next_wakeup_time = sleep_for(&next_wakeup_time, sleep_duration, time_source).await?;
        lights.set_state(light_to_toggle, OnOff::Off);
    }
}

/// Sleep for the given duration and return the next wakeup time
async fn sleep_for<TS: TimeSource>(
    next_wakeup_time: &TS::AbsoluteTime,
    duration: Duration,
    time_source: &TS,
) -> anyhow::Result<TS::AbsoluteTime> {
    time_source.async_sleep_until(next_wakeup_time).await;
    next_wakeup_time
        .checked_add(duration)
        .ok_or_else(|| anyhow::anyhow!("Overflow"))
}
