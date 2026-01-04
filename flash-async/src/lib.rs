use std::time::Duration;

use flashlib::{LightId, Lights, OnOff, TimeAdd, TimeSource};

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
    flash_duration: Duration,
    lights: &impl Lights,
    time_source: &TS,
) -> anyhow::Result<()> {
    tracing::info!("AsyncFlasher task started");
    let mut next_wakeup_time = now
        .checked_add(flash_duration)
        .ok_or_else(|| anyhow::anyhow!("Overflow"))?;

    loop {
        // Sleep then On
        next_wakeup_time = sleep_until(next_wakeup_time, flash_duration, time_source).await?;
        lights.set_state(light_to_toggle, OnOff::On)?;

        // Sleep then Off
        next_wakeup_time = sleep_until(next_wakeup_time, flash_duration, time_source).await?;
        lights.set_state(light_to_toggle, OnOff::Off)?;
    }
}

/// Sleep until the provided wakeup time.
/// On return, add the duration to the next wakeup time to get the next wakeup time.
async fn sleep_until<TS: TimeSource>(
    wakeup_time: TS::AbsoluteTime,
    duration: Duration,
    time_source: &TS,
) -> anyhow::Result<TS::AbsoluteTime> {
    time_source.async_sleep_until(&wakeup_time).await;
    wakeup_time
        .checked_add(duration)
        .ok_or_else(|| anyhow::anyhow!("Overflow"))
}
