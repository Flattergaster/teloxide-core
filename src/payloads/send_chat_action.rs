// This file is auto generated by `cg` <https://github.com/teloxide/cg> (9a82143).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatAction, ChatId, Message};

impl_payload! {
    /// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.
    ///
    /// > Example: The [ImageBot] needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use sendChatAction with action = upload_photo. The user will see a “sending photo” status for the bot.
    ///
    /// We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.
    ///
    /// [ImageBot]: https://t.me/imagebot
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendChatAction (SendChatActionSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for [text messages], upload_photo for [photos], record_video or upload_video for [videos], record_audio or upload_audio for [audio files], upload_document for [general files], find_location for [location data], record_video_note or upload_video_note for [video notes].
            ///
            /// [video notes]: crate::payloads::SendVideoNote
            /// [audio files]: crate::payloads::SendAudio
            /// [general files]: crate::payloads::SendDocument
            /// [location data]: crate::payloads::SendLocation
            /// [text messages]: crate::payloads::SendMessage
            /// [photos]: crate::payloads::SendPhoto
            /// [videos]: crate::payloads::SendVideo
            pub action: ChatAction,
        }
    }
}
