//! User-related data models for the QQ Guild Bot API.

mod member;
mod model;
mod role;

pub use member::Member;
pub use model::User;
pub use role::Role;

#[cfg(test)]
mod tests;
