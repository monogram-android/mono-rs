///Telegram Secret Chat schema layer 73.
///Telegram schema layer 73.
/// Layer declared by the source schema.
pub const LAYER: u32 = 73;
/// Canonical source used to generate this module.
pub const SOURCE_URL: &str = "https://core.telegram.org/schema/end-to-end";
use crate::latest::api::*;
///TL type `DecryptedMessage`: <https://core.telegram.org/type/DecryptedMessage>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DecryptedMessage {
    ///LAYER 73
    ///@introduced 73
    ///Contents of an encrypted message.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessage>.
    ///Related protocol documentation: <https://core.telegram.org/api/entities>.
    ///Related protocol documentation: <https://core.telegram.org/api/files#albums-grouped-media>.
    ///Related protocol documentation: <https://core.telegram.org/api/updates#updatemessageid-updates>.
    ///Related protocol documentation: <https://core.telegram.org/constructor/decryptedMessageActionSetMessageTTL>.
    ///Related protocol documentation: <https://core.telegram.org/mtproto/TL-combinators#conditional-fields>.
    ///Related protocol documentation: <https://core.telegram.org/type/%23>.
    ///Related protocol documentation: <https://core.telegram.org/type/DecryptedMessageMedia>.
    ///Related protocol documentation: <https://core.telegram.org/type/MessageEntity>.
    ///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DecryptedMessage(DecryptedMessageConstructor),
    ///@introduced 17
    ///Contents of an encrypted service message.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageService>.
    ///Related protocol documentation: <https://core.telegram.org/api/updates#updatemessageid-updates>.
    ///Related protocol documentation: <https://core.telegram.org/type/DecryptedMessageAction>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    DecryptedMessageService(DecryptedMessageServiceConstructor),
}
impl crate::codec::TlEncode for DecryptedMessage {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DecryptedMessage(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageService(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for DecryptedMessage {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for DecryptedMessage {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            DecryptedMessageConstructor::ID => {
                Ok(Self::DecryptedMessage(crate::codec::TlDecode::decode(decoder)?))
            }
            DecryptedMessageServiceConstructor::ID => {
                Ok(
                    Self::DecryptedMessageService(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            constructor => {
                Err(crate::codec::Error::UnknownConstructor {
                    constructor,
                    offset,
                })
            }
        }
    }
}
impl crate::codec::BareEncode for DecryptedMessage {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DecryptedMessage(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageService(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for DecryptedMessage {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "DecryptedMessage",
        })
    }
}
///TL type `DecryptedMessageAction`: <https://core.telegram.org/type/DecryptedMessageAction>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DecryptedMessageAction {
    ///@introduced 8
    ///Setting of a message lifetime after reading.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionSetMessageTTL>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    DecryptedMessageActionSetMessageTtl(DecryptedMessageActionSetMessageTtlConstructor),
    ///@introduced 8
    ///Messages marked as read.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionReadMessages>.
    ///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    DecryptedMessageActionReadMessages(DecryptedMessageActionReadMessagesConstructor),
    ///@introduced 8
    ///Deleted messages.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionDeleteMessages>.
    ///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    DecryptedMessageActionDeleteMessages(
        DecryptedMessageActionDeleteMessagesConstructor,
    ),
    ///@introduced 8
    ///A screenshot was taken.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionScreenshotMessages>.
    ///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    DecryptedMessageActionScreenshotMessages(
        DecryptedMessageActionScreenshotMessagesConstructor,
    ),
    ///@introduced 8
    ///The entire message history has been deleted.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionFlushHistory>.
    DecryptedMessageActionFlushHistory(DecryptedMessageActionFlushHistoryConstructor),
    ///@introduced 17
    ///Request for the other party in a Secret Chat to automatically resend a contiguous range of previously sent messages, as explained in Sequence number is Secret Chats .
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionResend>.
    ///Related protocol documentation: <https://core.telegram.org/api/end-to-end/seq_no>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    DecryptedMessageActionResend(DecryptedMessageActionResendConstructor),
    ///@introduced 17
    ///A notification stating the API layer that is used by the client. You should use your current layer and take notice of the layer used on the other side of a conversation when sending messages.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionNotifyLayer>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    DecryptedMessageActionNotifyLayer(DecryptedMessageActionNotifyLayerConstructor),
    ///@introduced 17
    ///User is preparing a message: typing, recording, uploading, etc.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionTyping>.
    ///Related protocol documentation: <https://core.telegram.org/type/SendMessageAction>.
    DecryptedMessageActionTyping(DecryptedMessageActionTypingConstructor),
    ///@introduced 20
    ///Request rekeying, see rekeying process
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionRequestKey>.
    ///Related protocol documentation: <https://core.telegram.org/api/end-to-end/pfs>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    DecryptedMessageActionRequestKey(DecryptedMessageActionRequestKeyConstructor),
    ///@introduced 20
    ///Accept new key
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionAcceptKey>.
    ///Related protocol documentation: <https://core.telegram.org/api/end-to-end/pfs>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    DecryptedMessageActionAcceptKey(DecryptedMessageActionAcceptKeyConstructor),
    ///@introduced 20
    ///Abort rekeying
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionAbortKey>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    DecryptedMessageActionAbortKey(DecryptedMessageActionAbortKeyConstructor),
    ///@introduced 20
    ///Commit new key, see rekeying process
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionCommitKey>.
    ///Related protocol documentation: <https://core.telegram.org/api/end-to-end/pfs>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    DecryptedMessageActionCommitKey(DecryptedMessageActionCommitKeyConstructor),
    ///@introduced 20
    ///NOOP action
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionNoop>.
    DecryptedMessageActionNoop(DecryptedMessageActionNoopConstructor),
}
impl crate::codec::TlEncode for DecryptedMessageAction {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DecryptedMessageActionSetMessageTtl(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageActionReadMessages(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageActionDeleteMessages(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageActionScreenshotMessages(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageActionFlushHistory(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageActionResend(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageActionNotifyLayer(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageActionTyping(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageActionRequestKey(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageActionAcceptKey(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageActionAbortKey(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageActionCommitKey(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageActionNoop(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for DecryptedMessageAction {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for DecryptedMessageAction {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            DecryptedMessageActionSetMessageTtlConstructor::ID => {
                Ok(
                    Self::DecryptedMessageActionSetMessageTtl(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageActionReadMessagesConstructor::ID => {
                Ok(
                    Self::DecryptedMessageActionReadMessages(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageActionDeleteMessagesConstructor::ID => {
                Ok(
                    Self::DecryptedMessageActionDeleteMessages(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageActionScreenshotMessagesConstructor::ID => {
                Ok(
                    Self::DecryptedMessageActionScreenshotMessages(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageActionFlushHistoryConstructor::ID => {
                Ok(
                    Self::DecryptedMessageActionFlushHistory(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageActionResendConstructor::ID => {
                Ok(
                    Self::DecryptedMessageActionResend(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageActionNotifyLayerConstructor::ID => {
                Ok(
                    Self::DecryptedMessageActionNotifyLayer(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageActionTypingConstructor::ID => {
                Ok(
                    Self::DecryptedMessageActionTyping(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageActionRequestKeyConstructor::ID => {
                Ok(
                    Self::DecryptedMessageActionRequestKey(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageActionAcceptKeyConstructor::ID => {
                Ok(
                    Self::DecryptedMessageActionAcceptKey(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageActionAbortKeyConstructor::ID => {
                Ok(
                    Self::DecryptedMessageActionAbortKey(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageActionCommitKeyConstructor::ID => {
                Ok(
                    Self::DecryptedMessageActionCommitKey(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageActionNoopConstructor::ID => {
                Ok(
                    Self::DecryptedMessageActionNoop(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            constructor => {
                Err(crate::codec::Error::UnknownConstructor {
                    constructor,
                    offset,
                })
            }
        }
    }
}
impl crate::codec::BareEncode for DecryptedMessageAction {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DecryptedMessageActionSetMessageTtl(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageActionReadMessages(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageActionDeleteMessages(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageActionScreenshotMessages(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageActionFlushHistory(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageActionResend(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageActionNotifyLayer(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageActionTyping(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageActionRequestKey(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageActionAcceptKey(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageActionAbortKey(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageActionCommitKey(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageActionNoop(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for DecryptedMessageAction {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "DecryptedMessageAction",
        })
    }
}
///TL type `DecryptedMessageLayer`: <https://core.telegram.org/type/DecryptedMessageLayer>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DecryptedMessageLayer {
    ///@introduced 17
    ///Sets the layer number for the contents of an encrypted message.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageLayer>.
    ///Related protocol documentation: <https://core.telegram.org/constructor/decryptedMessage>.
    ///Related protocol documentation: <https://core.telegram.org/type/DecryptedMessage>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    DecryptedMessageLayer(DecryptedMessageLayerConstructor),
}
impl crate::codec::TlEncode for DecryptedMessageLayer {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DecryptedMessageLayer(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for DecryptedMessageLayer {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for DecryptedMessageLayer {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            DecryptedMessageLayerConstructor::ID => {
                Ok(Self::DecryptedMessageLayer(crate::codec::TlDecode::decode(decoder)?))
            }
            constructor => {
                Err(crate::codec::Error::UnknownConstructor {
                    constructor,
                    offset,
                })
            }
        }
    }
}
impl crate::codec::BareEncode for DecryptedMessageLayer {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DecryptedMessageLayer(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for DecryptedMessageLayer {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::DecryptedMessageLayer(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `DecryptedMessageMedia`: <https://core.telegram.org/type/DecryptedMessageMedia>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DecryptedMessageMedia {
    ///@introduced 8
    ///Empty constructor, no media content.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaEmpty>.
    DecryptedMessageMediaEmpty(DecryptedMessageMediaEmptyConstructor),
    ///@introduced 45
    ///Photo attached to an encrypted message.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaPhoto>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DecryptedMessageMediaPhoto(DecryptedMessageMediaPhotoConstructor),
    ///@introduced 45
    ///Video attached to an encrypted message.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaVideo>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DecryptedMessageMediaVideo(DecryptedMessageMediaVideoConstructor),
    ///@introduced 8
    ///GeoPoint attached to an encrypted message.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaGeoPoint>.
    ///Related protocol documentation: <https://core.telegram.org/type/double>.
    DecryptedMessageMediaGeoPoint(DecryptedMessageMediaGeoPointConstructor),
    ///@introduced 8
    ///Contact attached to an encrypted message.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaContact>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DecryptedMessageMediaContact(DecryptedMessageMediaContactConstructor),
    ///@introduced 45
    ///Document attached to a message in a secret chat.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaDocument>.
    ///Related protocol documentation: <https://core.telegram.org/type/DocumentAttribute>.
    ///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DecryptedMessageMediaDocument(DecryptedMessageMediaDocumentConstructor),
    ///@introduced 17
    ///Audio file attached to a secret chat message.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaAudio>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DecryptedMessageMediaAudio(DecryptedMessageMediaAudioConstructor),
    ///@introduced 23
    ///Non-e2e documented forwarded from non-secret chat
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaExternalDocument>.
    ///Related protocol documentation: <https://core.telegram.org/type/DocumentAttribute>.
    ///Related protocol documentation: <https://core.telegram.org/type/PhotoSize>.
    ///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DecryptedMessageMediaExternalDocument(
        DecryptedMessageMediaExternalDocumentConstructor,
    ),
    ///@introduced 45
    ///Venue
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaVenue>.
    ///Related protocol documentation: <https://core.telegram.org/type/double>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DecryptedMessageMediaVenue(DecryptedMessageMediaVenueConstructor),
    ///@introduced 45
    ///Webpage preview
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaWebPage>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DecryptedMessageMediaWebPage(DecryptedMessageMediaWebPageConstructor),
}
impl crate::codec::TlEncode for DecryptedMessageMedia {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DecryptedMessageMediaEmpty(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageMediaPhoto(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageMediaVideo(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageMediaGeoPoint(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageMediaContact(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageMediaDocument(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageMediaAudio(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageMediaExternalDocument(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageMediaVenue(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DecryptedMessageMediaWebPage(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for DecryptedMessageMedia {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for DecryptedMessageMedia {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            DecryptedMessageMediaEmptyConstructor::ID => {
                Ok(
                    Self::DecryptedMessageMediaEmpty(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageMediaPhotoConstructor::ID => {
                Ok(
                    Self::DecryptedMessageMediaPhoto(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageMediaVideoConstructor::ID => {
                Ok(
                    Self::DecryptedMessageMediaVideo(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageMediaGeoPointConstructor::ID => {
                Ok(
                    Self::DecryptedMessageMediaGeoPoint(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageMediaContactConstructor::ID => {
                Ok(
                    Self::DecryptedMessageMediaContact(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageMediaDocumentConstructor::ID => {
                Ok(
                    Self::DecryptedMessageMediaDocument(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageMediaAudioConstructor::ID => {
                Ok(
                    Self::DecryptedMessageMediaAudio(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageMediaExternalDocumentConstructor::ID => {
                Ok(
                    Self::DecryptedMessageMediaExternalDocument(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageMediaVenueConstructor::ID => {
                Ok(
                    Self::DecryptedMessageMediaVenue(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DecryptedMessageMediaWebPageConstructor::ID => {
                Ok(
                    Self::DecryptedMessageMediaWebPage(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            constructor => {
                Err(crate::codec::Error::UnknownConstructor {
                    constructor,
                    offset,
                })
            }
        }
    }
}
impl crate::codec::BareEncode for DecryptedMessageMedia {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DecryptedMessageMediaEmpty(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageMediaPhoto(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageMediaVideo(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageMediaGeoPoint(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageMediaContact(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageMediaDocument(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageMediaAudio(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageMediaExternalDocument(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageMediaVenue(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DecryptedMessageMediaWebPage(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for DecryptedMessageMedia {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "DecryptedMessageMedia",
        })
    }
}
///TL type `DocumentAttribute`: <https://core.telegram.org/type/DocumentAttribute>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DocumentAttribute {
    ///@introduced 23
    ///Defines the width and height of an image uploaded as document
    ///Official Telegram documentation: <https://core.telegram.org/constructor/documentAttributeImageSize>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    DocumentAttributeImageSize(DocumentAttributeImageSizeConstructor),
    ///@introduced 23
    ///Defines an animated GIF
    ///Official Telegram documentation: <https://core.telegram.org/constructor/documentAttributeAnimated>.
    DocumentAttributeAnimated(DocumentAttributeAnimatedConstructor),
    ///@introduced 45
    ///Defines a sticker
    ///Official Telegram documentation: <https://core.telegram.org/constructor/documentAttributeSticker>.
    ///Related protocol documentation: <https://core.telegram.org/constructor/true>.
    ///Related protocol documentation: <https://core.telegram.org/mtproto/TL-combinators#conditional-fields>.
    ///Related protocol documentation: <https://core.telegram.org/type/%23>.
    ///Related protocol documentation: <https://core.telegram.org/type/InputStickerSet>.
    ///Related protocol documentation: <https://core.telegram.org/type/MaskCoords>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DocumentAttributeSticker(DocumentAttributeStickerConstructor),
    ///@introduced 66
    ///Defines a video
    ///Official Telegram documentation: <https://core.telegram.org/constructor/documentAttributeVideo>.
    ///Related protocol documentation: <https://core.telegram.org/api/stories>.
    ///Related protocol documentation: <https://core.telegram.org/constructor/true>.
    ///Related protocol documentation: <https://core.telegram.org/mtproto/TL-combinators#conditional-fields>.
    ///Related protocol documentation: <https://core.telegram.org/type/%23>.
    ///Related protocol documentation: <https://core.telegram.org/type/double>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DocumentAttributeVideo(DocumentAttributeVideoConstructor),
    ///@introduced 46
    ///Represents an audio file
    ///Official Telegram documentation: <https://core.telegram.org/constructor/documentAttributeAudio>.
    ///Related protocol documentation: <https://core.telegram.org/constructor/true>.
    ///Related protocol documentation: <https://core.telegram.org/mtproto/TL-combinators#conditional-fields>.
    ///Related protocol documentation: <https://core.telegram.org/type/%23>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    ///Related protocol documentation: <https://github.com/DrKLO/Telegram/blob/4d7a3a40c88ce28a211aa2814e9569389fc40567/TMessagesProj/jni/audio.c#L562>.
    DocumentAttributeAudio(DocumentAttributeAudioConstructor),
    ///@introduced 23
    ///A simple document with a file name
    ///Official Telegram documentation: <https://core.telegram.org/constructor/documentAttributeFilename>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DocumentAttributeFilename(DocumentAttributeFilenameConstructor),
}
impl crate::codec::TlEncode for DocumentAttribute {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DocumentAttributeImageSize(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DocumentAttributeAnimated(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DocumentAttributeSticker(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DocumentAttributeVideo(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DocumentAttributeAudio(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DocumentAttributeFilename(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for DocumentAttribute {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for DocumentAttribute {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            DocumentAttributeImageSizeConstructor::ID => {
                Ok(
                    Self::DocumentAttributeImageSize(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DocumentAttributeAnimatedConstructor::ID => {
                Ok(
                    Self::DocumentAttributeAnimated(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DocumentAttributeStickerConstructor::ID => {
                Ok(
                    Self::DocumentAttributeSticker(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DocumentAttributeVideoConstructor::ID => {
                Ok(
                    Self::DocumentAttributeVideo(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DocumentAttributeAudioConstructor::ID => {
                Ok(
                    Self::DocumentAttributeAudio(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            DocumentAttributeFilenameConstructor::ID => {
                Ok(
                    Self::DocumentAttributeFilename(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            constructor => {
                Err(crate::codec::Error::UnknownConstructor {
                    constructor,
                    offset,
                })
            }
        }
    }
}
impl crate::codec::BareEncode for DocumentAttribute {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DocumentAttributeImageSize(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DocumentAttributeAnimated(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DocumentAttributeSticker(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DocumentAttributeVideo(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DocumentAttributeAudio(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DocumentAttributeFilename(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for DocumentAttribute {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "DocumentAttribute",
        })
    }
}
///TL type `FileLocation`: <https://core.telegram.org/type/FileLocation>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FileLocation {
    ///@introduced 23
    ///File is currently unavailable.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/fileLocationUnavailable>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    FileLocationUnavailable(FileLocationUnavailableConstructor),
    ///@introduced 23
    ///File location.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/fileLocation>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    FileLocation(FileLocationConstructor),
}
impl crate::codec::TlEncode for FileLocation {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::FileLocationUnavailable(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::FileLocation(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for FileLocation {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for FileLocation {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            FileLocationUnavailableConstructor::ID => {
                Ok(
                    Self::FileLocationUnavailable(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            FileLocationConstructor::ID => {
                Ok(Self::FileLocation(crate::codec::TlDecode::decode(decoder)?))
            }
            constructor => {
                Err(crate::codec::Error::UnknownConstructor {
                    constructor,
                    offset,
                })
            }
        }
    }
}
impl crate::codec::BareEncode for FileLocation {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::FileLocationUnavailable(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::FileLocation(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for FileLocation {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "FileLocation",
        })
    }
}
///TL type `InputStickerSet`: <https://core.telegram.org/type/InputStickerSet>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputStickerSet {
    ///@introduced 45
    ///Stickerset by short name, from a stickerset deep link »
    ///Official Telegram documentation: <https://core.telegram.org/constructor/inputStickerSetShortName>.
    ///Related protocol documentation: <https://core.telegram.org/api/links#stickerset-links>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    InputStickerSetShortName(InputStickerSetShortNameConstructor),
    ///@introduced 45
    ///Empty constructor
    ///Official Telegram documentation: <https://core.telegram.org/constructor/inputStickerSetEmpty>.
    InputStickerSetEmpty(InputStickerSetEmptyConstructor),
}
impl crate::codec::TlEncode for InputStickerSet {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::InputStickerSetShortName(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::InputStickerSetEmpty(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for InputStickerSet {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for InputStickerSet {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            InputStickerSetShortNameConstructor::ID => {
                Ok(
                    Self::InputStickerSetShortName(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            InputStickerSetEmptyConstructor::ID => {
                Ok(Self::InputStickerSetEmpty(crate::codec::TlDecode::decode(decoder)?))
            }
            constructor => {
                Err(crate::codec::Error::UnknownConstructor {
                    constructor,
                    offset,
                })
            }
        }
    }
}
impl crate::codec::BareEncode for InputStickerSet {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::InputStickerSetShortName(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::InputStickerSetEmpty(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for InputStickerSet {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "InputStickerSet",
        })
    }
}
///TL type `MessageEntity`: <https://core.telegram.org/type/MessageEntity>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MessageEntity {
    ///@introduced 45
    ///Unknown message entity
    ///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityUnknown>.
    ///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    MessageEntityUnknown(MessageEntityUnknownConstructor),
    ///@introduced 45
    ///Message entity mentioning a user by @username ; messageEntityMentionName can also be used to mention users by their ID.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityMention>.
    ///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
    ///Related protocol documentation: <https://core.telegram.org/api/mentions>.
    ///Related protocol documentation: <https://core.telegram.org/constructor/messageEntityMentionName>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    MessageEntityMention(MessageEntityMentionConstructor),
    ///@introduced 45
    ///#hashtag message entity
    ///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityHashtag>.
    ///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    MessageEntityHashtag(MessageEntityHashtagConstructor),
    ///@introduced 45
    ///Message entity representing a bot /command
    ///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityBotCommand>.
    ///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    MessageEntityBotCommand(MessageEntityBotCommandConstructor),
    ///@introduced 45
    ///Message entity representing an in-text url: <https://google.com> ; for text urls , use messageEntityTextUrl .
    ///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityUrl>.
    ///Related protocol documentation: <https://google.com>.
    ///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
    ///Related protocol documentation: <https://core.telegram.org/constructor/messageEntityTextUrl>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://google.com>.
    MessageEntityUrl(MessageEntityUrlConstructor),
    ///@introduced 45
    ///Message entity representing an email@example.com .
    ///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityEmail>.
    ///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    MessageEntityEmail(MessageEntityEmailConstructor),
    ///@introduced 45
    ///Message entity representing bold text .
    ///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityBold>.
    ///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    MessageEntityBold(MessageEntityBoldConstructor),
    ///@introduced 45
    ///Message entity representing italic text .
    ///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityItalic>.
    ///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    MessageEntityItalic(MessageEntityItalicConstructor),
    ///@introduced 45
    ///Message entity representing a codeblock .
    ///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityCode>.
    ///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    MessageEntityCode(MessageEntityCodeConstructor),
    ///@introduced 45
    ///Message entity representing a preformatted codeblock , allowing the user to specify a programming language for the codeblock.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityPre>.
    ///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    MessageEntityPre(MessageEntityPreConstructor),
    ///@introduced 45
    ///Message entity representing a text url : for in-text urls like <https://google.com> use messageEntityUrl .
    ///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityTextUrl>.
    ///Related protocol documentation: <https://google.com>.
    ///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
    ///Related protocol documentation: <https://core.telegram.org/constructor/messageEntityUrl>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    ///Related protocol documentation: <https://google.com>.
    MessageEntityTextUrl(MessageEntityTextUrlConstructor),
}
impl crate::codec::TlEncode for MessageEntity {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MessageEntityUnknown(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::MessageEntityMention(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::MessageEntityHashtag(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::MessageEntityBotCommand(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::MessageEntityUrl(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::MessageEntityEmail(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::MessageEntityBold(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::MessageEntityItalic(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::MessageEntityCode(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::MessageEntityPre(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::MessageEntityTextUrl(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for MessageEntity {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for MessageEntity {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            MessageEntityUnknownConstructor::ID => {
                Ok(Self::MessageEntityUnknown(crate::codec::TlDecode::decode(decoder)?))
            }
            MessageEntityMentionConstructor::ID => {
                Ok(Self::MessageEntityMention(crate::codec::TlDecode::decode(decoder)?))
            }
            MessageEntityHashtagConstructor::ID => {
                Ok(Self::MessageEntityHashtag(crate::codec::TlDecode::decode(decoder)?))
            }
            MessageEntityBotCommandConstructor::ID => {
                Ok(
                    Self::MessageEntityBotCommand(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            MessageEntityUrlConstructor::ID => {
                Ok(Self::MessageEntityUrl(crate::codec::TlDecode::decode(decoder)?))
            }
            MessageEntityEmailConstructor::ID => {
                Ok(Self::MessageEntityEmail(crate::codec::TlDecode::decode(decoder)?))
            }
            MessageEntityBoldConstructor::ID => {
                Ok(Self::MessageEntityBold(crate::codec::TlDecode::decode(decoder)?))
            }
            MessageEntityItalicConstructor::ID => {
                Ok(Self::MessageEntityItalic(crate::codec::TlDecode::decode(decoder)?))
            }
            MessageEntityCodeConstructor::ID => {
                Ok(Self::MessageEntityCode(crate::codec::TlDecode::decode(decoder)?))
            }
            MessageEntityPreConstructor::ID => {
                Ok(Self::MessageEntityPre(crate::codec::TlDecode::decode(decoder)?))
            }
            MessageEntityTextUrlConstructor::ID => {
                Ok(Self::MessageEntityTextUrl(crate::codec::TlDecode::decode(decoder)?))
            }
            constructor => {
                Err(crate::codec::Error::UnknownConstructor {
                    constructor,
                    offset,
                })
            }
        }
    }
}
impl crate::codec::BareEncode for MessageEntity {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MessageEntityUnknown(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::MessageEntityMention(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::MessageEntityHashtag(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::MessageEntityBotCommand(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::MessageEntityUrl(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::MessageEntityEmail(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::MessageEntityBold(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::MessageEntityItalic(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::MessageEntityCode(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::MessageEntityPre(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::MessageEntityTextUrl(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for MessageEntity {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "MessageEntity",
        })
    }
}
///TL type `PhotoSize`: <https://core.telegram.org/type/PhotoSize>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhotoSize {
    ///@introduced 23
    ///Empty constructor. Image with this thumbnail is unavailable.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/photoSizeEmpty>.
    ///Related protocol documentation: <https://core.telegram.org/api/files#photosizetype-values>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    PhotoSizeEmpty(PhotoSizeEmptyConstructor),
    ///@introduced 23
    ///Image description.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/photoSize>.
    ///Related protocol documentation: <https://core.telegram.org/api/files#photosizetype-values>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    PhotoSize(PhotoSizeConstructor),
    ///@introduced 23
    ///Description of an image and its content.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/photoCachedSize>.
    ///Related protocol documentation: <https://core.telegram.org/api/files#photosizetype-values>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    PhotoCachedSize(PhotoCachedSizeConstructor),
}
impl crate::codec::TlEncode for PhotoSize {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::PhotoSizeEmpty(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::PhotoSize(value) => crate::codec::Boxed::encode_boxed(value, encoder),
            Self::PhotoCachedSize(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for PhotoSize {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for PhotoSize {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            PhotoSizeEmptyConstructor::ID => {
                Ok(Self::PhotoSizeEmpty(crate::codec::TlDecode::decode(decoder)?))
            }
            PhotoSizeConstructor::ID => {
                Ok(Self::PhotoSize(crate::codec::TlDecode::decode(decoder)?))
            }
            PhotoCachedSizeConstructor::ID => {
                Ok(Self::PhotoCachedSize(crate::codec::TlDecode::decode(decoder)?))
            }
            constructor => {
                Err(crate::codec::Error::UnknownConstructor {
                    constructor,
                    offset,
                })
            }
        }
    }
}
impl crate::codec::BareEncode for PhotoSize {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::PhotoSizeEmpty(value) => crate::codec::TlEncode::encode(value, encoder),
            Self::PhotoSize(value) => crate::codec::TlEncode::encode(value, encoder),
            Self::PhotoCachedSize(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for PhotoSize {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "PhotoSize",
        })
    }
}
///TL type `SendMessageAction`: <https://core.telegram.org/type/SendMessageAction>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SendMessageAction {
    ///@introduced 17
    ///User is typing.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageTypingAction>.
    SendMessageTypingAction(SendMessageTypingActionConstructor),
    ///@introduced 17
    ///Invalidate all previous action updates. E.g. when user deletes entered text or aborts a video upload.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageCancelAction>.
    SendMessageCancelAction(SendMessageCancelActionConstructor),
    ///@introduced 17
    ///User is recording a video.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageRecordVideoAction>.
    SendMessageRecordVideoAction(SendMessageRecordVideoActionConstructor),
    ///@introduced 17
    ///User is uploading a video.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageUploadVideoAction>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    SendMessageUploadVideoAction(SendMessageUploadVideoActionConstructor),
    ///@introduced 17
    ///User is recording a voice message.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageRecordAudioAction>.
    SendMessageRecordAudioAction(SendMessageRecordAudioActionConstructor),
    ///@introduced 17
    ///User is uploading a voice message.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageUploadAudioAction>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    SendMessageUploadAudioAction(SendMessageUploadAudioActionConstructor),
    ///@introduced 17
    ///User is uploading a photo.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageUploadPhotoAction>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    SendMessageUploadPhotoAction(SendMessageUploadPhotoActionConstructor),
    ///@introduced 17
    ///User is uploading a file.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageUploadDocumentAction>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    SendMessageUploadDocumentAction(SendMessageUploadDocumentActionConstructor),
    ///@introduced 17
    ///User is selecting a location to share.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageGeoLocationAction>.
    SendMessageGeoLocationAction(SendMessageGeoLocationActionConstructor),
    ///@introduced 17
    ///User is selecting a contact to share.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageChooseContactAction>.
    SendMessageChooseContactAction(SendMessageChooseContactActionConstructor),
    ///@introduced 66
    ///User is recording a round video to share
    ///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageRecordRoundAction>.
    SendMessageRecordRoundAction(SendMessageRecordRoundActionConstructor),
    ///@introduced 66
    ///User is uploading a round video
    ///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageUploadRoundAction>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    SendMessageUploadRoundAction(SendMessageUploadRoundActionConstructor),
}
impl crate::codec::TlEncode for SendMessageAction {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::SendMessageTypingAction(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::SendMessageCancelAction(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::SendMessageRecordVideoAction(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::SendMessageUploadVideoAction(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::SendMessageRecordAudioAction(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::SendMessageUploadAudioAction(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::SendMessageUploadPhotoAction(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::SendMessageUploadDocumentAction(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::SendMessageGeoLocationAction(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::SendMessageChooseContactAction(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::SendMessageRecordRoundAction(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::SendMessageUploadRoundAction(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for SendMessageAction {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for SendMessageAction {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            SendMessageTypingActionConstructor::ID => {
                Ok(
                    Self::SendMessageTypingAction(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            SendMessageCancelActionConstructor::ID => {
                Ok(
                    Self::SendMessageCancelAction(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            SendMessageRecordVideoActionConstructor::ID => {
                Ok(
                    Self::SendMessageRecordVideoAction(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            SendMessageUploadVideoActionConstructor::ID => {
                Ok(
                    Self::SendMessageUploadVideoAction(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            SendMessageRecordAudioActionConstructor::ID => {
                Ok(
                    Self::SendMessageRecordAudioAction(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            SendMessageUploadAudioActionConstructor::ID => {
                Ok(
                    Self::SendMessageUploadAudioAction(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            SendMessageUploadPhotoActionConstructor::ID => {
                Ok(
                    Self::SendMessageUploadPhotoAction(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            SendMessageUploadDocumentActionConstructor::ID => {
                Ok(
                    Self::SendMessageUploadDocumentAction(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            SendMessageGeoLocationActionConstructor::ID => {
                Ok(
                    Self::SendMessageGeoLocationAction(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            SendMessageChooseContactActionConstructor::ID => {
                Ok(
                    Self::SendMessageChooseContactAction(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            SendMessageRecordRoundActionConstructor::ID => {
                Ok(
                    Self::SendMessageRecordRoundAction(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            SendMessageUploadRoundActionConstructor::ID => {
                Ok(
                    Self::SendMessageUploadRoundAction(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            constructor => {
                Err(crate::codec::Error::UnknownConstructor {
                    constructor,
                    offset,
                })
            }
        }
    }
}
impl crate::codec::BareEncode for SendMessageAction {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::SendMessageTypingAction(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::SendMessageCancelAction(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::SendMessageRecordVideoAction(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::SendMessageUploadVideoAction(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::SendMessageRecordAudioAction(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::SendMessageUploadAudioAction(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::SendMessageUploadPhotoAction(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::SendMessageUploadDocumentAction(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::SendMessageGeoLocationAction(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::SendMessageChooseContactAction(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::SendMessageRecordRoundAction(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::SendMessageUploadRoundAction(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for SendMessageAction {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "SendMessageAction",
        })
    }
}
///LAYER 73
///@introduced 73
///Contents of an encrypted message.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessage>.
///Related protocol documentation: <https://core.telegram.org/api/entities>.
///Related protocol documentation: <https://core.telegram.org/api/files#albums-grouped-media>.
///Related protocol documentation: <https://core.telegram.org/api/updates#updatemessageid-updates>.
///Related protocol documentation: <https://core.telegram.org/constructor/decryptedMessageActionSetMessageTTL>.
///Related protocol documentation: <https://core.telegram.org/mtproto/TL-combinators#conditional-fields>.
///Related protocol documentation: <https://core.telegram.org/type/%23>.
///Related protocol documentation: <https://core.telegram.org/type/DecryptedMessageMedia>.
///Related protocol documentation: <https://core.telegram.org/type/MessageEntity>.
///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
///Related protocol documentation: <https://core.telegram.org/type/bytes>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageConstructor {
    ///Flags, see TL conditional fields (added in layer 45)
    pub flags: u32,
    ///TL field `no_webpage`.
    pub no_webpage: Option<Box<True>>,
    ///TL field `silent`.
    pub silent: Option<Box<True>>,
    ///Random message ID, assigned by the author of message. Must be equal to the ID passed to sending method. See here » for more info on random ID deduplication and updateMessageID mapping.
    pub random_id: i64,
    ///Message lifetime. Has higher priority than decryptedMessageActionSetMessageTTL . Parameter added in Layer 17.
    pub ttl: i32,
    ///Message text
    pub message: String,
    ///Media content
    pub media: Option<Box<DecryptedMessageMedia>>,
    ///Message entities for styled text (parameter added in layer 45)
    pub entities: Option<Box<Vector<Box<MessageEntity>>>>,
    ///Specifies the ID of the inline bot that generated the message (parameter added in layer 45)
    pub via_bot_name: Option<String>,
    ///Random message ID of the message this message replies to (parameter added in layer 45)
    pub reply_to_random_id: Option<i64>,
    ///Random group ID, assigned by the author of message. Multiple encrypted messages with a photo attached and with the same group ID indicate an album or grouped media (parameter added in layer 45)
    pub grouped_id: Option<i64>,
}
impl DecryptedMessageConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x91cc_4674;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessage";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessage";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(73);
    /// Mask for `flags` bit 1 controlling `no_webpage`.
    pub const NO_WEBPAGE_FLAG: u32 = 1_u32 << 1;
    /// Mask for `flags` bit 5 controlling `silent`.
    pub const SILENT_FLAG: u32 = 1_u32 << 5;
    /// Mask for `flags` bit 9 controlling `media`.
    pub const MEDIA_FLAG: u32 = 1_u32 << 9;
    /// Mask for `flags` bit 7 controlling `entities`.
    pub const ENTITIES_FLAG: u32 = 1_u32 << 7;
    /// Mask for `flags` bit 11 controlling `via_bot_name`.
    pub const VIA_BOT_NAME_FLAG: u32 = 1_u32 << 11;
    /// Mask for `flags` bit 3 controlling `reply_to_random_id`.
    pub const REPLY_TO_RANDOM_ID_FLAG: u32 = 1_u32 << 3;
    /// Mask for `flags` bit 17 controlling `grouped_id`.
    pub const GROUPED_ID_FLAG: u32 = 1_u32 << 17;
}
impl crate::codec::TlEncode for DecryptedMessageConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.flags, encoder)?;
        if crate::codec::flag_is_set(self.flags, 1) != self.no_webpage.is_some() {
            return Err(crate::codec::Error::InconsistentFlag {
                field: "no_webpage",
                flags: "flags",
                bit: 1,
            });
        }
        if crate::codec::flag_is_set(self.flags, 5) != self.silent.is_some() {
            return Err(crate::codec::Error::InconsistentFlag {
                field: "silent",
                flags: "flags",
                bit: 5,
            });
        }
        crate::codec::TlEncode::encode(&self.random_id, encoder)?;
        crate::codec::TlEncode::encode(&self.ttl, encoder)?;
        crate::codec::TlEncode::encode(&self.message, encoder)?;
        if crate::codec::flag_is_set(self.flags, 9) != self.media.is_some() {
            return Err(crate::codec::Error::InconsistentFlag {
                field: "media",
                flags: "flags",
                bit: 9,
            });
        }
        if let Some(value) = &self.media {
            crate::codec::TlEncode::encode(value, encoder)?;
        }
        if crate::codec::flag_is_set(self.flags, 7) != self.entities.is_some() {
            return Err(crate::codec::Error::InconsistentFlag {
                field: "entities",
                flags: "flags",
                bit: 7,
            });
        }
        if let Some(value) = &self.entities {
            crate::codec::TlEncode::encode(value, encoder)?;
        }
        if crate::codec::flag_is_set(self.flags, 11) != self.via_bot_name.is_some() {
            return Err(crate::codec::Error::InconsistentFlag {
                field: "via_bot_name",
                flags: "flags",
                bit: 11,
            });
        }
        if let Some(value) = &self.via_bot_name {
            crate::codec::TlEncode::encode(value, encoder)?;
        }
        if crate::codec::flag_is_set(self.flags, 3) != self.reply_to_random_id.is_some()
        {
            return Err(crate::codec::Error::InconsistentFlag {
                field: "reply_to_random_id",
                flags: "flags",
                bit: 3,
            });
        }
        if let Some(value) = &self.reply_to_random_id {
            crate::codec::TlEncode::encode(value, encoder)?;
        }
        if crate::codec::flag_is_set(self.flags, 17) != self.grouped_id.is_some() {
            return Err(crate::codec::Error::InconsistentFlag {
                field: "grouped_id",
                flags: "flags",
                bit: 17,
            });
        }
        if let Some(value) = &self.grouped_id {
            crate::codec::TlEncode::encode(value, encoder)?;
        }
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let flags = crate::codec::TlDecode::decode(decoder)?;
        let no_webpage = if crate::codec::flag_is_set(flags, 1) {
            Some(crate::codec::BareDecode::decode_bare(decoder)?)
        } else {
            None
        };
        let silent = if crate::codec::flag_is_set(flags, 5) {
            Some(crate::codec::BareDecode::decode_bare(decoder)?)
        } else {
            None
        };
        let random_id = crate::codec::TlDecode::decode(decoder)?;
        let ttl = crate::codec::TlDecode::decode(decoder)?;
        let message = crate::codec::TlDecode::decode(decoder)?;
        let media = if crate::codec::flag_is_set(flags, 9) {
            Some(crate::codec::TlDecode::decode(decoder)?)
        } else {
            None
        };
        let entities = if crate::codec::flag_is_set(flags, 7) {
            Some(crate::codec::TlDecode::decode(decoder)?)
        } else {
            None
        };
        let via_bot_name = if crate::codec::flag_is_set(flags, 11) {
            Some(crate::codec::TlDecode::decode(decoder)?)
        } else {
            None
        };
        let reply_to_random_id = if crate::codec::flag_is_set(flags, 3) {
            Some(crate::codec::TlDecode::decode(decoder)?)
        } else {
            None
        };
        let grouped_id = if crate::codec::flag_is_set(flags, 17) {
            Some(crate::codec::TlDecode::decode(decoder)?)
        } else {
            None
        };
        Ok(Self {
            flags,
            no_webpage,
            silent,
            random_id,
            ttl,
            message,
            media,
            entities,
            via_bot_name,
            reply_to_random_id,
            grouped_id,
        })
    }
}
///@introduced 17
///Contents of an encrypted service message.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageService>.
///Related protocol documentation: <https://core.telegram.org/api/updates#updatemessageid-updates>.
///Related protocol documentation: <https://core.telegram.org/type/DecryptedMessageAction>.
///Related protocol documentation: <https://core.telegram.org/type/bytes>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageServiceConstructor {
    ///Random message ID, assigned by the message author. Must be equal to the ID passed to the sending method. See here » for more info on random ID deduplication and updateMessageID mapping.
    pub random_id: i64,
    ///Action relevant to the service message
    pub action: Box<DecryptedMessageAction>,
}
impl DecryptedMessageServiceConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x7316_4160;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageService";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessage";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for DecryptedMessageServiceConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.random_id, encoder)?;
        crate::codec::TlEncode::encode(&self.action, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageServiceConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageServiceConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let random_id = crate::codec::TlDecode::decode(decoder)?;
        let action = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { random_id, action })
    }
}
///@introduced 8
///Empty constructor, no media content.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaEmpty>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageMediaEmptyConstructor {}
impl DecryptedMessageMediaEmptyConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x089f_5c4a;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaEmpty";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
}
impl crate::codec::TlEncode for DecryptedMessageMediaEmptyConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageMediaEmptyConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageMediaEmptyConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 45
///Photo attached to an encrypted message.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaPhoto>.
///Related protocol documentation: <https://core.telegram.org/type/bytes>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageMediaPhotoConstructor {
    ///Content of thumbnail file (JPEGfile, quality 55, set in a square 90x90)
    pub thumb: Vec<u8>,
    ///Thumbnail width
    pub thumb_w: i32,
    ///Thumbnail height
    pub thumb_h: i32,
    ///Photo width
    pub w: i32,
    ///Photo height
    pub h: i32,
    ///Size of the photo in bytes
    pub size: i32,
    ///Key to decrypt an attached file with a full version
    pub key: Vec<u8>,
    ///Initialization vector
    pub iv: Vec<u8>,
    ///Caption
    pub caption: String,
}
impl DecryptedMessageMediaPhotoConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xf1fa_8d78;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaPhoto";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for DecryptedMessageMediaPhotoConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::encode_bytes(&self.thumb, encoder)?;
        crate::codec::TlEncode::encode(&self.thumb_w, encoder)?;
        crate::codec::TlEncode::encode(&self.thumb_h, encoder)?;
        crate::codec::TlEncode::encode(&self.w, encoder)?;
        crate::codec::TlEncode::encode(&self.h, encoder)?;
        crate::codec::TlEncode::encode(&self.size, encoder)?;
        crate::codec::encode_bytes(&self.key, encoder)?;
        crate::codec::encode_bytes(&self.iv, encoder)?;
        crate::codec::TlEncode::encode(&self.caption, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageMediaPhotoConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageMediaPhotoConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let thumb = crate::codec::decode_bytes(decoder)?;
        let thumb_w = crate::codec::TlDecode::decode(decoder)?;
        let thumb_h = crate::codec::TlDecode::decode(decoder)?;
        let w = crate::codec::TlDecode::decode(decoder)?;
        let h = crate::codec::TlDecode::decode(decoder)?;
        let size = crate::codec::TlDecode::decode(decoder)?;
        let key = crate::codec::decode_bytes(decoder)?;
        let iv = crate::codec::decode_bytes(decoder)?;
        let caption = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            thumb,
            thumb_w,
            thumb_h,
            w,
            h,
            size,
            key,
            iv,
            caption,
        })
    }
}
///@introduced 45
///Video attached to an encrypted message.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaVideo>.
///Related protocol documentation: <https://core.telegram.org/type/bytes>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageMediaVideoConstructor {
    ///Content of thumbnail file (JPEG file, quality 55, set in a square 90x90)
    pub thumb: Vec<u8>,
    ///Thumbnail width
    pub thumb_w: i32,
    ///Thumbnail height
    pub thumb_h: i32,
    ///Duration of video in seconds
    pub duration: i32,
    ///MIME-type of the video file Parameter added in Layer 17.
    pub mime_type: String,
    ///Image width
    pub w: i32,
    ///Image height
    pub h: i32,
    ///File size
    pub size: i32,
    ///Key to decrypt the attached video file
    pub key: Vec<u8>,
    ///Initialization vector
    pub iv: Vec<u8>,
    ///Caption
    pub caption: String,
}
impl DecryptedMessageMediaVideoConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x970c_8c0e;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaVideo";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for DecryptedMessageMediaVideoConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::encode_bytes(&self.thumb, encoder)?;
        crate::codec::TlEncode::encode(&self.thumb_w, encoder)?;
        crate::codec::TlEncode::encode(&self.thumb_h, encoder)?;
        crate::codec::TlEncode::encode(&self.duration, encoder)?;
        crate::codec::TlEncode::encode(&self.mime_type, encoder)?;
        crate::codec::TlEncode::encode(&self.w, encoder)?;
        crate::codec::TlEncode::encode(&self.h, encoder)?;
        crate::codec::TlEncode::encode(&self.size, encoder)?;
        crate::codec::encode_bytes(&self.key, encoder)?;
        crate::codec::encode_bytes(&self.iv, encoder)?;
        crate::codec::TlEncode::encode(&self.caption, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageMediaVideoConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageMediaVideoConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let thumb = crate::codec::decode_bytes(decoder)?;
        let thumb_w = crate::codec::TlDecode::decode(decoder)?;
        let thumb_h = crate::codec::TlDecode::decode(decoder)?;
        let duration = crate::codec::TlDecode::decode(decoder)?;
        let mime_type = crate::codec::TlDecode::decode(decoder)?;
        let w = crate::codec::TlDecode::decode(decoder)?;
        let h = crate::codec::TlDecode::decode(decoder)?;
        let size = crate::codec::TlDecode::decode(decoder)?;
        let key = crate::codec::decode_bytes(decoder)?;
        let iv = crate::codec::decode_bytes(decoder)?;
        let caption = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            thumb,
            thumb_w,
            thumb_h,
            duration,
            mime_type,
            w,
            h,
            size,
            key,
            iv,
            caption,
        })
    }
}
///@introduced 8
///GeoPoint attached to an encrypted message.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaGeoPoint>.
///Related protocol documentation: <https://core.telegram.org/type/double>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageMediaGeoPointConstructor {
    ///Latitude of point
    pub lat: f64,
    ///Longitude of point
    pub long: f64,
}
impl DecryptedMessageMediaGeoPointConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x3548_0a59;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaGeoPoint";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
}
impl crate::codec::TlEncode for DecryptedMessageMediaGeoPointConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.lat, encoder)?;
        crate::codec::TlEncode::encode(&self.long, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageMediaGeoPointConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageMediaGeoPointConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let lat = crate::codec::TlDecode::decode(decoder)?;
        let long = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { lat, long })
    }
}
///@introduced 8
///Contact attached to an encrypted message.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaContact>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageMediaContactConstructor {
    ///Phone number
    pub phone_number: String,
    ///Contact's first name
    pub first_name: String,
    ///Contact's last name
    pub last_name: String,
    ///Telegram User ID of signed-up contact
    pub user_id: i32,
}
impl DecryptedMessageMediaContactConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x588a_0a97;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaContact";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
}
impl crate::codec::TlEncode for DecryptedMessageMediaContactConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.phone_number, encoder)?;
        crate::codec::TlEncode::encode(&self.first_name, encoder)?;
        crate::codec::TlEncode::encode(&self.last_name, encoder)?;
        crate::codec::TlEncode::encode(&self.user_id, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageMediaContactConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageMediaContactConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let phone_number = crate::codec::TlDecode::decode(decoder)?;
        let first_name = crate::codec::TlDecode::decode(decoder)?;
        let last_name = crate::codec::TlDecode::decode(decoder)?;
        let user_id = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            phone_number,
            first_name,
            last_name,
            user_id,
        })
    }
}
///@introduced 8
///Setting of a message lifetime after reading.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionSetMessageTTL>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageActionSetMessageTtlConstructor {
    ///Lifetime in seconds
    pub ttl_seconds: i32,
}
impl DecryptedMessageActionSetMessageTtlConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xa173_3aec;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageActionSetMessageTTL";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
}
impl crate::codec::TlEncode for DecryptedMessageActionSetMessageTtlConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.ttl_seconds, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageActionSetMessageTtlConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageActionSetMessageTtlConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let ttl_seconds = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { ttl_seconds })
    }
}
///@introduced 45
///Document attached to a message in a secret chat.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaDocument>.
///Related protocol documentation: <https://core.telegram.org/type/DocumentAttribute>.
///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
///Related protocol documentation: <https://core.telegram.org/type/bytes>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageMediaDocumentConstructor {
    ///Thumbnail-file contents (JPEG-file, quality 55, set in a 90x90 square)
    pub thumb: Vec<u8>,
    ///Thumbnail width
    pub thumb_w: i32,
    ///Thumbnail height
    pub thumb_h: i32,
    ///File MIME-type
    pub mime_type: String,
    ///Document size ( int on layer &lt;143, long on layer &gt;=143)
    pub size: i32,
    ///Key to decrypt the attached document file
    pub key: Vec<u8>,
    ///Initialization
    pub iv: Vec<u8>,
    ///Document attributes for media types
    pub attributes: Box<Vector<Box<DocumentAttribute>>>,
    ///Caption
    pub caption: String,
}
impl DecryptedMessageMediaDocumentConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x7afe_8ae2;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaDocument";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for DecryptedMessageMediaDocumentConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::encode_bytes(&self.thumb, encoder)?;
        crate::codec::TlEncode::encode(&self.thumb_w, encoder)?;
        crate::codec::TlEncode::encode(&self.thumb_h, encoder)?;
        crate::codec::TlEncode::encode(&self.mime_type, encoder)?;
        crate::codec::TlEncode::encode(&self.size, encoder)?;
        crate::codec::encode_bytes(&self.key, encoder)?;
        crate::codec::encode_bytes(&self.iv, encoder)?;
        crate::codec::TlEncode::encode(&self.attributes, encoder)?;
        crate::codec::TlEncode::encode(&self.caption, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageMediaDocumentConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageMediaDocumentConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let thumb = crate::codec::decode_bytes(decoder)?;
        let thumb_w = crate::codec::TlDecode::decode(decoder)?;
        let thumb_h = crate::codec::TlDecode::decode(decoder)?;
        let mime_type = crate::codec::TlDecode::decode(decoder)?;
        let size = crate::codec::TlDecode::decode(decoder)?;
        let key = crate::codec::decode_bytes(decoder)?;
        let iv = crate::codec::decode_bytes(decoder)?;
        let attributes = crate::codec::TlDecode::decode(decoder)?;
        let caption = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            thumb,
            thumb_w,
            thumb_h,
            mime_type,
            size,
            key,
            iv,
            attributes,
            caption,
        })
    }
}
///@introduced 17
///Audio file attached to a secret chat message.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaAudio>.
///Related protocol documentation: <https://core.telegram.org/type/bytes>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageMediaAudioConstructor {
    ///Audio duration in seconds
    pub duration: i32,
    ///MIME-type of the audio file Parameter added in Layer 13.
    pub mime_type: String,
    ///File size
    pub size: i32,
    ///Key to decrypt the attached media file
    pub key: Vec<u8>,
    ///Initialization vector
    pub iv: Vec<u8>,
}
impl DecryptedMessageMediaAudioConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x57e0_a9cb;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaAudio";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for DecryptedMessageMediaAudioConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.duration, encoder)?;
        crate::codec::TlEncode::encode(&self.mime_type, encoder)?;
        crate::codec::TlEncode::encode(&self.size, encoder)?;
        crate::codec::encode_bytes(&self.key, encoder)?;
        crate::codec::encode_bytes(&self.iv, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageMediaAudioConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageMediaAudioConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let duration = crate::codec::TlDecode::decode(decoder)?;
        let mime_type = crate::codec::TlDecode::decode(decoder)?;
        let size = crate::codec::TlDecode::decode(decoder)?;
        let key = crate::codec::decode_bytes(decoder)?;
        let iv = crate::codec::decode_bytes(decoder)?;
        Ok(Self {
            duration,
            mime_type,
            size,
            key,
            iv,
        })
    }
}
///@introduced 8
///Messages marked as read.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionReadMessages>.
///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageActionReadMessagesConstructor {
    ///List of message IDs
    pub random_ids: Box<Vector<i64>>,
}
impl DecryptedMessageActionReadMessagesConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x0c4f_40be;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageActionReadMessages";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
}
impl crate::codec::TlEncode for DecryptedMessageActionReadMessagesConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.random_ids, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageActionReadMessagesConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageActionReadMessagesConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let random_ids = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { random_ids })
    }
}
///@introduced 8
///Deleted messages.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionDeleteMessages>.
///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageActionDeleteMessagesConstructor {
    ///List of deleted message IDs
    pub random_ids: Box<Vector<i64>>,
}
impl DecryptedMessageActionDeleteMessagesConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x6561_4304;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageActionDeleteMessages";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
}
impl crate::codec::TlEncode for DecryptedMessageActionDeleteMessagesConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.random_ids, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageActionDeleteMessagesConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageActionDeleteMessagesConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let random_ids = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { random_ids })
    }
}
///@introduced 8
///A screenshot was taken.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionScreenshotMessages>.
///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageActionScreenshotMessagesConstructor {
    ///List of affected message ids (that appeared on the screenshot)
    pub random_ids: Box<Vector<i64>>,
}
impl DecryptedMessageActionScreenshotMessagesConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x8ac1_f475;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageActionScreenshotMessages";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
}
impl crate::codec::TlEncode for DecryptedMessageActionScreenshotMessagesConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.random_ids, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageActionScreenshotMessagesConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageActionScreenshotMessagesConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let random_ids = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { random_ids })
    }
}
///@introduced 8
///The entire message history has been deleted.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionFlushHistory>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageActionFlushHistoryConstructor {}
impl DecryptedMessageActionFlushHistoryConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x6719_e45c;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageActionFlushHistory";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
}
impl crate::codec::TlEncode for DecryptedMessageActionFlushHistoryConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageActionFlushHistoryConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageActionFlushHistoryConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 17
///Sets the layer number for the contents of an encrypted message.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageLayer>.
///Related protocol documentation: <https://core.telegram.org/constructor/decryptedMessage>.
///Related protocol documentation: <https://core.telegram.org/type/DecryptedMessage>.
///Related protocol documentation: <https://core.telegram.org/type/bytes>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageLayerConstructor {
    ///Set of random bytes to prevent content recognition in short encrypted messages. Clients are required to check that there are at least 15 random bytes included in each message. Messages with less than 15 random bytes must be ignored. Parameter moved here from decryptedMessage in Layer 17.
    pub random_bytes: Vec<u8>,
    ///Layer number. Mimimal value - 17 (the layer in which the constructor was added).
    pub layer: i32,
    ///2x the number of messages in the sender's inbox (including deleted and service messages), incremented by 1 if current user was not the chat creator Parameter added in Layer 17.
    pub in_seq_no: i32,
    ///2x the number of messages in the recipient's inbox (including deleted and service messages), incremented by 1 if current user was the chat creator Parameter added in Layer 17.
    pub out_seq_no: i32,
    ///The content of message itself
    pub message: Box<DecryptedMessage>,
}
impl DecryptedMessageLayerConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x1be3_1789;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageLayer";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageLayer";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for DecryptedMessageLayerConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::encode_bytes(&self.random_bytes, encoder)?;
        crate::codec::TlEncode::encode(&self.layer, encoder)?;
        crate::codec::TlEncode::encode(&self.in_seq_no, encoder)?;
        crate::codec::TlEncode::encode(&self.out_seq_no, encoder)?;
        crate::codec::TlEncode::encode(&self.message, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageLayerConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageLayerConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let random_bytes = crate::codec::decode_bytes(decoder)?;
        let layer = crate::codec::TlDecode::decode(decoder)?;
        let in_seq_no = crate::codec::TlDecode::decode(decoder)?;
        let out_seq_no = crate::codec::TlDecode::decode(decoder)?;
        let message = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            random_bytes,
            layer,
            in_seq_no,
            out_seq_no,
            message,
        })
    }
}
///@introduced 17
///User is typing.
///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageTypingAction>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct SendMessageTypingActionConstructor {}
impl SendMessageTypingActionConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x16bf_744e;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "sendMessageTypingAction";
    /// Original TL result expression.
    pub const RESULT: &'static str = "SendMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for SendMessageTypingActionConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for SendMessageTypingActionConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for SendMessageTypingActionConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 17
///Invalidate all previous action updates. E.g. when user deletes entered text or aborts a video upload.
///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageCancelAction>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct SendMessageCancelActionConstructor {}
impl SendMessageCancelActionConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xfd5e_c8f5;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "sendMessageCancelAction";
    /// Original TL result expression.
    pub const RESULT: &'static str = "SendMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for SendMessageCancelActionConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for SendMessageCancelActionConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for SendMessageCancelActionConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 17
///User is recording a video.
///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageRecordVideoAction>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct SendMessageRecordVideoActionConstructor {}
impl SendMessageRecordVideoActionConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xa187_d66f;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "sendMessageRecordVideoAction";
    /// Original TL result expression.
    pub const RESULT: &'static str = "SendMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for SendMessageRecordVideoActionConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for SendMessageRecordVideoActionConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for SendMessageRecordVideoActionConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 17
///User is uploading a video.
///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageUploadVideoAction>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct SendMessageUploadVideoActionConstructor {}
impl SendMessageUploadVideoActionConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x9204_2ff7;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "sendMessageUploadVideoAction";
    /// Original TL result expression.
    pub const RESULT: &'static str = "SendMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for SendMessageUploadVideoActionConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for SendMessageUploadVideoActionConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for SendMessageUploadVideoActionConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 17
///User is recording a voice message.
///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageRecordAudioAction>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct SendMessageRecordAudioActionConstructor {}
impl SendMessageRecordAudioActionConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xd52f_73f7;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "sendMessageRecordAudioAction";
    /// Original TL result expression.
    pub const RESULT: &'static str = "SendMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for SendMessageRecordAudioActionConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for SendMessageRecordAudioActionConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for SendMessageRecordAudioActionConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 17
///User is uploading a voice message.
///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageUploadAudioAction>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct SendMessageUploadAudioActionConstructor {}
impl SendMessageUploadAudioActionConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xe6ac_8a6f;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "sendMessageUploadAudioAction";
    /// Original TL result expression.
    pub const RESULT: &'static str = "SendMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for SendMessageUploadAudioActionConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for SendMessageUploadAudioActionConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for SendMessageUploadAudioActionConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 17
///User is uploading a photo.
///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageUploadPhotoAction>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct SendMessageUploadPhotoActionConstructor {}
impl SendMessageUploadPhotoActionConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x990a_3c1a;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "sendMessageUploadPhotoAction";
    /// Original TL result expression.
    pub const RESULT: &'static str = "SendMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for SendMessageUploadPhotoActionConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for SendMessageUploadPhotoActionConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for SendMessageUploadPhotoActionConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 17
///User is uploading a file.
///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageUploadDocumentAction>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct SendMessageUploadDocumentActionConstructor {}
impl SendMessageUploadDocumentActionConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x8fae_e98e;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "sendMessageUploadDocumentAction";
    /// Original TL result expression.
    pub const RESULT: &'static str = "SendMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for SendMessageUploadDocumentActionConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for SendMessageUploadDocumentActionConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for SendMessageUploadDocumentActionConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 17
///User is selecting a location to share.
///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageGeoLocationAction>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct SendMessageGeoLocationActionConstructor {}
impl SendMessageGeoLocationActionConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x176f_8ba1;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "sendMessageGeoLocationAction";
    /// Original TL result expression.
    pub const RESULT: &'static str = "SendMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for SendMessageGeoLocationActionConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for SendMessageGeoLocationActionConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for SendMessageGeoLocationActionConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 17
///User is selecting a contact to share.
///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageChooseContactAction>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct SendMessageChooseContactActionConstructor {}
impl SendMessageChooseContactActionConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x628c_bc6f;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "sendMessageChooseContactAction";
    /// Original TL result expression.
    pub const RESULT: &'static str = "SendMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for SendMessageChooseContactActionConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for SendMessageChooseContactActionConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for SendMessageChooseContactActionConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 17
///Request for the other party in a Secret Chat to automatically resend a contiguous range of previously sent messages, as explained in Sequence number is Secret Chats .
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionResend>.
///Related protocol documentation: <https://core.telegram.org/api/end-to-end/seq_no>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageActionResendConstructor {
    ///out_seq_no of the first message to be resent, with correct parity
    pub start_seq_no: i32,
    ///out_seq_no of the last message to be resent, with same parity.
    pub end_seq_no: i32,
}
impl DecryptedMessageActionResendConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x5111_10b0;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageActionResend";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for DecryptedMessageActionResendConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.start_seq_no, encoder)?;
        crate::codec::TlEncode::encode(&self.end_seq_no, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageActionResendConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageActionResendConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let start_seq_no = crate::codec::TlDecode::decode(decoder)?;
        let end_seq_no = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { start_seq_no, end_seq_no })
    }
}
///@introduced 17
///A notification stating the API layer that is used by the client. You should use your current layer and take notice of the layer used on the other side of a conversation when sending messages.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionNotifyLayer>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageActionNotifyLayerConstructor {
    ///Layer number, must be 17 or higher (this constructor was introduced in Layer 17.
    pub layer: i32,
}
impl DecryptedMessageActionNotifyLayerConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xf304_8883;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageActionNotifyLayer";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for DecryptedMessageActionNotifyLayerConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.layer, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageActionNotifyLayerConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageActionNotifyLayerConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let layer = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { layer })
    }
}
///@introduced 17
///User is preparing a message: typing, recording, uploading, etc.
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionTyping>.
///Related protocol documentation: <https://core.telegram.org/type/SendMessageAction>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageActionTypingConstructor {
    ///Type of action
    pub action: Box<SendMessageAction>,
}
impl DecryptedMessageActionTypingConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xccb2_7641;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageActionTyping";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(17);
}
impl crate::codec::TlEncode for DecryptedMessageActionTypingConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.action, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageActionTypingConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageActionTypingConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let action = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { action })
    }
}
///@introduced 20
///Request rekeying, see rekeying process
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionRequestKey>.
///Related protocol documentation: <https://core.telegram.org/api/end-to-end/pfs>.
///Related protocol documentation: <https://core.telegram.org/type/bytes>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageActionRequestKeyConstructor {
    ///Exchange ID
    pub exchange_id: i64,
    ///g_a, see rekeying process
    pub g_a: Vec<u8>,
}
impl DecryptedMessageActionRequestKeyConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xf3c9_611b;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageActionRequestKey";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(20);
}
impl crate::codec::TlEncode for DecryptedMessageActionRequestKeyConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.exchange_id, encoder)?;
        crate::codec::encode_bytes(&self.g_a, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageActionRequestKeyConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageActionRequestKeyConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let exchange_id = crate::codec::TlDecode::decode(decoder)?;
        let g_a = crate::codec::decode_bytes(decoder)?;
        Ok(Self { exchange_id, g_a })
    }
}
///@introduced 20
///Accept new key
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionAcceptKey>.
///Related protocol documentation: <https://core.telegram.org/api/end-to-end/pfs>.
///Related protocol documentation: <https://core.telegram.org/type/bytes>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageActionAcceptKeyConstructor {
    ///Exchange ID
    pub exchange_id: i64,
    ///B parameter, see rekeying process
    pub g_b: Vec<u8>,
    ///Key fingerprint, see rekeying process
    pub key_fingerprint: i64,
}
impl DecryptedMessageActionAcceptKeyConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x6fe1_735b;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageActionAcceptKey";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(20);
}
impl crate::codec::TlEncode for DecryptedMessageActionAcceptKeyConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.exchange_id, encoder)?;
        crate::codec::encode_bytes(&self.g_b, encoder)?;
        crate::codec::TlEncode::encode(&self.key_fingerprint, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageActionAcceptKeyConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageActionAcceptKeyConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let exchange_id = crate::codec::TlDecode::decode(decoder)?;
        let g_b = crate::codec::decode_bytes(decoder)?;
        let key_fingerprint = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            exchange_id,
            g_b,
            key_fingerprint,
        })
    }
}
///@introduced 20
///Abort rekeying
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionAbortKey>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageActionAbortKeyConstructor {
    ///Exchange ID
    pub exchange_id: i64,
}
impl DecryptedMessageActionAbortKeyConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xdd05_ec6b;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageActionAbortKey";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(20);
}
impl crate::codec::TlEncode for DecryptedMessageActionAbortKeyConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.exchange_id, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageActionAbortKeyConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageActionAbortKeyConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let exchange_id = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { exchange_id })
    }
}
///@introduced 20
///Commit new key, see rekeying process
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionCommitKey>.
///Related protocol documentation: <https://core.telegram.org/api/end-to-end/pfs>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageActionCommitKeyConstructor {
    ///Exchange ID, see rekeying process
    pub exchange_id: i64,
    ///Key fingerprint, see rekeying process
    pub key_fingerprint: i64,
}
impl DecryptedMessageActionCommitKeyConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xec2e_0b9b;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageActionCommitKey";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(20);
}
impl crate::codec::TlEncode for DecryptedMessageActionCommitKeyConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.exchange_id, encoder)?;
        crate::codec::TlEncode::encode(&self.key_fingerprint, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageActionCommitKeyConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageActionCommitKeyConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let exchange_id = crate::codec::TlDecode::decode(decoder)?;
        let key_fingerprint = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            exchange_id,
            key_fingerprint,
        })
    }
}
///@introduced 20
///NOOP action
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageActionNoop>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageActionNoopConstructor {}
impl DecryptedMessageActionNoopConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xa82f_dd63;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageActionNoop";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(20);
}
impl crate::codec::TlEncode for DecryptedMessageActionNoopConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageActionNoopConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageActionNoopConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 23
///Defines the width and height of an image uploaded as document
///Official Telegram documentation: <https://core.telegram.org/constructor/documentAttributeImageSize>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DocumentAttributeImageSizeConstructor {
    ///Width of image
    pub w: i32,
    ///Height of image
    pub h: i32,
}
impl DocumentAttributeImageSizeConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x6c37_c15c;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "documentAttributeImageSize";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DocumentAttribute";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(23);
}
impl crate::codec::TlEncode for DocumentAttributeImageSizeConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.w, encoder)?;
        crate::codec::TlEncode::encode(&self.h, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DocumentAttributeImageSizeConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DocumentAttributeImageSizeConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let w = crate::codec::TlDecode::decode(decoder)?;
        let h = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { w, h })
    }
}
///@introduced 23
///Defines an animated GIF
///Official Telegram documentation: <https://core.telegram.org/constructor/documentAttributeAnimated>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DocumentAttributeAnimatedConstructor {}
impl DocumentAttributeAnimatedConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x11b5_8939;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "documentAttributeAnimated";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DocumentAttribute";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(23);
}
impl crate::codec::TlEncode for DocumentAttributeAnimatedConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for DocumentAttributeAnimatedConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DocumentAttributeAnimatedConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 45
///Defines a sticker
///Official Telegram documentation: <https://core.telegram.org/constructor/documentAttributeSticker>.
///Related protocol documentation: <https://core.telegram.org/constructor/true>.
///Related protocol documentation: <https://core.telegram.org/mtproto/TL-combinators#conditional-fields>.
///Related protocol documentation: <https://core.telegram.org/type/%23>.
///Related protocol documentation: <https://core.telegram.org/type/InputStickerSet>.
///Related protocol documentation: <https://core.telegram.org/type/MaskCoords>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DocumentAttributeStickerConstructor {
    ///Alternative emoji representation of sticker
    pub alt: String,
    ///Associated stickerset
    pub stickerset: Box<InputStickerSet>,
}
impl DocumentAttributeStickerConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x3a55_6302;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "documentAttributeSticker";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DocumentAttribute";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for DocumentAttributeStickerConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.alt, encoder)?;
        crate::codec::TlEncode::encode(&self.stickerset, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DocumentAttributeStickerConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DocumentAttributeStickerConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let alt = crate::codec::TlDecode::decode(decoder)?;
        let stickerset = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { alt, stickerset })
    }
}
///@introduced 66
///Defines a video
///Official Telegram documentation: <https://core.telegram.org/constructor/documentAttributeVideo>.
///Related protocol documentation: <https://core.telegram.org/api/stories>.
///Related protocol documentation: <https://core.telegram.org/constructor/true>.
///Related protocol documentation: <https://core.telegram.org/mtproto/TL-combinators#conditional-fields>.
///Related protocol documentation: <https://core.telegram.org/type/%23>.
///Related protocol documentation: <https://core.telegram.org/type/double>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DocumentAttributeVideoConstructor {
    ///Flags, see TL conditional fields
    pub flags: u32,
    ///Whether this is a round video
    pub round_message: Option<Box<True>>,
    ///Duration in seconds
    pub duration: i32,
    ///Video width
    pub w: i32,
    ///Video height
    pub h: i32,
}
impl DocumentAttributeVideoConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x0ef0_2ce6;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "documentAttributeVideo";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DocumentAttribute";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(66);
    /// Mask for `flags` bit 0 controlling `round_message`.
    pub const ROUND_MESSAGE_FLAG: u32 = 1_u32 << 0;
}
impl crate::codec::TlEncode for DocumentAttributeVideoConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.flags, encoder)?;
        if crate::codec::flag_is_set(self.flags, 0) != self.round_message.is_some() {
            return Err(crate::codec::Error::InconsistentFlag {
                field: "round_message",
                flags: "flags",
                bit: 0,
            });
        }
        crate::codec::TlEncode::encode(&self.duration, encoder)?;
        crate::codec::TlEncode::encode(&self.w, encoder)?;
        crate::codec::TlEncode::encode(&self.h, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DocumentAttributeVideoConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DocumentAttributeVideoConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let flags = crate::codec::TlDecode::decode(decoder)?;
        let round_message = if crate::codec::flag_is_set(flags, 0) {
            Some(crate::codec::BareDecode::decode_bare(decoder)?)
        } else {
            None
        };
        let duration = crate::codec::TlDecode::decode(decoder)?;
        let w = crate::codec::TlDecode::decode(decoder)?;
        let h = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            flags,
            round_message,
            duration,
            w,
            h,
        })
    }
}
///@introduced 46
///Represents an audio file
///Official Telegram documentation: <https://core.telegram.org/constructor/documentAttributeAudio>.
///Related protocol documentation: <https://core.telegram.org/constructor/true>.
///Related protocol documentation: <https://core.telegram.org/mtproto/TL-combinators#conditional-fields>.
///Related protocol documentation: <https://core.telegram.org/type/%23>.
///Related protocol documentation: <https://core.telegram.org/type/bytes>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
///Related protocol documentation: <https://github.com/DrKLO/Telegram/blob/4d7a3a40c88ce28a211aa2814e9569389fc40567/TMessagesProj/jni/audio.c#L562>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DocumentAttributeAudioConstructor {
    ///Flags, see TL conditional fields
    pub flags: u32,
    ///Whether this is a voice message
    pub voice: Option<Box<True>>,
    ///Duration in seconds
    pub duration: i32,
    ///Name of song
    pub title: Option<String>,
    ///Performer
    pub performer: Option<String>,
    ///Waveform: consists in a series of bitpacked 5-bit values. Example implementation: android .
    pub waveform: Option<Vec<u8>>,
}
impl DocumentAttributeAudioConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x9852_f9c6;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "documentAttributeAudio";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DocumentAttribute";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(46);
    /// Mask for `flags` bit 10 controlling `voice`.
    pub const VOICE_FLAG: u32 = 1_u32 << 10;
    /// Mask for `flags` bit 0 controlling `title`.
    pub const TITLE_FLAG: u32 = 1_u32 << 0;
    /// Mask for `flags` bit 1 controlling `performer`.
    pub const PERFORMER_FLAG: u32 = 1_u32 << 1;
    /// Mask for `flags` bit 2 controlling `waveform`.
    pub const WAVEFORM_FLAG: u32 = 1_u32 << 2;
}
impl crate::codec::TlEncode for DocumentAttributeAudioConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.flags, encoder)?;
        if crate::codec::flag_is_set(self.flags, 10) != self.voice.is_some() {
            return Err(crate::codec::Error::InconsistentFlag {
                field: "voice",
                flags: "flags",
                bit: 10,
            });
        }
        crate::codec::TlEncode::encode(&self.duration, encoder)?;
        if crate::codec::flag_is_set(self.flags, 0) != self.title.is_some() {
            return Err(crate::codec::Error::InconsistentFlag {
                field: "title",
                flags: "flags",
                bit: 0,
            });
        }
        if let Some(value) = &self.title {
            crate::codec::TlEncode::encode(value, encoder)?;
        }
        if crate::codec::flag_is_set(self.flags, 1) != self.performer.is_some() {
            return Err(crate::codec::Error::InconsistentFlag {
                field: "performer",
                flags: "flags",
                bit: 1,
            });
        }
        if let Some(value) = &self.performer {
            crate::codec::TlEncode::encode(value, encoder)?;
        }
        if crate::codec::flag_is_set(self.flags, 2) != self.waveform.is_some() {
            return Err(crate::codec::Error::InconsistentFlag {
                field: "waveform",
                flags: "flags",
                bit: 2,
            });
        }
        if let Some(value) = &self.waveform {
            crate::codec::encode_bytes(value, encoder)?;
        }
        Ok(())
    }
}
impl crate::codec::Boxed for DocumentAttributeAudioConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DocumentAttributeAudioConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let flags = crate::codec::TlDecode::decode(decoder)?;
        let voice = if crate::codec::flag_is_set(flags, 10) {
            Some(crate::codec::BareDecode::decode_bare(decoder)?)
        } else {
            None
        };
        let duration = crate::codec::TlDecode::decode(decoder)?;
        let title = if crate::codec::flag_is_set(flags, 0) {
            Some(crate::codec::TlDecode::decode(decoder)?)
        } else {
            None
        };
        let performer = if crate::codec::flag_is_set(flags, 1) {
            Some(crate::codec::TlDecode::decode(decoder)?)
        } else {
            None
        };
        let waveform = if crate::codec::flag_is_set(flags, 2) {
            Some(crate::codec::decode_bytes(decoder)?)
        } else {
            None
        };
        Ok(Self {
            flags,
            voice,
            duration,
            title,
            performer,
            waveform,
        })
    }
}
///@introduced 23
///A simple document with a file name
///Official Telegram documentation: <https://core.telegram.org/constructor/documentAttributeFilename>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DocumentAttributeFilenameConstructor {
    ///The file name
    pub file_name: String,
}
impl DocumentAttributeFilenameConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x1559_0068;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "documentAttributeFilename";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DocumentAttribute";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(23);
}
impl crate::codec::TlEncode for DocumentAttributeFilenameConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.file_name, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DocumentAttributeFilenameConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DocumentAttributeFilenameConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let file_name = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { file_name })
    }
}
///@introduced 23
///Empty constructor. Image with this thumbnail is unavailable.
///Official Telegram documentation: <https://core.telegram.org/constructor/photoSizeEmpty>.
///Related protocol documentation: <https://core.telegram.org/api/files#photosizetype-values>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct PhotoSizeEmptyConstructor {
    ///PhotoSize.type value »
    pub type_: String,
}
impl PhotoSizeEmptyConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x0e17_e23c;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "photoSizeEmpty";
    /// Original TL result expression.
    pub const RESULT: &'static str = "PhotoSize";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(23);
}
impl crate::codec::TlEncode for PhotoSizeEmptyConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.type_, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for PhotoSizeEmptyConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for PhotoSizeEmptyConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let type_ = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { type_ })
    }
}
///@introduced 23
///Image description.
///Official Telegram documentation: <https://core.telegram.org/constructor/photoSize>.
///Related protocol documentation: <https://core.telegram.org/api/files#photosizetype-values>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct PhotoSizeConstructor {
    ///PhotoSize.type value »
    pub type_: String,
    ///TL field `location`.
    pub location: Box<FileLocation>,
    ///Image width
    pub w: i32,
    ///Image height
    pub h: i32,
    ///File size
    pub size: i32,
}
impl PhotoSizeConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x77bf_b61b;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "photoSize";
    /// Original TL result expression.
    pub const RESULT: &'static str = "PhotoSize";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(23);
}
impl crate::codec::TlEncode for PhotoSizeConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.type_, encoder)?;
        crate::codec::TlEncode::encode(&self.location, encoder)?;
        crate::codec::TlEncode::encode(&self.w, encoder)?;
        crate::codec::TlEncode::encode(&self.h, encoder)?;
        crate::codec::TlEncode::encode(&self.size, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for PhotoSizeConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for PhotoSizeConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let type_ = crate::codec::TlDecode::decode(decoder)?;
        let location = crate::codec::TlDecode::decode(decoder)?;
        let w = crate::codec::TlDecode::decode(decoder)?;
        let h = crate::codec::TlDecode::decode(decoder)?;
        let size = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            type_,
            location,
            w,
            h,
            size,
        })
    }
}
///@introduced 23
///Description of an image and its content.
///Official Telegram documentation: <https://core.telegram.org/constructor/photoCachedSize>.
///Related protocol documentation: <https://core.telegram.org/api/files#photosizetype-values>.
///Related protocol documentation: <https://core.telegram.org/type/bytes>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct PhotoCachedSizeConstructor {
    ///PhotoSize.type value »
    pub type_: String,
    ///TL field `location`.
    pub location: Box<FileLocation>,
    ///Image width
    pub w: i32,
    ///Image height
    pub h: i32,
    ///Binary data, file content
    pub bytes: Vec<u8>,
}
impl PhotoCachedSizeConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xe9a7_34fa;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "photoCachedSize";
    /// Original TL result expression.
    pub const RESULT: &'static str = "PhotoSize";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(23);
}
impl crate::codec::TlEncode for PhotoCachedSizeConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.type_, encoder)?;
        crate::codec::TlEncode::encode(&self.location, encoder)?;
        crate::codec::TlEncode::encode(&self.w, encoder)?;
        crate::codec::TlEncode::encode(&self.h, encoder)?;
        crate::codec::encode_bytes(&self.bytes, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for PhotoCachedSizeConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for PhotoCachedSizeConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let type_ = crate::codec::TlDecode::decode(decoder)?;
        let location = crate::codec::TlDecode::decode(decoder)?;
        let w = crate::codec::TlDecode::decode(decoder)?;
        let h = crate::codec::TlDecode::decode(decoder)?;
        let bytes = crate::codec::decode_bytes(decoder)?;
        Ok(Self {
            type_,
            location,
            w,
            h,
            bytes,
        })
    }
}
///@introduced 23
///File is currently unavailable.
///Official Telegram documentation: <https://core.telegram.org/constructor/fileLocationUnavailable>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct FileLocationUnavailableConstructor {
    ///Volume ID
    pub volume_id: i64,
    ///Local ID
    pub local_id: i32,
    ///Secret
    pub secret: i64,
}
impl FileLocationUnavailableConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x7c59_6b46;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "fileLocationUnavailable";
    /// Original TL result expression.
    pub const RESULT: &'static str = "FileLocation";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(23);
}
impl crate::codec::TlEncode for FileLocationUnavailableConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.volume_id, encoder)?;
        crate::codec::TlEncode::encode(&self.local_id, encoder)?;
        crate::codec::TlEncode::encode(&self.secret, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for FileLocationUnavailableConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for FileLocationUnavailableConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let volume_id = crate::codec::TlDecode::decode(decoder)?;
        let local_id = crate::codec::TlDecode::decode(decoder)?;
        let secret = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            volume_id,
            local_id,
            secret,
        })
    }
}
///@introduced 23
///File location.
///Official Telegram documentation: <https://core.telegram.org/constructor/fileLocation>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct FileLocationConstructor {
    ///DC ID
    pub dc_id: i32,
    ///Volume ID
    pub volume_id: i64,
    ///Local ID
    pub local_id: i32,
    ///Secret
    pub secret: i64,
}
impl FileLocationConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x53d6_9076;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "fileLocation";
    /// Original TL result expression.
    pub const RESULT: &'static str = "FileLocation";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(23);
}
impl crate::codec::TlEncode for FileLocationConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.dc_id, encoder)?;
        crate::codec::TlEncode::encode(&self.volume_id, encoder)?;
        crate::codec::TlEncode::encode(&self.local_id, encoder)?;
        crate::codec::TlEncode::encode(&self.secret, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for FileLocationConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for FileLocationConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let dc_id = crate::codec::TlDecode::decode(decoder)?;
        let volume_id = crate::codec::TlDecode::decode(decoder)?;
        let local_id = crate::codec::TlDecode::decode(decoder)?;
        let secret = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            dc_id,
            volume_id,
            local_id,
            secret,
        })
    }
}
///@introduced 23
///Non-e2e documented forwarded from non-secret chat
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaExternalDocument>.
///Related protocol documentation: <https://core.telegram.org/type/DocumentAttribute>.
///Related protocol documentation: <https://core.telegram.org/type/PhotoSize>.
///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageMediaExternalDocumentConstructor {
    ///Document ID
    pub id: i64,
    ///access hash
    pub access_hash: i64,
    ///Date
    pub date: i32,
    ///Mime type
    pub mime_type: String,
    ///Size
    pub size: i32,
    ///Thumbnail
    pub thumb: Box<PhotoSize>,
    ///DC ID
    pub dc_id: i32,
    ///Attributes for media types
    pub attributes: Box<Vector<Box<DocumentAttribute>>>,
}
impl DecryptedMessageMediaExternalDocumentConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xfa95_b0dd;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaExternalDocument";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(23);
}
impl crate::codec::TlEncode for DecryptedMessageMediaExternalDocumentConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.id, encoder)?;
        crate::codec::TlEncode::encode(&self.access_hash, encoder)?;
        crate::codec::TlEncode::encode(&self.date, encoder)?;
        crate::codec::TlEncode::encode(&self.mime_type, encoder)?;
        crate::codec::TlEncode::encode(&self.size, encoder)?;
        crate::codec::TlEncode::encode(&self.thumb, encoder)?;
        crate::codec::TlEncode::encode(&self.dc_id, encoder)?;
        crate::codec::TlEncode::encode(&self.attributes, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageMediaExternalDocumentConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageMediaExternalDocumentConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let id = crate::codec::TlDecode::decode(decoder)?;
        let access_hash = crate::codec::TlDecode::decode(decoder)?;
        let date = crate::codec::TlDecode::decode(decoder)?;
        let mime_type = crate::codec::TlDecode::decode(decoder)?;
        let size = crate::codec::TlDecode::decode(decoder)?;
        let thumb = crate::codec::TlDecode::decode(decoder)?;
        let dc_id = crate::codec::TlDecode::decode(decoder)?;
        let attributes = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            id,
            access_hash,
            date,
            mime_type,
            size,
            thumb,
            dc_id,
            attributes,
        })
    }
}
///@introduced 45
///Unknown message entity
///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityUnknown>.
///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MessageEntityUnknownConstructor {
    ///Offset of message entity within message (in UTF-16 code units )
    pub offset: i32,
    ///Length of message entity within message (in UTF-16 code units )
    pub length: i32,
}
impl MessageEntityUnknownConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xbb92_ba95;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "messageEntityUnknown";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MessageEntity";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for MessageEntityUnknownConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.offset, encoder)?;
        crate::codec::TlEncode::encode(&self.length, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MessageEntityUnknownConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MessageEntityUnknownConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let offset = crate::codec::TlDecode::decode(decoder)?;
        let length = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { offset, length })
    }
}
///@introduced 45
///Message entity mentioning a user by @username ; messageEntityMentionName can also be used to mention users by their ID.
///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityMention>.
///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
///Related protocol documentation: <https://core.telegram.org/api/mentions>.
///Related protocol documentation: <https://core.telegram.org/constructor/messageEntityMentionName>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MessageEntityMentionConstructor {
    ///Offset of message entity within message (in UTF-16 code units )
    pub offset: i32,
    ///Length of message entity within message (in UTF-16 code units )
    pub length: i32,
}
impl MessageEntityMentionConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xfa04_579d;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "messageEntityMention";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MessageEntity";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for MessageEntityMentionConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.offset, encoder)?;
        crate::codec::TlEncode::encode(&self.length, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MessageEntityMentionConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MessageEntityMentionConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let offset = crate::codec::TlDecode::decode(decoder)?;
        let length = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { offset, length })
    }
}
///@introduced 45
///#hashtag message entity
///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityHashtag>.
///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MessageEntityHashtagConstructor {
    ///Offset of message entity within message (in UTF-16 code units )
    pub offset: i32,
    ///Length of message entity within message (in UTF-16 code units )
    pub length: i32,
}
impl MessageEntityHashtagConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x6f63_5b0d;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "messageEntityHashtag";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MessageEntity";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for MessageEntityHashtagConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.offset, encoder)?;
        crate::codec::TlEncode::encode(&self.length, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MessageEntityHashtagConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MessageEntityHashtagConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let offset = crate::codec::TlDecode::decode(decoder)?;
        let length = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { offset, length })
    }
}
///@introduced 45
///Message entity representing a bot /command
///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityBotCommand>.
///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MessageEntityBotCommandConstructor {
    ///Offset of message entity within message (in UTF-16 code units )
    pub offset: i32,
    ///Length of message entity within message (in UTF-16 code units )
    pub length: i32,
}
impl MessageEntityBotCommandConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x6cef_8ac7;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "messageEntityBotCommand";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MessageEntity";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for MessageEntityBotCommandConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.offset, encoder)?;
        crate::codec::TlEncode::encode(&self.length, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MessageEntityBotCommandConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MessageEntityBotCommandConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let offset = crate::codec::TlDecode::decode(decoder)?;
        let length = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { offset, length })
    }
}
///@introduced 45
///Message entity representing an in-text url: <https://google.com> ; for text urls , use messageEntityTextUrl .
///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityUrl>.
///Related protocol documentation: <https://google.com>.
///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
///Related protocol documentation: <https://core.telegram.org/constructor/messageEntityTextUrl>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://google.com>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MessageEntityUrlConstructor {
    ///Offset of message entity within message (in UTF-16 code units )
    pub offset: i32,
    ///Length of message entity within message (in UTF-16 code units )
    pub length: i32,
}
impl MessageEntityUrlConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x6ed0_2538;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "messageEntityUrl";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MessageEntity";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for MessageEntityUrlConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.offset, encoder)?;
        crate::codec::TlEncode::encode(&self.length, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MessageEntityUrlConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MessageEntityUrlConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let offset = crate::codec::TlDecode::decode(decoder)?;
        let length = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { offset, length })
    }
}
///@introduced 45
///Message entity representing an email@example.com .
///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityEmail>.
///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MessageEntityEmailConstructor {
    ///Offset of message entity within message (in UTF-16 code units )
    pub offset: i32,
    ///Length of message entity within message (in UTF-16 code units )
    pub length: i32,
}
impl MessageEntityEmailConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x64e4_75c2;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "messageEntityEmail";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MessageEntity";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for MessageEntityEmailConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.offset, encoder)?;
        crate::codec::TlEncode::encode(&self.length, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MessageEntityEmailConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MessageEntityEmailConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let offset = crate::codec::TlDecode::decode(decoder)?;
        let length = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { offset, length })
    }
}
///@introduced 45
///Message entity representing bold text .
///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityBold>.
///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MessageEntityBoldConstructor {
    ///Offset of message entity within message (in UTF-16 code units )
    pub offset: i32,
    ///Length of message entity within message (in UTF-16 code units )
    pub length: i32,
}
impl MessageEntityBoldConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xbd61_0bc9;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "messageEntityBold";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MessageEntity";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for MessageEntityBoldConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.offset, encoder)?;
        crate::codec::TlEncode::encode(&self.length, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MessageEntityBoldConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MessageEntityBoldConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let offset = crate::codec::TlDecode::decode(decoder)?;
        let length = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { offset, length })
    }
}
///@introduced 45
///Message entity representing italic text .
///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityItalic>.
///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MessageEntityItalicConstructor {
    ///Offset of message entity within message (in UTF-16 code units )
    pub offset: i32,
    ///Length of message entity within message (in UTF-16 code units )
    pub length: i32,
}
impl MessageEntityItalicConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x826f_8b60;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "messageEntityItalic";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MessageEntity";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for MessageEntityItalicConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.offset, encoder)?;
        crate::codec::TlEncode::encode(&self.length, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MessageEntityItalicConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MessageEntityItalicConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let offset = crate::codec::TlDecode::decode(decoder)?;
        let length = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { offset, length })
    }
}
///@introduced 45
///Message entity representing a codeblock .
///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityCode>.
///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MessageEntityCodeConstructor {
    ///Offset of message entity within message (in UTF-16 code units )
    pub offset: i32,
    ///Length of message entity within message (in UTF-16 code units )
    pub length: i32,
}
impl MessageEntityCodeConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x28a2_0571;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "messageEntityCode";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MessageEntity";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for MessageEntityCodeConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.offset, encoder)?;
        crate::codec::TlEncode::encode(&self.length, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MessageEntityCodeConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MessageEntityCodeConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let offset = crate::codec::TlDecode::decode(decoder)?;
        let length = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { offset, length })
    }
}
///@introduced 45
///Message entity representing a preformatted codeblock , allowing the user to specify a programming language for the codeblock.
///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityPre>.
///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MessageEntityPreConstructor {
    ///Offset of message entity within message (in UTF-16 code units )
    pub offset: i32,
    ///Length of message entity within message (in UTF-16 code units )
    pub length: i32,
    ///Programming language of the code
    pub language: String,
}
impl MessageEntityPreConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x7392_4be0;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "messageEntityPre";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MessageEntity";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for MessageEntityPreConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.offset, encoder)?;
        crate::codec::TlEncode::encode(&self.length, encoder)?;
        crate::codec::TlEncode::encode(&self.language, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MessageEntityPreConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MessageEntityPreConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let offset = crate::codec::TlDecode::decode(decoder)?;
        let length = crate::codec::TlDecode::decode(decoder)?;
        let language = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { offset, length, language })
    }
}
///@introduced 45
///Message entity representing a text url : for in-text urls like <https://google.com> use messageEntityUrl .
///Official Telegram documentation: <https://core.telegram.org/constructor/messageEntityTextUrl>.
///Related protocol documentation: <https://google.com>.
///Related protocol documentation: <https://core.telegram.org/api/entities#entity-length>.
///Related protocol documentation: <https://core.telegram.org/constructor/messageEntityUrl>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
///Related protocol documentation: <https://google.com>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MessageEntityTextUrlConstructor {
    ///Offset of message entity within message (in UTF-16 code units )
    pub offset: i32,
    ///Length of message entity within message (in UTF-16 code units )
    pub length: i32,
    ///The actual URL
    pub url: String,
}
impl MessageEntityTextUrlConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x76a6_d327;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "messageEntityTextUrl";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MessageEntity";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for MessageEntityTextUrlConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.offset, encoder)?;
        crate::codec::TlEncode::encode(&self.length, encoder)?;
        crate::codec::TlEncode::encode(&self.url, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MessageEntityTextUrlConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MessageEntityTextUrlConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let offset = crate::codec::TlDecode::decode(decoder)?;
        let length = crate::codec::TlDecode::decode(decoder)?;
        let url = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { offset, length, url })
    }
}
///@introduced 45
///Stickerset by short name, from a stickerset deep link »
///Official Telegram documentation: <https://core.telegram.org/constructor/inputStickerSetShortName>.
///Related protocol documentation: <https://core.telegram.org/api/links#stickerset-links>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct InputStickerSetShortNameConstructor {
    ///Short name from a stickerset deep link »
    pub short_name: String,
}
impl InputStickerSetShortNameConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x861c_c8a0;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "inputStickerSetShortName";
    /// Original TL result expression.
    pub const RESULT: &'static str = "InputStickerSet";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for InputStickerSetShortNameConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.short_name, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for InputStickerSetShortNameConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for InputStickerSetShortNameConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let short_name = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { short_name })
    }
}
///@introduced 45
///Empty constructor
///Official Telegram documentation: <https://core.telegram.org/constructor/inputStickerSetEmpty>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct InputStickerSetEmptyConstructor {}
impl InputStickerSetEmptyConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xffb6_2b95;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "inputStickerSetEmpty";
    /// Original TL result expression.
    pub const RESULT: &'static str = "InputStickerSet";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for InputStickerSetEmptyConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for InputStickerSetEmptyConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for InputStickerSetEmptyConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 45
///Venue
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaVenue>.
///Related protocol documentation: <https://core.telegram.org/type/double>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageMediaVenueConstructor {
    ///Latitude of venue
    pub lat: f64,
    ///Longitude of venue
    pub long: f64,
    ///Venue name
    pub title: String,
    ///Address
    pub address: String,
    ///Venue provider: currently only "foursquare" and "gplaces" (Google Places) need to be supported
    pub provider: String,
    ///Venue ID in the provider's database
    pub venue_id: String,
}
impl DecryptedMessageMediaVenueConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x8a0d_f56f;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaVenue";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for DecryptedMessageMediaVenueConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.lat, encoder)?;
        crate::codec::TlEncode::encode(&self.long, encoder)?;
        crate::codec::TlEncode::encode(&self.title, encoder)?;
        crate::codec::TlEncode::encode(&self.address, encoder)?;
        crate::codec::TlEncode::encode(&self.provider, encoder)?;
        crate::codec::TlEncode::encode(&self.venue_id, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageMediaVenueConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageMediaVenueConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let lat = crate::codec::TlDecode::decode(decoder)?;
        let long = crate::codec::TlDecode::decode(decoder)?;
        let title = crate::codec::TlDecode::decode(decoder)?;
        let address = crate::codec::TlDecode::decode(decoder)?;
        let provider = crate::codec::TlDecode::decode(decoder)?;
        let venue_id = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            lat,
            long,
            title,
            address,
            provider,
            venue_id,
        })
    }
}
///@introduced 45
///Webpage preview
///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaWebPage>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DecryptedMessageMediaWebPageConstructor {
    ///URL of webpage
    pub url: String,
}
impl DecryptedMessageMediaWebPageConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xe505_11d8;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaWebPage";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(45);
}
impl crate::codec::TlEncode for DecryptedMessageMediaWebPageConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.url, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DecryptedMessageMediaWebPageConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DecryptedMessageMediaWebPageConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let url = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { url })
    }
}
///@introduced 66
///User is recording a round video to share
///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageRecordRoundAction>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct SendMessageRecordRoundActionConstructor {}
impl SendMessageRecordRoundActionConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x88f2_7fbc;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "sendMessageRecordRoundAction";
    /// Original TL result expression.
    pub const RESULT: &'static str = "SendMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(66);
}
impl crate::codec::TlEncode for SendMessageRecordRoundActionConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for SendMessageRecordRoundActionConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for SendMessageRecordRoundActionConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///@introduced 66
///User is uploading a round video
///Official Telegram documentation: <https://core.telegram.org/constructor/sendMessageUploadRoundAction>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct SendMessageUploadRoundActionConstructor {}
impl SendMessageUploadRoundActionConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xbb71_8624;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "sendMessageUploadRoundAction";
    /// Original TL result expression.
    pub const RESULT: &'static str = "SendMessageAction";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(73);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(66);
}
impl crate::codec::TlEncode for SendMessageUploadRoundActionConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for SendMessageUploadRoundActionConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for SendMessageUploadRoundActionConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
