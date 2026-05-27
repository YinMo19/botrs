use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Interaction type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InteractionType {
    /// Ping interaction
    Ping = 1,
    /// Application command interaction
    ApplicationCommand = 2,
    /// HTTP proxy interaction
    HttpProxy = 10,
    /// Inline keyboard interaction
    InlineKeyboard = 11,
}

wire_enum_with_default!(InteractionType, u8, Ping, {
    Ping = 1,
    ApplicationCommand = 2,
    HttpProxy = 10,
    InlineKeyboard = 11,
});

impl Serialize for InteractionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for InteractionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(u8::deserialize(deserializer)?))
    }
}

/// Interaction data type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InteractionDataType {
    /// Chat input search
    ChatInputSearch = 9,
    /// HTTP proxy
    HttpProxy = 10,
    /// Inline keyboard button click
    InlineKeyboardButtonClick = 11,
    /// C2C callback command click
    CallbackCommandClick = 12,
    /// Message feedback click
    MessageFeedbackClick = 13,
    /// Clear session click
    ClearSessionClick = 14,
}

pub type LayoutType = u32;
pub const LAYOUT_TYPE_IMAGE_TEXT: LayoutType = 0;
pub type ActionType = u32;
pub const ACTION_TYPE_SEND_ARK: ActionType = 0;

wire_enum_with_default!(InteractionDataType, u8, ChatInputSearch, {
    ChatInputSearch = 9,
    HttpProxy = 10,
    InlineKeyboardButtonClick = 11,
    CallbackCommandClick = 12,
    MessageFeedbackClick = 13,
    ClearSessionClick = 14,
});

impl Serialize for InteractionDataType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for InteractionDataType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(u8::deserialize(deserializer)?))
    }
}
