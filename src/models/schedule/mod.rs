//! Schedule-related data structures for the QQ Guild Bot API.
//!
//! This module contains structures for creating and managing channel schedules
//! in QQ Guild bots.

mod model;
mod remind;
mod wrapper;

pub use model::Schedule;
pub use remind::RemindType;
pub use wrapper::ScheduleWrapper;

#[cfg(test)]
mod tests;
