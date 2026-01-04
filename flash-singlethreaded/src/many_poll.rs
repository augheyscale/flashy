use std::time::{Duration, Instant};

use flashlib::{LightId, Lights, OnOff, TimeSource};

struct Flasher {
    sleep_duration: Duration,
    light_id: LightId,
}

pub struct ManyPoll {
    flashers: Vec<(Flasher, Instant, OnOff)>,
}

pub struct ManyPollBuilder {
    flashers: Vec<Flasher>,
}

impl ManyPollBuilder {
    pub fn new() -> Self {
        Self {
            flashers: Vec::new(),
        }
    }

    pub fn add_flasher(&mut self, sleep_duration: Duration, light_id: LightId) {
        self.flashers.push(Flasher {
            sleep_duration,
            light_id,
        });
    }

    pub fn init(self, start_time: Instant) -> ManyPoll {
        ManyPoll {
            flashers: self
                .flashers
                .into_iter()
                .map(|flasher| {
                    let next_flash_time = start_time + flasher.sleep_duration;
                    (flasher, next_flash_time, OnOff::On)
                })
                .collect(),
        }
    }
}

impl ManyPoll {
    pub fn poll(&mut self, lights: &impl Lights, time_source: &impl TimeSource) {
        let now = time_source.now();
        // See which flasher has expired
        for (flasher, next_flash_time, next_flash_state) in self.flashers.iter_mut() {
            if now >= *next_flash_time {
                lights.set_state(flasher.light_id, *next_flash_state);
                *next_flash_state = next_flash_state.other();
                *next_flash_time = *next_flash_time + flasher.sleep_duration;
            }
        }
    }
}
