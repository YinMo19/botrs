//! Interaction-related functionality for QQ Bot
//!
//! This module provides structures and implementations for handling user interactions,
//! including button clicks, command interactions, and other interactive elements.

mod data;
mod interaction;
mod search;
mod types;

pub use data::{InteractionData, Resolved};
pub use interaction::Interaction;
pub use search::{SearchInputResolved, SearchLayout, SearchRecord, SearchRsp};
pub use types::{
    ACTION_TYPE_SEND_ARK, ActionType, ActionTypeSendARK, InteractionDataType,
    InteractionDataTypeCallbackCommandClick, InteractionDataTypeChatSearch,
    InteractionDataTypeClearSessionClick, InteractionDataTypeInlineKeyboardClick,
    InteractionDataTypeMessageFeedbackClick, InteractionType, InteractionTypeCommand,
    InteractionTypePing, LAYOUT_TYPE_IMAGE_TEXT, LayoutType, LayoutTypeImageText,
};

#[cfg(test)]
mod tests;
