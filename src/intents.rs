//! Intent flags for controlling which events the bot receives.
//!
//! This module provides the `Intents` struct and related functionality for managing
//! which gateway events your bot will receive. Intents act as a permission system
//! for gateway events.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the intents that control which gateway events the bot receives.
///
/// Intents are a system that allows you to control which events your bot receives
/// over the gateway connection. This helps reduce bandwidth and processing overhead
/// by only receiving events your bot actually needs.
///
/// # Examples
///
/// ```rust
/// use botrs::Intents;
///
/// // Create intents for basic guild and message events
/// let intents = Intents::default();
///
/// // Create intents with specific events enabled
/// let intents = Intents::new()
///     .with_guilds()
///     .with_public_guild_messages()
///     .with_direct_message();
///
/// // Enable all available intents
/// let intents = Intents::all();
///
/// // Start with no intents and selectively enable
/// let intents = Intents::none()
///     .with_public_guild_messages();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Intents {
    /// The raw bits representing the enabled intents
    pub bits: u32,
}

impl Intents {
    /// Create a new empty set of intents.
    pub const fn new() -> Self {
        Self { bits: 0 }
    }

    /// Create an intent set with no intents enabled.
    pub const fn none() -> Self {
        Self::new()
    }

    /// Create an intent set with all available intents enabled.
    pub const fn all() -> Self {
        Self {
            bits: Self::GUILDS
                | Self::GUILD_MEMBERS
                | Self::GUILD_MESSAGES
                | Self::GUILD_MESSAGE_REACTIONS
                | Self::DIRECT_MESSAGE
                | Self::INTERACTION
                | Self::MESSAGE_AUDIT
                | Self::FORUMS
                | Self::AUDIO_ACTION
                | Self::PUBLIC_GUILD_MESSAGES
                | Self::AUDIO_OR_LIVE_CHANNEL_MEMBER
                | Self::OPEN_FORUM_EVENT
                | Self::PUBLIC_MESSAGES,
        }
    }

    /// Create the default set of intents for most bots.
    ///
    /// This includes all public intents and excludes privileged intents
    /// that require special permissions (guild_messages and forums).
    pub const fn default() -> Self {
        Self::all()
            .without_intent(Self::GUILD_MESSAGES)
            .without_intent(Self::FORUMS)
    }

    // Intent flag constants
    /// Guilds intent - guild create/update/delete events
    pub const GUILDS: u32 = 1 << 0;

    /// Guild members intent - member join/update/leave events
    pub const GUILD_MEMBERS: u32 = 1 << 1;

    /// Guild messages intent - all messages in guilds (privileged)
    pub const GUILD_MESSAGES: u32 = 1 << 9;

    /// Guild message reactions intent - reaction add/remove events
    pub const GUILD_MESSAGE_REACTIONS: u32 = 1 << 10;

    /// Direct messages intent - private message events
    pub const DIRECT_MESSAGE: u32 = 1 << 12;

    /// Interaction intent - button clicks, slash commands, etc.
    pub const INTERACTION: u32 = 1 << 26;

    /// Message audit intent - message audit events
    pub const MESSAGE_AUDIT: u32 = 1 << 27;

    /// Forums intent - forum thread and post events (privileged)
    pub const FORUMS: u32 = 1 << 28;

    /// Audio action intent - voice channel events
    pub const AUDIO_ACTION: u32 = 1 << 29;

    /// Public guild messages intent - @mentions and replies
    pub const PUBLIC_GUILD_MESSAGES: u32 = 1 << 30;

    /// Audio or live channel member intent - voice/live channel member events
    pub const AUDIO_OR_LIVE_CHANNEL_MEMBER: u32 = 1 << 19;

    /// Open forum event intent - public forum events
    pub const OPEN_FORUM_EVENT: u32 = 1 << 18;

    /// Public messages intent - group and C2C message events
    pub const PUBLIC_MESSAGES: u32 = 1 << 25;

    /// Check if a specific intent is enabled.
    pub const fn contains(self, intent: u32) -> bool {
        (self.bits & intent) == intent
    }

