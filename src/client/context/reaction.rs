use super::Context;
use crate::client::prelude::*;

impl Context {
    /// Adds a reaction.
    pub async fn put_reaction(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji_type: i32,
        emoji_id: &str,
    ) -> Result<()> {
        self.api
            .put_reaction(&self.token, channel_id, message_id, emoji_type, emoji_id)
            .await
    }

    /// Adds a reaction to a message using a structured emoji value.
    pub async fn create_message_reaction(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &ReactionEmoji,
    ) -> Result<()> {
        self.api
            .create_message_reaction(&self.token, channel_id, message_id, emoji)
            .await
    }

    /// Removes the bot's reaction.
    pub async fn delete_reaction(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji_type: i32,
        emoji_id: &str,
    ) -> Result<()> {
        self.api
            .delete_reaction(&self.token, channel_id, message_id, emoji_type, emoji_id)
            .await
    }

    /// Removes the bot's reaction using a structured emoji value.
    pub async fn delete_own_message_reaction(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &ReactionEmoji,
    ) -> Result<()> {
        self.api
            .delete_own_message_reaction(&self.token, channel_id, message_id, emoji)
            .await
    }

    /// Lists users that reacted with a specific emoji.
    pub async fn get_message_reaction_users(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &ReactionEmoji,
        pager: &MessageReactionPager,
    ) -> Result<ReactionUsers> {
        self.api
            .get_message_reaction_users(&self.token, channel_id, message_id, emoji, pager)
            .await
    }

    /// Lists users that reacted with a specific emoji.
    pub async fn get_reaction_users(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji_type: EmojiType,
        emoji_id: &str,
        cookie: Option<&str>,
        limit: Option<u32>,
    ) -> Result<ReactionUsers> {
        self.api
            .get_reaction_users(
                &self.token,
                channel_id,
                message_id,
                emoji_type,
                emoji_id,
                cookie,
                limit,
            )
            .await
    }
}
