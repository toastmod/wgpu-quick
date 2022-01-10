use std::time::{Instant, Duration};

pub enum TimerStatus {
    Ready,
    Waiting(Duration),
    Ignore,
}

pub fn nanos_per_frame(fps: &f64) -> Duration {
    Duration::from_nanos( (1000000000f64 / fps ) as u64)
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

    /// Check the wait time left.
    pub fn check(&self) -> TimerStatus {
        match self {
            Timing::ASAP => TimerStatus::Ready,
            Timing::Framerate { last_called_at, desired_framerate } => {
                let elapsed = last_called_at.elapsed();
                let delta_per_frame = nanos_per_frame(desired_framerate);

                if (elapsed >= delta_per_frame){
                    TimerStatus::Ready
                }else{
                    TimerStatus::Waiting(delta_per_frame-elapsed)
                }
            },
            Timing::SpecificTime { last_called_at, desired_wait_time } => {
                let elapsed = last_called_at.elapsed();

                if (&elapsed >= desired_wait_time){
                    TimerStatus::Ready
                }else{
                    TimerStatus::Waiting(*desired_wait_time - elapsed)
                }
            },
            Timing::Never => TimerStatus::Ignore
        }
    }
}