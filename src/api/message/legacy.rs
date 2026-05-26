use base64::Engine;

use crate::models::message::{
    Ark, C2CMessageParams, DirectMessageParams, Embed, GroupMessageParams, Keyboard,
    KeyboardPayload, MarkdownPayload, Media, MessageParams, Reference,
};

pub(crate) struct ChannelLikeMessageParts {
    content: Option<String>,
    embed: Option<Embed>,
    ark: Option<Ark>,
    message_reference: Option<Reference>,
    image: Option<String>,
    file_image: Option<String>,
    msg_id: Option<String>,
    event_id: Option<String>,
    markdown: Option<MarkdownPayload>,
    keyboard: Option<Keyboard>,
}

impl ChannelLikeMessageParts {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        content: Option<&str>,
        embed: Option<&Embed>,
        ark: Option<&Ark>,
        message_reference: Option<&Reference>,
        image: Option<&str>,
        file_image: Option<&[u8]>,
        msg_id: Option<&str>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&Keyboard>,
    ) -> Self {
        Self {
            content: content.map(str::to_string),
            embed: embed.cloned(),
            ark: ark.cloned(),
            message_reference: message_reference.cloned(),
            image: image.map(str::to_string),
            file_image: file_image
                .map(|data| base64::engine::general_purpose::STANDARD.encode(data)),
            msg_id: msg_id.map(str::to_string),
            event_id: event_id.map(str::to_string),
            markdown: markdown.cloned(),
            keyboard: keyboard.cloned(),
        }
    }
}

impl From<ChannelLikeMessageParts> for MessageParams {
    fn from(parts: ChannelLikeMessageParts) -> Self {
        Self {
            content: parts.content,
            msg_type: None,
            embed: parts.embed,
            ark: parts.ark,
            message_reference: parts.message_reference,
            image: parts.image,
            file_image: parts.file_image,
            msg_id: parts.msg_id,
            event_id: parts.event_id,
            markdown: parts.markdown,
            keyboard: parts.keyboard,
            ..Default::default()
        }
    }
}

impl From<ChannelLikeMessageParts> for DirectMessageParams {
    fn from(parts: ChannelLikeMessageParts) -> Self {
        Self {
            content: parts.content,
            msg_type: None,
            embed: parts.embed,
            ark: parts.ark,
            message_reference: parts.message_reference,
            image: parts.image,
            file_image: parts.file_image,
            msg_id: parts.msg_id,
            event_id: parts.event_id,
            markdown: parts.markdown,
            keyboard: parts.keyboard,
            ..Default::default()
        }
    }
}

pub(crate) struct OpenMessageParts {
    msg_type: u32,
    content: Option<String>,
    embed: Option<Embed>,
    ark: Option<Ark>,
    message_reference: Option<Reference>,
    media: Option<Media>,
    msg_id: Option<String>,
    msg_seq: Option<u32>,
    event_id: Option<String>,
    markdown: Option<MarkdownPayload>,
    keyboard: Option<KeyboardPayload>,
}

impl OpenMessageParts {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        msg_type: Option<u32>,
        content: Option<&str>,
        embed: Option<&Embed>,
        ark: Option<&Ark>,
        message_reference: Option<&Reference>,
        media: Option<&Media>,
        msg_id: Option<&str>,
        msg_seq: Option<u32>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&KeyboardPayload>,
    ) -> Self {
        Self {
            msg_type: msg_type.unwrap_or(0),
            content: content.map(str::to_string),
            embed: embed.cloned(),
            ark: ark.cloned(),
            message_reference: message_reference.cloned(),
            media: media.cloned(),
            msg_id: msg_id.map(str::to_string),
            msg_seq: Some(msg_seq.unwrap_or(1)),
            event_id: event_id.map(str::to_string),
            markdown: markdown.cloned(),
            keyboard: keyboard.cloned(),
        }
    }
}

impl From<OpenMessageParts> for GroupMessageParams {
    fn from(parts: OpenMessageParts) -> Self {
        Self {
            msg_type: parts.msg_type,
            content: parts.content,
            embed: parts.embed,
            ark: parts.ark,
            message_reference: parts.message_reference,
            media: parts.media,
            msg_id: parts.msg_id,
            msg_seq: parts.msg_seq,
            event_id: parts.event_id,
            markdown: parts.markdown,
            keyboard: parts.keyboard,
            ..Default::default()
        }
    }
}

impl From<OpenMessageParts> for C2CMessageParams {
    fn from(parts: OpenMessageParts) -> Self {
        Self {
            msg_type: parts.msg_type,
            content: parts.content,
            embed: parts.embed,
            ark: parts.ark,
            message_reference: parts.message_reference,
            media: parts.media,
            msg_id: parts.msg_id,
            msg_seq: parts.msg_seq,
            event_id: parts.event_id,
            markdown: parts.markdown,
            keyboard: parts.keyboard,
            ..Default::default()
        }
    }
}