    /// Enable a specific intent.
    pub const fn with_intent(mut self, intent: u32) -> Self {
        self.bits |= intent;
        self
    }

    /// Disable a specific intent.
    pub const fn without_intent(mut self, intent: u32) -> Self {
        self.bits &= !intent;
        self
    }

    /// Enable guilds intent.
    pub const fn with_guilds(self) -> Self {
        self.with_intent(Self::GUILDS)
    }

    /// Enable guild members intent.
    pub const fn with_guild_members(self) -> Self {
        self.with_intent(Self::GUILD_MEMBERS)
    }

    /// Enable guild messages intent (privileged).
    pub const fn with_guild_messages(self) -> Self {
        self.with_intent(Self::GUILD_MESSAGES)
    }

    /// Enable guild message reactions intent.
    pub const fn with_guild_message_reactions(self) -> Self {
        self.with_intent(Self::GUILD_MESSAGE_REACTIONS)
    }

    /// Enable direct messages intent.
    pub const fn with_direct_message(self) -> Self {
        self.with_intent(Self::DIRECT_MESSAGE)
    }

    /// Enable interaction intent.
    pub const fn with_interaction(self) -> Self {
        self.with_intent(Self::INTERACTION)
    }

    /// Enable message audit intent.
    pub const fn with_message_audit(self) -> Self {
        self.with_intent(Self::MESSAGE_AUDIT)
    }

    /// Enable forums intent (privileged).
    pub const fn with_forums(self) -> Self {
        self.with_intent(Self::FORUMS)
    }

    /// Enable audio action intent.
    pub const fn with_audio_action(self) -> Self {
        self.with_intent(Self::AUDIO_ACTION)
    }

    /// Enable public guild messages intent.
    pub const fn with_public_guild_messages(self) -> Self {
        self.with_intent(Self::PUBLIC_GUILD_MESSAGES)
    }

    /// Enable audio or live channel member intent.
    pub const fn with_audio_or_live_channel_member(self) -> Self {
        self.with_intent(Self::AUDIO_OR_LIVE_CHANNEL_MEMBER)
    }

    /// Enable open forum event intent.
    pub const fn with_open_forum_event(self) -> Self {
        self.with_intent(Self::OPEN_FORUM_EVENT)
    }

    /// Enable public messages intent.
    pub const fn with_public_messages(self) -> Self {
        self.with_intent(Self::PUBLIC_MESSAGES)
    }

    /// Check if guilds intent is enabled.
    pub const fn guilds(self) -> bool {
        self.contains(Self::GUILDS)
    }

    /// Check if guild members intent is enabled.
    pub const fn guild_members(self) -> bool {
        self.contains(Self::GUILD_MEMBERS)
    }

    /// Check if guild messages intent is enabled.
    pub const fn guild_messages(self) -> bool {
        self.contains(Self::GUILD_MESSAGES)
    }

    /// Check if guild message reactions intent is enabled.
    pub const fn guild_message_reactions(self) -> bool {
        self.contains(Self::GUILD_MESSAGE_REACTIONS)
    }

    /// Check if direct messages intent is enabled.
    pub const fn direct_message(self) -> bool {
        self.contains(Self::DIRECT_MESSAGE)
    }

    /// Check if interaction intent is enabled.
    pub const fn interaction(self) -> bool {
        self.contains(Self::INTERACTION)
    }

    /// Check if message audit intent is enabled.
    pub const fn message_audit(self) -> bool {
        self.contains(Self::MESSAGE_AUDIT)
    }

    /// Check if forums intent is enabled.
    pub const fn forums(self) -> bool {
        self.contains(Self::FORUMS)
    }

    /// Check if audio action intent is enabled.
    pub const fn audio_action(self) -> bool {
        self.contains(Self::AUDIO_ACTION)
    }

    /// Check if public guild messages intent is enabled.
    pub const fn public_guild_messages(self) -> bool {
        self.contains(Self::PUBLIC_GUILD_MESSAGES)
    }

