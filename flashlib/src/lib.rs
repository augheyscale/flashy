use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub enum LightId {
    One,
    Two,
    Three,
    Four,
    Five,
}
impl LightId {
    pub fn all() -> &'static [LightId; 5] {
        &[
            LightId::One,
            LightId::Two,
            LightId::Three,
            LightId::Four,
            LightId::Five,
        ]
    }
    pub fn index(&self) -> usize {
        match self {
            LightId::One => 0,
            LightId::Two => 1,
            LightId::Three => 2,
            LightId::Four => 3,
            LightId::Five => 4,
        }
    }
    pub const fn num_lights() -> usize {
        5
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OnOff {
    On,
    Off,
}
impl OnOff {
    pub fn other(&self) -> OnOff {
        match self {
            OnOff::On => OnOff::Off,
            OnOff::Off => OnOff::On,
        }
    }
}

pub trait Lights {
    fn get_state(&self, light_id: LightId) -> OnOff;
    fn set_state(&self, light_id: LightId, state: OnOff);
    fn set_all_lights(&self, state: OnOff) {
        for light_id in LightId::all() {
            self.set_state(*light_id, state);
        }
    }
}

pub trait TimeSource {
    fn now(&self) -> Instant;
    fn sleep(&self, duration: Duration);
    fn sleep_until(&self, time: Instant);
    fn async_sleep_until(&self, time: Instant) -> impl Future<Output = ()>;
}
