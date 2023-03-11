use std::time::{Instant, Duration};

pub enum TimerStatus {
    Ready,
    Waiting(Instant),
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
    Framerate{next_call: Instant, desired_framerate: f64},

    /// Wait a specific `Duration`
    SpecificTime{next_call: Instant, desired_wait_time: Duration },

    /// This will not call the function.
    Never,
}

impl Timing {

    pub fn framerate(fps: f64) -> Self {
        Timing::Framerate {
            next_call: Instant::now() + nanos_per_frame(&fps),
            desired_framerate: fps
        }
    }

    pub fn rate(rate: Duration) -> Self {
        Timing::SpecificTime {
            next_call: Instant::now() + rate,
            desired_wait_time: rate
        }
    }

    /// Resets the timer.
    pub fn reset(&mut self) {
        match self {
            Timing::ASAP => {}
            // Timing::Framerate { last_called_at, desired_framerate } => {
            //     *last_called_at = Instant::now();
            // }
            // Timing::SpecificTime { last_called_at, desired_wait_time } => {
            //     *last_called_at = Instant::now();
            // }
            Timing::SpecificTime { next_call, desired_wait_time } => {
                *next_call = Instant::now() + *desired_wait_time;
            }
            Timing::Framerate { next_call, desired_framerate } => {
                *next_call = Instant::now() + nanos_per_frame(desired_framerate);
            }
            Timing::Never => {}
        }
    }

    /// Check the wait time left.
    pub fn check(&self) -> TimerStatus {
        match self {
            Timing::ASAP => TimerStatus::Ready,
            Timing::Framerate { next_call, desired_framerate: _ } => {
                if next_call <= &Instant::now() {
                    TimerStatus::Ready
                }else{
                    // TimerStatus::Waiting(delta_per_frame-elapsed)
                    TimerStatus::Waiting(next_call.clone())
                }
            },
            Timing::SpecificTime { next_call, desired_wait_time: _ } => {
                if next_call <= &Instant::now() {
                    TimerStatus::Ready
                }else{
                    TimerStatus::Waiting(next_call.clone())
                }
            },
            Timing::Never => TimerStatus::Ignore
        }
    }
}