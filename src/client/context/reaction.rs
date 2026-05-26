use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn add_reaction(
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

    /// Adds a reaction to a message using a structured emoji object.

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

    /// Removes a reaction from a message.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    /// * `emoji_type` - The emoji type (1 for system emoji, 2 for custom emoji)
    /// * `emoji_id` - The emoji ID
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.

    pub async fn remove_reaction(
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

    /// Deletes own reaction from a message using a structured emoji object.

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

    /// Gets message reaction users using structured emoji and pager objects.

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
}
