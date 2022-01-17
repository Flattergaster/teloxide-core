// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{InlineKeyboardMarkup, MessageEntity, ParseMode, True};

impl_payload! {
    /// Use this method to edit captions of messages. On success, _True_ is returned.
    ///
    /// See also: [`EditMessageCaption`](crate::payloads::EditMessageCaption)
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub EditMessageCaptionInline (EditMessageCaptionInlineSetters) => True {
        required {
            /// Identifier of the inline message
            pub inline_message_id: String,
        }
        optional {
            /// New caption of the message, 0-1024 characters after entities parsing
            pub caption: String,
            /// Mode for parsing entities in the message text. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub parse_mode: ParseMode,
            /// List of special entities that appear in the caption, which can be specified instead of _parse\_mode_
            pub caption_entities: Vec<MessageEntity>,
            /// A JSON-serialized object for an [inline keyboard].
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: InlineKeyboardMarkup,
        }
    }
}
