use std::time::{Instant, Duration};
use std::intrinsics::nontemporal_store;

pub fn nanos_per_frame(fps: &f64) -> Duration {
    Duration::from_secs( (1000000000f64 / fps ) as u64)
}

/// Defines the timing for when a function should be called next.
pub enum Timing {

    /// Call immediately at the rate of `check()`.
    ASAP,

    /// Called at a certain framerate.
    Framerate{last_called_at: Instant, desired_framerate: f64},

    /// Wait a specific `Duration`
    SpecificTime{last_called_at: Instant, desired_wait_time: Duration },

    /// This will not call the function.
    Never,
}

impl Timing {

    /// Resets the timer.
    pub fn reset(&mut self) {
        match self {
            Timing::ASAP => {}
            Timing::Framerate { last_called_at, desired_framerate } => {
                *last_called_at = Instant::now();
            }
            Timing::SpecificTime { last_called_at, desired_wait_time } => {
                *last_called_at = Instant::now();
            }
            Timing::Never => {}
        }
    }

    /// Returns true if ready to call.
    pub fn check(&self) -> bool {
        match self {
            Timing::ASAP => true,
            Timing::Framerate { last_called_at, desired_framerate } => (Instant::now().duration_since(last_called_at.clone()) >= nanos_per_frame(desired_framerate)),
            Timing::SpecificTime { last_called_at, desired_wait_time } => (Instant::now().duration_since(last_called_at.clone()) >= desired_wait_time),
            Timing::Never => false
        }
    }
}