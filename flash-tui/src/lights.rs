use std::sync::{Arc, Mutex};
use flashlib::{LightId, Lights, OnOff};

pub struct TuiLights {
    pub(crate) states: Arc<Mutex<[OnOff; LightId::num_lights()]>>,
}

impl TuiLights {
    pub fn new() -> Self {
        Self {
            states: Arc::new(Mutex::new([OnOff::Off; LightId::num_lights()])),
        }
    }
}

impl Lights for TuiLights {
    fn get_state(&self, light_id: LightId) -> OnOff {
        self.states.lock().unwrap()[light_id.index()]
    }

    fn set_state(&self, light_id: LightId, state: OnOff) {
        let mut states = self.states.lock().unwrap();
        states[light_id.index()] = state;
    }
}

