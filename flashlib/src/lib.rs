use std::time::Duration;

/// Identifier for a specific light in the system.
///
/// The system supports five lights, each identified by a unique `LightId`.
/// This enum provides a type-safe way to reference lights.
#[derive(Clone, Copy)]
pub enum LightId {
    /// First light
    One,
    /// Second light
    Two,
    /// Third light
    Three,
    /// Fourth light
    Four,
    /// Fifth light
    Five,
}
impl LightId {
    /// Returns a slice containing all five lights in order.
    pub fn all() -> &'static [LightId; 5] {
        &[
            LightId::One,
            LightId::Two,
            LightId::Three,
            LightId::Four,
            LightId::Five,
        ]
    }
    /// Returns the zero-based index of this light.
    pub fn index(&self) -> usize {
        match self {
            LightId::One => 0,
            LightId::Two => 1,
            LightId::Three => 2,
            LightId::Four => 3,
            LightId::Five => 4,
        }
    }
    /// Returns the total number of lights in the system.
    pub const fn num_lights() -> usize {
        5
    }
}

/// Represents the on/off state of a light.
///
/// This enum provides a type-safe way to represent binary light states.
#[derive(Clone, Copy, Debug)]
pub enum OnOff {
    /// Light is turned on
    On,
    /// Light is turned off
    Off,
}
impl OnOff {
    /// Returns the opposite state.
    pub fn other(&self) -> OnOff {
        match self {
            OnOff::On => OnOff::Off,
            OnOff::Off => OnOff::On,
        }
    }
}

/// Trait for controlling and querying light states.
///
/// This trait abstracts the interface for interacting with lights, allowing
/// different implementations (e.g., hardware, simulation, TUI) to be used
/// interchangeably.
pub trait Lights {
    /// Gets the current state of the specified light.
    fn get_state(&self, light_id: LightId) -> OnOff;

    /// Sets the state of the specified light.
    fn set_state(&self, light_id: LightId, state: OnOff);

    /// Sets all lights to the same state.
    ///
    /// This is a convenience method that calls `set_state` for each light.
    fn set_all_lights(&self, state: OnOff) {
        for light_id in LightId::all() {
            self.set_state(*light_id, state);
        }
    }
}

pub trait TimeAdd: Sized + PartialOrd {
    fn checked_add(&self, duration: Duration) -> Option<Self>;
}

/// Trait for time-related operations, abstracting time sources.
///
/// This trait allows different time implementations (real-time, simulated,
/// test doubles) to be used interchangeably. It provides both synchronous
/// and asynchronous sleep operations.
pub trait TimeSource {
    type AbsoluteTime: TimeAdd + Clone;

    /// Returns the current time as an `Instant`.
    fn now(&self) -> Self::AbsoluteTime;

    /// Sleeps for the specified duration.
    ///
    /// This is a blocking operation that will pause execution for at least
    /// the specified duration.
    fn sleep(&self, duration: Duration);

    /// Sleeps until the specified time.
    ///
    /// This is a blocking operation that will pause execution until the
    /// specified instant is reached.
    fn sleep_until(&self, time: &Self::AbsoluteTime);

    /// Asynchronously sleeps until the specified time.
    ///
    /// This allows non-blocking sleep operations in async contexts.
    fn async_sleep_until(&self, time: &Self::AbsoluteTime) -> impl Future<Output = ()>;
}
