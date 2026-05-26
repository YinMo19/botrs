//! Schedule-related data structures for the QQ Guild Bot API.
//!
//! This module contains structures for creating and managing channel schedules
//! in QQ Guild bots.

mod remind;
mod schedule;
mod wrapper;

pub use remind::RemindType;
pub use schedule::Schedule;
pub use wrapper::ScheduleWrapper;

#[cfg(test)]
mod tests;
