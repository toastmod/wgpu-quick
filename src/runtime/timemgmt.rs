use std::collections::HashMap;
use crate::looputil::Timing;

pub(crate) struct LoopSchedule{
    timers: HashMap<usize, Timing>
}

// impl LoopSchedule {
//
//     pub(crate) fn rebuild(&mut self) {
//         for timer in &self.timers {
//
//         }
//     }
//
//     pub fn set_timing(&mut self, id: usize, timing: Timing) {
//
//     }
// }