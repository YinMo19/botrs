use crate::api::BotApi;
use crate::error::Result;
use crate::reaction::{Emoji as ReactionEmoji, MessageReactionPager, ReactionUsers};

impl BotApi {
    /// Message reaction add API.
    #[allow(non_snake_case)]
    pub async fn CreateMessageReaction(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &ReactionEmoji,
    ) -> Result<()> {
        self.create_message_reaction(self.token_required()?, channel_id, message_id, emoji)
            .await
    }

    /// Message reaction delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteOwnMessageReaction(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &ReactionEmoji,
    ) -> Result<()> {
        self.delete_own_message_reaction(self.token_required()?, channel_id, message_id, emoji)
            .await
    }

    /// Message reaction users API.
    #[allow(non_snake_case)]
    pub async fn GetMessageReactionUsers(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &ReactionEmoji,
        pager: &MessageReactionPager,
    ) -> Result<ReactionUsers> {
        self.get_message_reaction_users(
            self.token_required()?,
            channel_id,
            message_id,
            emoji,
            pager,
        )
        .await
    }
}
