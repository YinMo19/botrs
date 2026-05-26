use serde::{Deserialize, Serialize};

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
                | Self::ENTER_AIO
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

    /// Enter AIO intent
    pub const ENTER_AIO: u32 = 1 << 23;

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

    /// Enable enter AIO intent.
    pub const fn with_enter_aio(self) -> Self {
        self.with_intent(Self::ENTER_AIO)
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

    /// Check if enter AIO intent is enabled.
    pub const fn enter_aio(self) -> bool {
        self.contains(Self::ENTER_AIO)
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
