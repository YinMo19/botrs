use super::Intents;

pub type Intent = u32;
#[allow(non_upper_case_globals)]
pub const IntentGuilds: Intent = Intents::GUILDS;
#[allow(non_upper_case_globals)]
pub const IntentGuildMembers: Intent = Intents::GUILD_MEMBERS;
#[allow(non_upper_case_globals)]
pub const IntentGuildBans: Intent = 1 << 2;
#[allow(non_upper_case_globals)]
pub const IntentGuildEmojis: Intent = 1 << 3;
#[allow(non_upper_case_globals)]
pub const IntentGuildIntegrations: Intent = 1 << 4;
#[allow(non_upper_case_globals)]
pub const IntentGuildWebhooks: Intent = 1 << 5;
#[allow(non_upper_case_globals)]
pub const IntentGuildInvites: Intent = 1 << 6;
#[allow(non_upper_case_globals)]
pub const IntentGuildVoiceStates: Intent = 1 << 7;
#[allow(non_upper_case_globals)]
pub const IntentGuildPresences: Intent = 1 << 8;
#[allow(non_upper_case_globals)]
pub const IntentGuildMessages: Intent = Intents::GUILD_MESSAGES;
#[allow(non_upper_case_globals)]
pub const IntentGuildMessageReactions: Intent = Intents::GUILD_MESSAGE_REACTIONS;
#[allow(non_upper_case_globals)]
pub const IntentGuildMessageTyping: Intent = 1 << 11;
#[allow(non_upper_case_globals)]
pub const IntentDirectMessages: Intent = Intents::DIRECT_MESSAGE;
#[allow(non_upper_case_globals)]
pub const IntentDirectMessageReactions: Intent = 1 << 13;
#[allow(non_upper_case_globals)]
pub const IntentDirectMessageTyping: Intent = 1 << 14;
#[allow(non_upper_case_globals)]
pub const IntentEnterAIO: Intent = Intents::ENTER_AIO;
#[allow(non_upper_case_globals)]
pub const IntentGroupMessages: Intent = Intents::PUBLIC_MESSAGES;
#[allow(non_upper_case_globals)]
pub const IntentInteraction: Intent = Intents::INTERACTION;
#[allow(non_upper_case_globals)]
pub const IntentAudit: Intent = Intents::MESSAGE_AUDIT;
#[allow(non_upper_case_globals)]
pub const IntentForum: Intent = Intents::FORUMS;
#[allow(non_upper_case_globals)]
pub const IntentAudio: Intent = Intents::AUDIO_ACTION;
#[allow(non_upper_case_globals)]
pub const IntentGuildAtMessage: Intent = Intents::PUBLIC_GUILD_MESSAGES;
#[allow(non_upper_case_globals)]
pub const IntentNone: Intent = 0;