    /// Check if audio or live channel member intent is enabled.
    pub const fn audio_or_live_channel_member(self) -> bool {
        self.contains(Self::AUDIO_OR_LIVE_CHANNEL_MEMBER)
    }

    /// Check if open forum event intent is enabled.
    pub const fn open_forum_event(self) -> bool {
        self.contains(Self::OPEN_FORUM_EVENT)
    }

    /// Check if public messages intent is enabled.
    pub const fn public_messages(self) -> bool {
        self.contains(Self::PUBLIC_MESSAGES)
    }

    /// Check if any privileged intents are enabled.
    ///
    /// Privileged intents require special approval from QQ.
    pub const fn has_privileged(self) -> bool {
        self.contains(Self::GUILD_MESSAGES) || self.contains(Self::FORUMS)
    }

    /// Get the raw intent bits.
    pub const fn bits(self) -> u32 {
        self.bits
    }

    /// Create intents from raw bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }
}

impl Default for Intents {
    fn default() -> Self {
        Self::default()
    }
}

impl fmt::Display for Intents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if self.guilds() {
            parts.push("GUILDS");
        }
        if self.guild_members() {
            parts.push("GUILD_MEMBERS");
        }
        if self.guild_messages() {
            parts.push("GUILD_MESSAGES");
        }
        if self.guild_message_reactions() {
            parts.push("GUILD_MESSAGE_REACTIONS");
        }
        if self.direct_message() {
            parts.push("DIRECT_MESSAGE");
        }
        if self.interaction() {
            parts.push("INTERACTION");
        }
        if self.message_audit() {
            parts.push("MESSAGE_AUDIT");
        }
        if self.forums() {
            parts.push("FORUMS");
        }
        if self.audio_action() {
            parts.push("AUDIO_ACTION");
        }
        if self.public_guild_messages() {
            parts.push("PUBLIC_GUILD_MESSAGES");
        }
        if self.audio_or_live_channel_member() {
            parts.push("AUDIO_OR_LIVE_CHANNEL_MEMBER");
        }
        if self.open_forum_event() {
            parts.push("OPEN_FORUM_EVENT");
        }
        if self.public_messages() {
            parts.push("PUBLIC_MESSAGES");
        }

        if parts.is_empty() {
            write!(f, "Intents(NONE)")
        } else {
            write!(f, "Intents({})", parts.join(" | "))
        }
    }
}

impl std::ops::BitOr for Intents {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl std::ops::BitOrAssign for Intents {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl std::ops::BitAnd for Intents {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits,
        }
    }
}

impl std::ops::BitAndAssign for Intents {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits;
    }
}

impl std::ops::BitXor for Intents {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits ^ rhs.bits,
        }
    }
}

impl std::ops::BitXorAssign for Intents {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits;
    }
}

impl std::ops::Not for Intents {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self { bits: !self.bits }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_creation() {
        let intents = Intents::new();
        assert_eq!(intents.bits(), 0);

        let intents = Intents::all();
        assert!(intents.guilds());
        assert!(intents.public_guild_messages());
    }

    #[test]
    fn test_intent_operations() {
        let mut intents = Intents::none();
        assert!(!intents.guilds());

        intents = intents.with_guilds();
        assert!(intents.guilds());

        let other = Intents::none().with_public_guild_messages();
        let combined = intents | other;
        assert!(combined.guilds());
        assert!(combined.public_guild_messages());
    }

    #[test]
    fn test_privileged_intents() {
        let intents = Intents::none().with_guild_messages();
        assert!(intents.has_privileged());

        let intents = Intents::none().with_forums();
        assert!(intents.has_privileged());

        let intents = Intents::none().with_public_guild_messages();
        assert!(!intents.has_privileged());
    }

    #[test]
    fn test_display() {
        let intents = Intents::none();
        assert_eq!(format!("{}", intents), "Intents(NONE)");

        let intents = Intents::none().with_guilds().with_public_guild_messages();
        let display = format!("{}", intents);
        assert!(display.contains("GUILDS"));
        assert!(display.contains("PUBLIC_GUILD_MESSAGES"));
    }
}
