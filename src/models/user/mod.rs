//! User-related data models for the QQ Guild Bot API.

mod member;
mod model;

pub use member::Member;
pub use model::User;

#[cfg(test)]
mod tests;
