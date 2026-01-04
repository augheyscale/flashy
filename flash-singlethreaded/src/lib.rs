pub mod many_poll;
pub use many_poll::{ManyPoll, ManyPollBuilder};
use std::time::{Duration, Instant};

use flashlib::{LightId, Lights, OnOff, TimeSource};

pub struct FlashPolled {
    next_toggle_time: Instant,
    next_flash_state: OnOff,
}

impl FlashPolled {
    pub fn new(time_source: &impl TimeSource) -> Self {
        Self {
            next_toggle_time: time_source.now() + Duration::from_secs(1),
            next_flash_state: OnOff::On,
        }
    }

    pub fn poll(&mut self, lights: &impl Lights, time_source: &impl TimeSource) {
        let now = time_source.now();
        if now >= self.next_toggle_time {
            tracing::debug!("Toggling light state to {:?}", self.next_flash_state);
            lights.set_state(LightId::One, self.next_flash_state);
            self.next_toggle_time = now + Duration::from_secs(1);
            self.next_flash_state = self.next_flash_state.other();
            tracing::info!("Light toggled, next toggle in 1 second");
        }
    }
}

pub struct FlashSleep {
    light_to_toggle: LightId,
    sleep_duration: Duration,
}
impl FlashSleep {
    pub fn new(light_to_toggle: LightId, sleep_duration: Duration) -> Self {
        Self {
            light_to_toggle,
            sleep_duration,
        }
    }
}

impl FlashSleep {
    pub fn run(
        &mut self,
        lights: &impl Lights,
        time_source: &impl TimeSource,
        start_time: Instant,
    ) {
        let mut next_flash_state = OnOff::On;
        tracing::info!("FlashSleep task started");
        let mut next_wakeup_time = start_time + self.sleep_duration;
        loop {
            time_source.sleep_until(next_wakeup_time);
            next_wakeup_time = next_wakeup_time + self.sleep_duration;
            lights.set_state(self.light_to_toggle, next_flash_state);
            next_flash_state = next_flash_state.other();
        }
    }
}
