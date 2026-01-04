use flashlib::{LightId, Lights, OnOff};
use std::sync::{Arc, Mutex};

/// TUI implementation of the `Lights` trait.
///
/// This struct provides a thread-safe implementation of light state management
/// for use in the terminal user interface. All lights are initialized to `Off`
/// when created.
///
/// # Thread Safety
///
/// This implementation uses `Arc<Mutex<>>` internally to allow safe concurrent
/// access from multipole threads.
pub struct TuiLights {
    pub(crate) states: Arc<Mutex<[OnOff; LightId::num_lights()]>>,
}

impl Default for TuiLights {
    /// Creates a new `TuiLights` instance with all lights turned off.
    fn default() -> Self {
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
