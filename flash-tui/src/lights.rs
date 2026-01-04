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
    fn get_state(&self, light_id: LightId) -> anyhow::Result<OnOff> {
        Ok(*self
            .states
            .lock()
            .unwrap()
            .get(light_id.index())
            .ok_or_else(|| anyhow::anyhow!("Light index out of bounds"))?)
    }
    fn set_state(&self, light_id: LightId, state: OnOff) -> anyhow::Result<()> {
        let mut states = self.states.lock().unwrap();
        *states
            .get_mut(light_id.index())
            .ok_or_else(|| anyhow::anyhow!("Light index out of bounds"))? = state;
        Ok(())
    }
}
