// This file is auto generated by `cg` <https://github.com/teloxide/cg> (9a82143).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, ChatMember};

impl_payload! {
    /// Use this method to get a list of administrators in a chat. On success, returns an Array of [`ChatMember`] objects that contains information about all chat administrators except other bots. If the chat is a group or a supergroup and no administrators were appointed, only the creator will be returned.
    ///
    /// [`ChatMember`]: crate::types::ChatMember
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub GetChatAdministrators (GetChatAdministratorsSetters) => ChatMember {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
        }
    }
}
