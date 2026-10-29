///Telegram Secret Chat schema layer 8.
///Telegram schema layer 8.
/// Layer declared by the source schema.
pub const LAYER: u32 = 8;
/// Canonical source used to generate this module.
pub const SOURCE_URL: &str = "https://core.telegram.org/schema/end-to-end";
use crate::latest::api::*;
///TL type `DecryptedMessage`: <https://core.telegram.org/type/DecryptedMessage>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DecryptedMessage {
    ///LAYER 8
    ///@introduced 8
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
    ///@introduced 8
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
///TL type `DecryptedMessageMedia`: <https://core.telegram.org/type/DecryptedMessageMedia>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DecryptedMessageMedia {
    ///@introduced 8
    ///Empty constructor, no media content.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaEmpty>.
    DecryptedMessageMediaEmpty(DecryptedMessageMediaEmptyConstructor),
    ///@introduced 8
    ///Photo attached to an encrypted message.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaPhoto>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DecryptedMessageMediaPhoto(DecryptedMessageMediaPhotoConstructor),
    ///@introduced 8
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
    ///@introduced 8
    ///Document attached to a message in a secret chat.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaDocument>.
    ///Related protocol documentation: <https://core.telegram.org/type/DocumentAttribute>.
    ///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DecryptedMessageMediaDocument(DecryptedMessageMediaDocumentConstructor),
    ///@introduced 8
    ///Audio file attached to a secret chat message.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/decryptedMessageMediaAudio>.
    ///Related protocol documentation: <https://core.telegram.org/type/bytes>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    DecryptedMessageMediaAudio(DecryptedMessageMediaAudioConstructor),
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
///LAYER 8
///@introduced 8
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
    ///Random message ID, assigned by the author of message. Must be equal to the ID passed to sending method. See here » for more info on random ID deduplication and updateMessageID mapping.
    pub random_id: i64,
    ///Random bytes, removed in layer 17.
    pub random_bytes: Vec<u8>,
    ///Message text
    pub message: String,
    ///Media content
    pub media: Box<DecryptedMessageMedia>,
}
impl DecryptedMessageConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x1f81_4f1f;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessage";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessage";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
}
impl crate::codec::TlEncode for DecryptedMessageConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.random_id, encoder)?;
        crate::codec::encode_bytes(&self.random_bytes, encoder)?;
        crate::codec::TlEncode::encode(&self.message, encoder)?;
        crate::codec::TlEncode::encode(&self.media, encoder)?;
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
        let random_id = crate::codec::TlDecode::decode(decoder)?;
        let random_bytes = crate::codec::decode_bytes(decoder)?;
        let message = crate::codec::TlDecode::decode(decoder)?;
        let media = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            random_id,
            random_bytes,
            message,
            media,
        })
    }
}
///@introduced 8
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
    ///Random bytes, removed in Layer 17.
    pub random_bytes: Vec<u8>,
    ///Action relevant to the service message
    pub action: Box<DecryptedMessageAction>,
}
impl DecryptedMessageServiceConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xaa48_327d;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageService";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessage";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
}
impl crate::codec::TlEncode for DecryptedMessageServiceConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.random_id, encoder)?;
        crate::codec::encode_bytes(&self.random_bytes, encoder)?;
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
        let random_bytes = crate::codec::decode_bytes(decoder)?;
        let action = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            random_id,
            random_bytes,
            action,
        })
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
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
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
///@introduced 8
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
}
impl DecryptedMessageMediaPhotoConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x3279_8a8c;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaPhoto";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
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
        Ok(Self {
            thumb,
            thumb_w,
            thumb_h,
            w,
            h,
            size,
            key,
            iv,
        })
    }
}
///@introduced 8
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
}
impl DecryptedMessageMediaVideoConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x4cee_6ef3;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaVideo";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
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
        crate::codec::TlEncode::encode(&self.w, encoder)?;
        crate::codec::TlEncode::encode(&self.h, encoder)?;
        crate::codec::TlEncode::encode(&self.size, encoder)?;
        crate::codec::encode_bytes(&self.key, encoder)?;
        crate::codec::encode_bytes(&self.iv, encoder)?;
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
        let w = crate::codec::TlDecode::decode(decoder)?;
        let h = crate::codec::TlDecode::decode(decoder)?;
        let size = crate::codec::TlDecode::decode(decoder)?;
        let key = crate::codec::decode_bytes(decoder)?;
        let iv = crate::codec::decode_bytes(decoder)?;
        Ok(Self {
            thumb,
            thumb_w,
            thumb_h,
            duration,
            w,
            h,
            size,
            key,
            iv,
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
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
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
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
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
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
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
///@introduced 8
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
    ///File name, moved to attributes in Layer 45.
    pub file_name: String,
    ///File MIME-type
    pub mime_type: String,
    ///Document size ( int on layer &lt;143, long on layer &gt;=143)
    pub size: i32,
    ///Key to decrypt the attached document file
    pub key: Vec<u8>,
    ///Initialization
    pub iv: Vec<u8>,
}
impl DecryptedMessageMediaDocumentConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xb095_434b;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaDocument";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
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
        crate::codec::TlEncode::encode(&self.file_name, encoder)?;
        crate::codec::TlEncode::encode(&self.mime_type, encoder)?;
        crate::codec::TlEncode::encode(&self.size, encoder)?;
        crate::codec::encode_bytes(&self.key, encoder)?;
        crate::codec::encode_bytes(&self.iv, encoder)?;
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
        let file_name = crate::codec::TlDecode::decode(decoder)?;
        let mime_type = crate::codec::TlDecode::decode(decoder)?;
        let size = crate::codec::TlDecode::decode(decoder)?;
        let key = crate::codec::decode_bytes(decoder)?;
        let iv = crate::codec::decode_bytes(decoder)?;
        Ok(Self {
            thumb,
            thumb_w,
            thumb_h,
            file_name,
            mime_type,
            size,
            key,
            iv,
        })
    }
}
///@introduced 8
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
    ///File size
    pub size: i32,
    ///Key to decrypt the attached media file
    pub key: Vec<u8>,
    ///Initialization vector
    pub iv: Vec<u8>,
}
impl DecryptedMessageMediaAudioConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x6080_758f;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "decryptedMessageMediaAudio";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DecryptedMessageMedia";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = Some(8);
}
impl crate::codec::TlEncode for DecryptedMessageMediaAudioConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.duration, encoder)?;
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
        let size = crate::codec::TlDecode::decode(decoder)?;
        let key = crate::codec::decode_bytes(decoder)?;
        let iv = crate::codec::decode_bytes(decoder)?;
        Ok(Self { duration, size, key, iv })
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
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
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
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
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
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
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
    pub const SCHEMA_LAYER: Option<u32> = Some(8);
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
