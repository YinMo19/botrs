use serde::{Deserialize, Serialize};

macro_rules! intent_accessors {
    ($($with:ident, $check:ident, $intent:ident, $doc:literal);+ $(;)?) => {
        $(
            #[doc = concat!("Enable ", $doc, " intent.")]
            pub const fn $with(self) -> Self {
                self.with_intent(Self::$intent)
            }

            #[doc = concat!("Check if ", $doc, " intent is enabled.")]
            pub const fn $check(self) -> bool {
                self.contains(Self::$intent)
            }
        )+
    };
}

/// Represents the intents that control which gateway events the bot receives.
///
/// `Intents::new()` starts empty. `Intents::default()` is the standard public
/// preset and excludes privileged categories. `Intents::all()` includes the
/// standard categories represented by this type except `ENTER_AIO`, which stays
/// opt-in.
///
/// # Examples
///
/// ```rust
/// use botrs::Intents;
///
/// // Use the standard public preset.
/// let intents = Intents::default();
///
/// // Create a strict custom subscription from an empty set.
/// let intents = Intents::new()
///     .with_guilds()
///     .with_public_guild_messages()
///     .with_direct_message();
///
/// // Enable all standard intents except ENTER_AIO.
/// let intents = Intents::all();
///
/// // Start with no intents and selectively enable
/// let intents = Intents::new()
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

    /// Create an intent set with all standard intents except `ENTER_AIO`.
    ///
    /// This includes privileged intents such as [`Self::GUILD_MESSAGES`] and
    /// [`Self::FORUMS`]. `ENTER_AIO` remains opt-in through
    /// [`Self::with_enter_aio`].
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
    /// This includes non-privileged public intents and excludes the privileged
    /// `GUILD_MESSAGES` and `FORUMS` intents. `ENTER_AIO` remains explicitly
    /// opt-in.
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

    intent_accessors! {
        with_guilds, guilds, GUILDS, "guilds";
        with_guild_members, guild_members, GUILD_MEMBERS, "guild members";
        with_guild_messages, guild_messages, GUILD_MESSAGES, "guild messages";
        with_guild_message_reactions, guild_message_reactions, GUILD_MESSAGE_REACTIONS, "guild message reactions";
        with_direct_message, direct_message, DIRECT_MESSAGE, "direct messages";
        with_interaction, interaction, INTERACTION, "interaction";
        with_message_audit, message_audit, MESSAGE_AUDIT, "message audit";
        with_forums, forums, FORUMS, "forums";
        with_audio_action, audio_action, AUDIO_ACTION, "audio action";
        with_public_guild_messages, public_guild_messages, PUBLIC_GUILD_MESSAGES, "public guild messages";
        with_audio_or_live_channel_member, audio_or_live_channel_member, AUDIO_OR_LIVE_CHANNEL_MEMBER, "audio or live channel member";
        with_open_forum_event, open_forum_event, OPEN_FORUM_EVENT, "open forum event";
        with_enter_aio, enter_aio, ENTER_AIO, "enter AIO";
        with_public_messages, public_messages, PUBLIC_MESSAGES, "public messages";
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
