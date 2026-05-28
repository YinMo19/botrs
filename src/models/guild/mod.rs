//! Guild-related data models for the QQ Guild Bot API.
//!
//! This module contains guild types for the QQ Bot Open API.

mod member;
mod model;
mod pager;
mod settings;

pub use member::Member;
pub use model::Guild;
pub use pager::{GuildMembersPager, GuildPager, GuildRoleMembers, GuildRoleMembersPager};
pub use settings::{DeleteHistoryMsgDays, MemberDeleteOptions, MessageSetting};

#[cfg(test)]
mod tests;
