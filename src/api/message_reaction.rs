use super::{BotApi, resource};
use crate::error::Result;
use crate::models::emoji::EmojiType;
use crate::reaction::{Emoji as ReactionEmoji, MessageReactionPager, ReactionUsers};
use crate::token::Token;
use std::collections::HashMap;
use tracing::debug;

impl BotApi {
    /// Adds a reaction to a message using raw emoji type and ID values.
    pub async fn put_reaction(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        emoji_type: i32,
        emoji_id: &str,
    ) -> Result<()> {
        debug!(
            "Adding reaction to message {} in channel {}",
            message_id, channel_id
        );
        let path = resource::message_reaction(channel_id, message_id, emoji_type, emoji_id);
        self.http
            .put(token, &path, None::<&()>, None::<&()>)
            .await?;
        Ok(())
    }

    /// Adds a reaction to a message using a structured emoji value.
    pub async fn create_message_reaction(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        emoji: &ReactionEmoji,
    ) -> Result<()> {
        self.put_reaction(token, channel_id, message_id, emoji.emoji_type, &emoji.id)
            .await
    }

    /// Removes the bot's reaction using raw emoji type and ID values.
    pub async fn delete_reaction(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        emoji_type: i32,
        emoji_id: &str,
    ) -> Result<()> {
        debug!(
            "Removing reaction from message {} in channel {}",
            message_id, channel_id
        );
        let path = resource::message_reaction(channel_id, message_id, emoji_type, emoji_id);
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }

    /// Removes the bot's reaction using a structured emoji value.
    pub async fn delete_own_message_reaction(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        emoji: &ReactionEmoji,
    ) -> Result<()> {
        self.delete_reaction(token, channel_id, message_id, emoji.emoji_type, &emoji.id)
            .await
    }

    /// Lists users that reacted with a specific emoji.
    ///
    /// `cookie` continues pagination from a previous response; `limit` defaults
    /// to the platform default when omitted.
    pub async fn get_reaction_users(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        emoji_type: EmojiType,
        emoji_id: &str,
        cookie: Option<&str>,
        limit: Option<u32>,
    ) -> Result<ReactionUsers> {
        debug!(
            "Getting reaction users for message {} with emoji {}",
            message_id, emoji_id
        );

        let mut params = HashMap::new();
        params.insert("limit", limit.unwrap_or(20).to_string());
        if let Some(cookie) = cookie {
            params.insert("cookie", cookie.to_string());
        }

        let path =
            resource::message_reaction(channel_id, message_id, u8::from(emoji_type), emoji_id);
        let response = self.http.get(token, &path, Some(&params)).await?;
        Self::decode_json(response)
    }

    /// Lists users that reacted with a specific emoji using structured options.
    pub async fn get_message_reaction_users(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        emoji: &ReactionEmoji,
        pager: &MessageReactionPager,
    ) -> Result<ReactionUsers> {
        debug!(
            "Getting reaction users for message {} with emoji {:?}",
            message_id, emoji.id
        );
        let params = pager.query_params();
        let path = resource::message_reaction(channel_id, message_id, emoji.emoji_type, &emoji.id);
        let response = self
            .http
            .get(
                token,
                &path,
                if params.is_empty() {
                    None
                } else {
                    Some(&params)
                },
            )
            .await?;
        Self::decode_json(response)
    }
}
