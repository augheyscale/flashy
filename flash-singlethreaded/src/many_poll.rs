use flashlib::{Lights, TimeAdd};

/// A container for multiple `FlashPolled` instances.
///
/// This struct allows managing and polling multiple flashers simultaneously.
/// It's useful when you want to flash multiple lights independently with
/// different timing.
pub struct ManyPoll<AbsoluteTime: TimeAdd> {
    flashers: Vec<crate::FlashPolled<AbsoluteTime>>,
}

impl<AbsoluteTime: TimeAdd> ManyPoll<AbsoluteTime> {
    /// Creates a new `ManyPoll` instance with the specified flashers.
    pub fn new(flashers: Vec<crate::FlashPolled<AbsoluteTime>>) -> Self {
        Self { flashers }
    }

    /// Polls all managed flashers.
    ///
    /// This method calls `poll` on each flasher in sequence, allowing all
    /// lights to be updated in a single call.
    pub fn poll(&mut self, lights: &impl Lights, now: &AbsoluteTime) -> anyhow::Result<()> {
        // Poll each flasher
        for flasher in self.flashers.iter_mut() {
            flasher.poll(lights, now)?;
        }
        Ok(())
    }
}
