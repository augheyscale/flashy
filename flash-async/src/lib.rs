use std::time::{Duration, Instant};

use flashlib::{LightId, Lights, OnOff, TimeSource};

pub async fn run_async_flasher(
    now: Instant,
    light_to_toggle: LightId,
    sleep_duration: Duration,
    lights: &impl Lights,
    time_source: &impl TimeSource,
) -> anyhow::Result<()> {
    tracing::info!("AsyncFlasher task started");
    let mut next_wakeup_time = now + sleep_duration;

    let set_light_state = |state: OnOff| {
        lights.set_state(light_to_toggle, state);
    };

    loop {
        next_wakeup_time = sleep_for(next_wakeup_time, sleep_duration, time_source).await?;
        set_light_state(OnOff::On);

        next_wakeup_time = sleep_for(next_wakeup_time, sleep_duration, time_source).await?;
        set_light_state(OnOff::Off);
    }
}

/// Sleep for the given duration and return the next wakeup time
async fn sleep_for(
    next_wakeup_time: Instant,
    duration: Duration,
    time_source: &impl TimeSource,
) -> anyhow::Result<Instant> {
    time_source.async_sleep_until(next_wakeup_time).await;
    next_wakeup_time
        .checked_add(duration)
        .ok_or_else(|| anyhow::anyhow!("Overflow"))
}
