use super::Intents;
use std::fmt;

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
        if self.enter_aio() {
            parts.push("ENTER_AIO");
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
