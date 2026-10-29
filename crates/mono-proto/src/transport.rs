///Shared Telegram MTProto transport schema; see <https://core.telegram.org/mtproto/mtproto-transports> for framing, obfuscation, and MTProxy connection details.
/// Canonical source used to generate this module.
pub const SOURCE_URL: &str = "https://core.telegram.org/schema/mtproto";
///TL type `BadMsgNotification`: <https://core.telegram.org/type/BadMsgNotification>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BadMsgNotification {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/bad_msg_notification>.
    BadMsgNotification(BadMsgNotificationConstructor),
    ///Official Telegram documentation: <https://core.telegram.org/constructor/bad_server_salt>.
    BadServerSalt(BadServerSaltConstructor),
}
impl crate::codec::TlEncode for BadMsgNotification {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::BadMsgNotification(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::BadServerSalt(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for BadMsgNotification {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for BadMsgNotification {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            BadMsgNotificationConstructor::ID => {
                Ok(Self::BadMsgNotification(crate::codec::TlDecode::decode(decoder)?))
            }
            BadServerSaltConstructor::ID => {
                Ok(Self::BadServerSalt(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for BadMsgNotification {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::BadMsgNotification(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::BadServerSalt(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for BadMsgNotification {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "BadMsgNotification",
        })
    }
}
///TL type `BindAuthKeyInner`: <https://core.telegram.org/type/BindAuthKeyInner>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BindAuthKeyInner {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/bind_auth_key_inner>.
    BindAuthKeyInner(BindAuthKeyInnerConstructor),
}
impl crate::codec::TlEncode for BindAuthKeyInner {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::BindAuthKeyInner(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for BindAuthKeyInner {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for BindAuthKeyInner {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            BindAuthKeyInnerConstructor::ID => {
                Ok(Self::BindAuthKeyInner(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for BindAuthKeyInner {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::BindAuthKeyInner(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for BindAuthKeyInner {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::BindAuthKeyInner(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `Client_DH_Inner_Data`: <https://core.telegram.org/type/Client_DH_Inner_Data>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClientDhInnerData {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/client_DH_inner_data>.
    ClientDhInnerData(ClientDhInnerDataConstructor),
}
impl crate::codec::TlEncode for ClientDhInnerData {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::ClientDhInnerData(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for ClientDhInnerData {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for ClientDhInnerData {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            ClientDhInnerDataConstructor::ID => {
                Ok(Self::ClientDhInnerData(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for ClientDhInnerData {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::ClientDhInnerData(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for ClientDhInnerData {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::ClientDhInnerData(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `DestroyAuthKeyRes`: <https://core.telegram.org/type/DestroyAuthKeyRes>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DestroyAuthKeyRes {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/destroy_auth_key_ok>.
    DestroyAuthKeyOk(DestroyAuthKeyOkConstructor),
    ///Official Telegram documentation: <https://core.telegram.org/constructor/destroy_auth_key_none>.
    DestroyAuthKeyNone(DestroyAuthKeyNoneConstructor),
    ///Official Telegram documentation: <https://core.telegram.org/constructor/destroy_auth_key_fail>.
    DestroyAuthKeyFail(DestroyAuthKeyFailConstructor),
}
impl crate::codec::TlEncode for DestroyAuthKeyRes {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DestroyAuthKeyOk(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DestroyAuthKeyNone(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DestroyAuthKeyFail(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for DestroyAuthKeyRes {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for DestroyAuthKeyRes {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            DestroyAuthKeyOkConstructor::ID => {
                Ok(Self::DestroyAuthKeyOk(crate::codec::TlDecode::decode(decoder)?))
            }
            DestroyAuthKeyNoneConstructor::ID => {
                Ok(Self::DestroyAuthKeyNone(crate::codec::TlDecode::decode(decoder)?))
            }
            DestroyAuthKeyFailConstructor::ID => {
                Ok(Self::DestroyAuthKeyFail(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for DestroyAuthKeyRes {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DestroyAuthKeyOk(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DestroyAuthKeyNone(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DestroyAuthKeyFail(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for DestroyAuthKeyRes {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "DestroyAuthKeyRes",
        })
    }
}
///TL type `DestroySessionRes`: <https://core.telegram.org/type/DestroySessionRes>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DestroySessionRes {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/destroy_session_ok>.
    DestroySessionOk(DestroySessionOkConstructor),
    ///Official Telegram documentation: <https://core.telegram.org/constructor/destroy_session_none>.
    DestroySessionNone(DestroySessionNoneConstructor),
}
impl crate::codec::TlEncode for DestroySessionRes {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DestroySessionOk(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::DestroySessionNone(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for DestroySessionRes {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for DestroySessionRes {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            DestroySessionOkConstructor::ID => {
                Ok(Self::DestroySessionOk(crate::codec::TlDecode::decode(decoder)?))
            }
            DestroySessionNoneConstructor::ID => {
                Ok(Self::DestroySessionNone(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for DestroySessionRes {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DestroySessionOk(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::DestroySessionNone(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for DestroySessionRes {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "DestroySessionRes",
        })
    }
}
///TL type `FutureSalt`: <https://core.telegram.org/type/FutureSalt>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FutureSalt {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/future_salt>.
    FutureSalt(FutureSaltConstructor),
}
impl crate::codec::TlEncode for FutureSalt {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::FutureSalt(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for FutureSalt {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for FutureSalt {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            FutureSaltConstructor::ID => {
                Ok(Self::FutureSalt(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for FutureSalt {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::FutureSalt(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for FutureSalt {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::FutureSalt(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `FutureSalts`: <https://core.telegram.org/type/FutureSalts>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FutureSalts {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/future_salts>.
    FutureSalts(FutureSaltsConstructor),
}
impl crate::codec::TlEncode for FutureSalts {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::FutureSalts(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for FutureSalts {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for FutureSalts {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            FutureSaltsConstructor::ID => {
                Ok(Self::FutureSalts(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for FutureSalts {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::FutureSalts(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for FutureSalts {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::FutureSalts(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `HttpWait`: <https://core.telegram.org/type/HttpWait>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HttpWait {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/http_wait>.
    HttpWait(HttpWaitConstructor),
}
impl crate::codec::TlEncode for HttpWait {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::HttpWait(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for HttpWait {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for HttpWait {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            HttpWaitConstructor::ID => {
                Ok(Self::HttpWait(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for HttpWait {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::HttpWait(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for HttpWait {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::HttpWait(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `Int128`: <https://core.telegram.org/type/Int128>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Int128 {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/int128>.
    Int128(Int128Constructor),
}
impl crate::codec::TlEncode for Int128 {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::Int128(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for Int128 {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for Int128 {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            Int128Constructor::ID => {
                Ok(Self::Int128(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for Int128 {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::Int128(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for Int128 {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(Self::Int128(crate::codec::TlDecode::decode(decoder)?)))
    }
}
///TL type `Int256`: <https://core.telegram.org/type/Int256>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Int256 {
    ///Represents a 256-bit integer.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/int256>.
    Int256(Int256Constructor),
}
impl crate::codec::TlEncode for Int256 {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::Int256(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for Int256 {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for Int256 {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            Int256Constructor::ID => {
                Ok(Self::Int256(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for Int256 {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::Int256(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for Int256 {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(Self::Int256(crate::codec::TlDecode::decode(decoder)?)))
    }
}
///TL type `Message`: <https://core.telegram.org/type/Message>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Message {
    ///A message
    ///Official Telegram documentation: <https://core.telegram.org/constructor/message>.
    ///Related protocol documentation: <https://core.telegram.org/api/ai#summarize-messages>.
    ///Related protocol documentation: <https://core.telegram.org/api/boost>.
    ///Related protocol documentation: <https://core.telegram.org/api/bots/connected-business-bots>.
    ///Related protocol documentation: <https://core.telegram.org/api/bots/guest-mode#guest-messages>.
    ///Related protocol documentation: <https://core.telegram.org/api/business#quick-reply-shortcuts>.
    ///Related protocol documentation: <https://core.telegram.org/api/channel#supergroups>.
    ///Related protocol documentation: <https://core.telegram.org/api/effects>.
    ///Related protocol documentation: <https://core.telegram.org/api/entities>.
    ///Related protocol documentation: <https://core.telegram.org/api/factcheck>.
    ///Related protocol documentation: <https://core.telegram.org/api/files#albums-grouped-media>.
    ///Related protocol documentation: <https://core.telegram.org/api/files#video-qualities>.
    ///Related protocol documentation: <https://core.telegram.org/api/mentions>.
    ///Related protocol documentation: <https://core.telegram.org/api/monoforum>.
    ///Related protocol documentation: <https://core.telegram.org/api/paid-messages>.
    ///Related protocol documentation: <https://core.telegram.org/api/pin>.
    ///Related protocol documentation: <https://core.telegram.org/api/rank>.
    ///Related protocol documentation: <https://core.telegram.org/api/saved-messages>.
    ///Related protocol documentation: <https://core.telegram.org/api/scheduled-messages>.
    ///Related protocol documentation: <https://core.telegram.org/api/scheduled-messages#repeating-scheduled-messages>.
    ///Related protocol documentation: <https://core.telegram.org/api/stars>.
    ///Related protocol documentation: <https://core.telegram.org/api/suggested-posts>.
    ///Related protocol documentation: <https://core.telegram.org/api/threads>.
    ///Related protocol documentation: <https://core.telegram.org/api/views#read-message-contents>.
    ///Related protocol documentation: <https://core.telegram.org/constructor/channelFull>.
    ///Related protocol documentation: <https://core.telegram.org/constructor/inputPeerSelf>.
    ///Related protocol documentation: <https://core.telegram.org/constructor/messageMediaDocument>.
    ///Related protocol documentation: <https://core.telegram.org/constructor/true>.
    ///Related protocol documentation: <https://core.telegram.org/method/messages.reportMessagesDelivery>.
    ///Related protocol documentation: <https://core.telegram.org/method/messages.sendMultiMedia>.
    ///Related protocol documentation: <https://core.telegram.org/mtproto/TL-combinators#conditional-fields>.
    ///Related protocol documentation: <https://core.telegram.org/type/%23>.
    ///Related protocol documentation: <https://core.telegram.org/type/FactCheck>.
    ///Related protocol documentation: <https://core.telegram.org/type/MessageEntity>.
    ///Related protocol documentation: <https://core.telegram.org/type/MessageFwdHeader>.
    ///Related protocol documentation: <https://core.telegram.org/type/MessageMedia>.
    ///Related protocol documentation: <https://core.telegram.org/type/MessageReactions>.
    ///Related protocol documentation: <https://core.telegram.org/type/MessageReplies>.
    ///Related protocol documentation: <https://core.telegram.org/type/MessageReplyHeader>.
    ///Related protocol documentation: <https://core.telegram.org/type/Peer>.
    ///Related protocol documentation: <https://core.telegram.org/type/ReplyMarkup>.
    ///Related protocol documentation: <https://core.telegram.org/type/RestrictionReason>.
    ///Related protocol documentation: <https://core.telegram.org/type/SuggestedPost>.
    ///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
    ///Related protocol documentation: <https://core.telegram.org/type/int>.
    ///Related protocol documentation: <https://core.telegram.org/type/long>.
    ///Related protocol documentation: <https://core.telegram.org/type/string>.
    ///Related protocol documentation: <https://telegram.org/blog/content-protection-delete-by-date-and-more>.
    ///Related protocol documentation: <https://telegram.org/blog/star-messages-gateway-2-0-and-more#save-even-more-on-user-verification>.
    Message(MessageConstructor),
}
impl crate::codec::TlEncode for Message {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::Message(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for Message {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for Message {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            MessageConstructor::ID => {
                Ok(Self::Message(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for Message {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::Message(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for Message {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::Message(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `MessageContainer`: <https://core.telegram.org/type/MessageContainer>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MessageContainer {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/msg_container>.
    MsgContainer(MsgContainerConstructor),
}
impl crate::codec::TlEncode for MessageContainer {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgContainer(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for MessageContainer {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for MessageContainer {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            MsgContainerConstructor::ID => {
                Ok(Self::MsgContainer(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for MessageContainer {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgContainer(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for MessageContainer {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::MsgContainer(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `MessageCopy`: <https://core.telegram.org/type/MessageCopy>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MessageCopy {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/msg_copy>.
    MsgCopy(MsgCopyConstructor),
}
impl crate::codec::TlEncode for MessageCopy {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgCopy(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for MessageCopy {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for MessageCopy {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            MsgCopyConstructor::ID => {
                Ok(Self::MsgCopy(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for MessageCopy {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgCopy(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for MessageCopy {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::MsgCopy(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `MsgDetailedInfo`: <https://core.telegram.org/type/MsgDetailedInfo>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MsgDetailedInfo {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/msg_detailed_info>.
    MsgDetailedInfo(MsgDetailedInfoConstructor),
    ///Official Telegram documentation: <https://core.telegram.org/constructor/msg_new_detailed_info>.
    MsgNewDetailedInfo(MsgNewDetailedInfoConstructor),
}
impl crate::codec::TlEncode for MsgDetailedInfo {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgDetailedInfo(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::MsgNewDetailedInfo(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for MsgDetailedInfo {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for MsgDetailedInfo {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            MsgDetailedInfoConstructor::ID => {
                Ok(Self::MsgDetailedInfo(crate::codec::TlDecode::decode(decoder)?))
            }
            MsgNewDetailedInfoConstructor::ID => {
                Ok(Self::MsgNewDetailedInfo(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for MsgDetailedInfo {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgDetailedInfo(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::MsgNewDetailedInfo(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for MsgDetailedInfo {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "MsgDetailedInfo",
        })
    }
}
///TL type `MsgResendReq`: <https://core.telegram.org/type/MsgResendReq>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MsgResendReq {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/msg_resend_req>.
    MsgResendReq(MsgResendReqConstructor),
}
impl crate::codec::TlEncode for MsgResendReq {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgResendReq(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for MsgResendReq {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for MsgResendReq {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            MsgResendReqConstructor::ID => {
                Ok(Self::MsgResendReq(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for MsgResendReq {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgResendReq(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for MsgResendReq {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::MsgResendReq(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `MsgsAck`: <https://core.telegram.org/type/MsgsAck>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MsgsAck {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/msgs_ack>.
    MsgsAck(MsgsAckConstructor),
}
impl crate::codec::TlEncode for MsgsAck {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgsAck(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for MsgsAck {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for MsgsAck {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            MsgsAckConstructor::ID => {
                Ok(Self::MsgsAck(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for MsgsAck {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgsAck(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for MsgsAck {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::MsgsAck(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `MsgsAllInfo`: <https://core.telegram.org/type/MsgsAllInfo>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MsgsAllInfo {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/msgs_all_info>.
    MsgsAllInfo(MsgsAllInfoConstructor),
}
impl crate::codec::TlEncode for MsgsAllInfo {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgsAllInfo(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for MsgsAllInfo {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for MsgsAllInfo {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            MsgsAllInfoConstructor::ID => {
                Ok(Self::MsgsAllInfo(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for MsgsAllInfo {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgsAllInfo(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for MsgsAllInfo {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::MsgsAllInfo(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `MsgsStateInfo`: <https://core.telegram.org/type/MsgsStateInfo>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MsgsStateInfo {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/msgs_state_info>.
    MsgsStateInfo(MsgsStateInfoConstructor),
}
impl crate::codec::TlEncode for MsgsStateInfo {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgsStateInfo(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for MsgsStateInfo {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for MsgsStateInfo {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            MsgsStateInfoConstructor::ID => {
                Ok(Self::MsgsStateInfo(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for MsgsStateInfo {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgsStateInfo(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for MsgsStateInfo {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::MsgsStateInfo(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `MsgsStateReq`: <https://core.telegram.org/type/MsgsStateReq>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MsgsStateReq {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/msgs_state_req>.
    MsgsStateReq(MsgsStateReqConstructor),
}
impl crate::codec::TlEncode for MsgsStateReq {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgsStateReq(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for MsgsStateReq {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for MsgsStateReq {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            MsgsStateReqConstructor::ID => {
                Ok(Self::MsgsStateReq(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for MsgsStateReq {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::MsgsStateReq(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for MsgsStateReq {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::MsgsStateReq(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `NewSession`: <https://core.telegram.org/type/NewSession>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NewSession {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/new_session_created>.
    NewSessionCreated(NewSessionCreatedConstructor),
}
impl crate::codec::TlEncode for NewSession {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::NewSessionCreated(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for NewSession {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for NewSession {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            NewSessionCreatedConstructor::ID => {
                Ok(Self::NewSessionCreated(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for NewSession {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::NewSessionCreated(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for NewSession {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::NewSessionCreated(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `Object`: <https://core.telegram.org/type/Object>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Object {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/gzip_packed>.
    GzipPacked(GzipPackedConstructor),
}
impl crate::codec::TlEncode for Object {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::GzipPacked(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for Object {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for Object {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            GzipPackedConstructor::ID => {
                Ok(Self::GzipPacked(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for Object {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::GzipPacked(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for Object {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::GzipPacked(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `P_Q_inner_data`: <https://core.telegram.org/type/P_Q_inner_data>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PQInnerData {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/p_q_inner_data_dc>.
    PQInnerDataDc(PQInnerDataDcConstructor),
    ///Official Telegram documentation: <https://core.telegram.org/constructor/p_q_inner_data_temp_dc>.
    PQInnerDataTempDc(PQInnerDataTempDcConstructor),
}
impl crate::codec::TlEncode for PQInnerData {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::PQInnerDataDc(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::PQInnerDataTempDc(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for PQInnerData {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for PQInnerData {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            PQInnerDataDcConstructor::ID => {
                Ok(Self::PQInnerDataDc(crate::codec::TlDecode::decode(decoder)?))
            }
            PQInnerDataTempDcConstructor::ID => {
                Ok(Self::PQInnerDataTempDc(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for PQInnerData {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::PQInnerDataDc(value) => crate::codec::TlEncode::encode(value, encoder),
            Self::PQInnerDataTempDc(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for PQInnerData {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "P_Q_inner_data",
        })
    }
}
///TL type `Pong`: <https://core.telegram.org/type/Pong>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Pong {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/pong>.
    Pong(PongConstructor),
}
impl crate::codec::TlEncode for Pong {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::Pong(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for Pong {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for Pong {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            PongConstructor::ID => {
                Ok(Self::Pong(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for Pong {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::Pong(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for Pong {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(Self::Pong(crate::codec::TlDecode::decode(decoder)?)))
    }
}
///TL type `ResPQ`: <https://core.telegram.org/type/ResPQ>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ResPq {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/resPQ>.
    ResPq(ResPqConstructor),
}
impl crate::codec::TlEncode for ResPq {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::ResPq(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for ResPq {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for ResPq {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            ResPqConstructor::ID => {
                Ok(Self::ResPq(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for ResPq {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::ResPq(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for ResPq {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(Self::ResPq(crate::codec::TlDecode::decode(decoder)?)))
    }
}
///TL type `RpcDropAnswer`: <https://core.telegram.org/type/RpcDropAnswer>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RpcDropAnswer {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/rpc_answer_unknown>.
    RpcAnswerUnknown(RpcAnswerUnknownConstructor),
    ///Official Telegram documentation: <https://core.telegram.org/constructor/rpc_answer_dropped_running>.
    RpcAnswerDroppedRunning(RpcAnswerDroppedRunningConstructor),
    ///Official Telegram documentation: <https://core.telegram.org/constructor/rpc_answer_dropped>.
    RpcAnswerDropped(RpcAnswerDroppedConstructor),
}
impl crate::codec::TlEncode for RpcDropAnswer {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::RpcAnswerUnknown(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::RpcAnswerDroppedRunning(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
            Self::RpcAnswerDropped(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for RpcDropAnswer {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for RpcDropAnswer {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            RpcAnswerUnknownConstructor::ID => {
                Ok(Self::RpcAnswerUnknown(crate::codec::TlDecode::decode(decoder)?))
            }
            RpcAnswerDroppedRunningConstructor::ID => {
                Ok(
                    Self::RpcAnswerDroppedRunning(
                        crate::codec::TlDecode::decode(decoder)?,
                    ),
                )
            }
            RpcAnswerDroppedConstructor::ID => {
                Ok(Self::RpcAnswerDropped(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for RpcDropAnswer {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::RpcAnswerUnknown(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::RpcAnswerDroppedRunning(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
            Self::RpcAnswerDropped(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for RpcDropAnswer {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "RpcDropAnswer",
        })
    }
}
///TL type `RpcError`: <https://core.telegram.org/type/RpcError>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RpcError {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/rpc_error>.
    RpcError(RpcErrorConstructor),
}
impl crate::codec::TlEncode for RpcError {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::RpcError(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for RpcError {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for RpcError {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            RpcErrorConstructor::ID => {
                Ok(Self::RpcError(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for RpcError {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::RpcError(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for RpcError {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::RpcError(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `RpcResult`: <https://core.telegram.org/type/RpcResult>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RpcResult {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/rpc_result>.
    RpcResult(RpcResultConstructor),
}
impl crate::codec::TlEncode for RpcResult {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::RpcResult(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for RpcResult {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for RpcResult {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            RpcResultConstructor::ID => {
                Ok(Self::RpcResult(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for RpcResult {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::RpcResult(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for RpcResult {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::RpcResult(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `Server_DH_Params`: <https://core.telegram.org/type/Server_DH_Params>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ServerDhParams {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/server_DH_params_ok>.
    ServerDhParamsOk(ServerDhParamsOkConstructor),
}
impl crate::codec::TlEncode for ServerDhParams {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::ServerDhParamsOk(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for ServerDhParams {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for ServerDhParams {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            ServerDhParamsOkConstructor::ID => {
                Ok(Self::ServerDhParamsOk(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for ServerDhParams {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::ServerDhParamsOk(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for ServerDhParams {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::ServerDhParamsOk(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `Server_DH_inner_data`: <https://core.telegram.org/type/Server_DH_inner_data>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ServerDhInnerData {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/server_DH_inner_data>.
    ServerDhInnerData(ServerDhInnerDataConstructor),
}
impl crate::codec::TlEncode for ServerDhInnerData {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::ServerDhInnerData(value) => {
                crate::codec::Boxed::encode_boxed(value, encoder)
            }
        }
    }
}
impl crate::codec::TlDecode for ServerDhInnerData {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for ServerDhInnerData {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            ServerDhInnerDataConstructor::ID => {
                Ok(Self::ServerDhInnerData(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for ServerDhInnerData {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::ServerDhInnerData(value) => {
                crate::codec::TlEncode::encode(value, encoder)
            }
        }
    }
}
impl crate::codec::BareDecode for ServerDhInnerData {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(
                Self::ServerDhInnerData(crate::codec::TlDecode::decode(decoder)?),
            ))
    }
}
///TL type `Set_client_DH_params_answer`: <https://core.telegram.org/type/Set_client_DH_params_answer>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SetClientDhParamsAnswer {
    ///Official Telegram documentation: <https://core.telegram.org/constructor/dh_gen_ok>.
    DhGenOk(DhGenOkConstructor),
    ///Official Telegram documentation: <https://core.telegram.org/constructor/dh_gen_retry>.
    DhGenRetry(DhGenRetryConstructor),
    ///Official Telegram documentation: <https://core.telegram.org/constructor/dh_gen_fail>.
    DhGenFail(DhGenFailConstructor),
}
impl crate::codec::TlEncode for SetClientDhParamsAnswer {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DhGenOk(value) => crate::codec::Boxed::encode_boxed(value, encoder),
            Self::DhGenRetry(value) => crate::codec::Boxed::encode_boxed(value, encoder),
            Self::DhGenFail(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl crate::codec::TlDecode for SetClientDhParamsAnswer {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl crate::codec::BoxedDecode for SetClientDhParamsAnswer {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            DhGenOkConstructor::ID => {
                Ok(Self::DhGenOk(crate::codec::TlDecode::decode(decoder)?))
            }
            DhGenRetryConstructor::ID => {
                Ok(Self::DhGenRetry(crate::codec::TlDecode::decode(decoder)?))
            }
            DhGenFailConstructor::ID => {
                Ok(Self::DhGenFail(crate::codec::TlDecode::decode(decoder)?))
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
impl crate::codec::BareEncode for SetClientDhParamsAnswer {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::DhGenOk(value) => crate::codec::TlEncode::encode(value, encoder),
            Self::DhGenRetry(value) => crate::codec::TlEncode::encode(value, encoder),
            Self::DhGenFail(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl crate::codec::BareDecode for SetClientDhParamsAnswer {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Err(crate::codec::Error::AmbiguousBareConstructor {
            type_name: "Set_client_DH_params_answer",
        })
    }
}
///TL type `Vector`: <https://core.telegram.org/type/Vector>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Vector<T> {
    ///A universal vector constructor.
    ///Official Telegram documentation: <https://core.telegram.org/constructor/vector>.
    Vector(VectorConstructor<T>),
}
impl<T: crate::codec::TlEncode> crate::codec::TlEncode for Vector<T> {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::Vector(value) => crate::codec::Boxed::encode_boxed(value, encoder),
        }
    }
}
impl<T: crate::codec::TlDecode> crate::codec::TlDecode for Vector<T> {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder.nested(<Self as crate::codec::BoxedDecode>::decode_boxed)
    }
}
impl<T: crate::codec::TlDecode> crate::codec::BoxedDecode for Vector<T> {
    fn decode_boxed(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            <VectorConstructor<T>>::ID => {
                Ok(Self::Vector(crate::codec::TlDecode::decode(decoder)?))
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
impl<T: crate::codec::TlEncode> crate::codec::BareEncode for Vector<T> {
    fn encode_bare(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        match self {
            Self::Vector(value) => crate::codec::TlEncode::encode(value, encoder),
        }
    }
}
impl<T: crate::codec::TlDecode> crate::codec::BareDecode for Vector<T> {
    fn decode_bare(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        decoder
            .nested(|decoder| Ok(Self::Vector(crate::codec::TlDecode::decode(decoder)?)))
    }
}
///A universal vector constructor.
///Official Telegram documentation: <https://core.telegram.org/constructor/vector>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct VectorConstructor<T> {
    ///TL field `field_0`.
    pub field_0: u32,
    ///TL field `field_1`.
    pub field_1: Vec<T>,
}
impl<T> VectorConstructor<T> {
    /// TL constructor number.
    pub const ID: u32 = 0x1cb5_c415;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = false;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "vector";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Vector<t>";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl<T: crate::codec::TlEncode> crate::codec::TlEncode for VectorConstructor<T> {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.field_0, encoder)?;
        crate::codec::validate_dependent_len(
            usize::try_from(self.field_0)
                .map_err(|_| crate::codec::Error::InvalidNatural {
                    value: u64::from(self.field_0),
                })?,
            (&self.field_1).len(),
        )?;
        for value in &self.field_1 {
            crate::codec::TlEncode::encode(value, encoder)?;
        }
        Ok(())
    }
}
impl<T: crate::codec::TlEncode> crate::codec::Boxed for VectorConstructor<T> {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl<T: crate::codec::TlDecode> crate::codec::TlDecode for VectorConstructor<T> {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let field_0 = crate::codec::TlDecode::decode(decoder)?;
        let field_1 = {
            let count = usize::try_from(field_0)
                .map_err(|_| crate::codec::Error::InvalidNatural {
                    value: u64::from(field_0),
                })?;
            decoder.validate_vector_len(count)?;
            (0..count)
                .map(|_| crate::codec::TlDecode::decode(decoder))
                .collect::<Result<Vec<_>, crate::codec::Error>>()?
        };
        Ok(Self { field_0, field_1 })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/int128>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct Int128Constructor {
    ///TL field `field_0`.
    pub field_0: [i32; 4],
}
impl Int128Constructor {
    /// TL constructor number.
    pub const ID: u32 = 0x84cc_f7b7;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = false;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "int128";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Int128";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for Int128Constructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::validate_dependent_len(4usize, (&self.field_0).len())?;
        for value in &self.field_0 {
            crate::codec::TlEncode::encode(value, encoder)?;
        }
        Ok(())
    }
}
impl crate::codec::Boxed for Int128Constructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for Int128Constructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let field_0 = {
            let count = 4usize;
            decoder.validate_vector_len(count)?;
            let values = (0..count)
                .map(|_| crate::codec::TlDecode::decode(decoder))
                .collect::<Result<Vec<_>, crate::codec::Error>>()?;
            values
                .try_into()
                .map_err(|values: Vec<_>| crate::codec::Error::DependentLength {
                    declared: count,
                    actual: values.len(),
                })?
        };
        Ok(Self { field_0 })
    }
}
///Represents a 256-bit integer.
///Official Telegram documentation: <https://core.telegram.org/constructor/int256>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct Int256Constructor {
    ///TL field `field_0`.
    pub field_0: [i32; 8],
}
impl Int256Constructor {
    /// TL constructor number.
    pub const ID: u32 = 0x7bed_eb5b;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = false;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "int256";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Int256";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for Int256Constructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::validate_dependent_len(8usize, (&self.field_0).len())?;
        for value in &self.field_0 {
            crate::codec::TlEncode::encode(value, encoder)?;
        }
        Ok(())
    }
}
impl crate::codec::Boxed for Int256Constructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for Int256Constructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let field_0 = {
            let count = 8usize;
            decoder.validate_vector_len(count)?;
            let values = (0..count)
                .map(|_| crate::codec::TlDecode::decode(decoder))
                .collect::<Result<Vec<_>, crate::codec::Error>>()?;
            values
                .try_into()
                .map_err(|values: Vec<_>| crate::codec::Error::DependentLength {
                    declared: count,
                    actual: values.len(),
                })?
        };
        Ok(Self { field_0 })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/resPQ>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct ResPqConstructor {
    ///TL field `nonce`.
    pub nonce: bnum::types::I128,
    ///TL field `server_nonce`.
    pub server_nonce: bnum::types::I128,
    ///TL field `pq`.
    pub pq: Vec<u8>,
    ///TL field `server_public_key_fingerprints`.
    pub server_public_key_fingerprints: Box<Vector<i64>>,
}
impl ResPqConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x0516_2463;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "resPQ";
    /// Original TL result expression.
    pub const RESULT: &'static str = "ResPQ";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for ResPqConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.server_nonce, encoder)?;
        crate::codec::encode_bytes(&self.pq, encoder)?;
        crate::codec::TlEncode::encode(&self.server_public_key_fingerprints, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for ResPqConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for ResPqConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let nonce = crate::codec::TlDecode::decode(decoder)?;
        let server_nonce = crate::codec::TlDecode::decode(decoder)?;
        let pq = crate::codec::decode_bytes(decoder)?;
        let server_public_key_fingerprints = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            nonce,
            server_nonce,
            pq,
            server_public_key_fingerprints,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/p_q_inner_data_dc>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct PQInnerDataDcConstructor {
    ///TL field `pq`.
    pub pq: Vec<u8>,
    ///TL field `p`.
    pub p: Vec<u8>,
    ///TL field `q`.
    pub q: Vec<u8>,
    ///TL field `nonce`.
    pub nonce: bnum::types::I128,
    ///TL field `server_nonce`.
    pub server_nonce: bnum::types::I128,
    ///TL field `new_nonce`.
    pub new_nonce: bnum::types::I256,
    ///TL field `dc`.
    pub dc: i32,
}
impl PQInnerDataDcConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xa9f5_5f95;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "p_q_inner_data_dc";
    /// Original TL result expression.
    pub const RESULT: &'static str = "P_Q_inner_data";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for PQInnerDataDcConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::encode_bytes(&self.pq, encoder)?;
        crate::codec::encode_bytes(&self.p, encoder)?;
        crate::codec::encode_bytes(&self.q, encoder)?;
        crate::codec::TlEncode::encode(&self.nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.server_nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.new_nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.dc, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for PQInnerDataDcConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for PQInnerDataDcConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let pq = crate::codec::decode_bytes(decoder)?;
        let p = crate::codec::decode_bytes(decoder)?;
        let q = crate::codec::decode_bytes(decoder)?;
        let nonce = crate::codec::TlDecode::decode(decoder)?;
        let server_nonce = crate::codec::TlDecode::decode(decoder)?;
        let new_nonce = crate::codec::TlDecode::decode(decoder)?;
        let dc = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            pq,
            p,
            q,
            nonce,
            server_nonce,
            new_nonce,
            dc,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/p_q_inner_data_temp_dc>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct PQInnerDataTempDcConstructor {
    ///TL field `pq`.
    pub pq: Vec<u8>,
    ///TL field `p`.
    pub p: Vec<u8>,
    ///TL field `q`.
    pub q: Vec<u8>,
    ///TL field `nonce`.
    pub nonce: bnum::types::I128,
    ///TL field `server_nonce`.
    pub server_nonce: bnum::types::I128,
    ///TL field `new_nonce`.
    pub new_nonce: bnum::types::I256,
    ///TL field `dc`.
    pub dc: i32,
    ///TL field `expires_in`.
    pub expires_in: i32,
}
impl PQInnerDataTempDcConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x56fd_df88;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "p_q_inner_data_temp_dc";
    /// Original TL result expression.
    pub const RESULT: &'static str = "P_Q_inner_data";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for PQInnerDataTempDcConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::encode_bytes(&self.pq, encoder)?;
        crate::codec::encode_bytes(&self.p, encoder)?;
        crate::codec::encode_bytes(&self.q, encoder)?;
        crate::codec::TlEncode::encode(&self.nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.server_nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.new_nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.dc, encoder)?;
        crate::codec::TlEncode::encode(&self.expires_in, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for PQInnerDataTempDcConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for PQInnerDataTempDcConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let pq = crate::codec::decode_bytes(decoder)?;
        let p = crate::codec::decode_bytes(decoder)?;
        let q = crate::codec::decode_bytes(decoder)?;
        let nonce = crate::codec::TlDecode::decode(decoder)?;
        let server_nonce = crate::codec::TlDecode::decode(decoder)?;
        let new_nonce = crate::codec::TlDecode::decode(decoder)?;
        let dc = crate::codec::TlDecode::decode(decoder)?;
        let expires_in = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            pq,
            p,
            q,
            nonce,
            server_nonce,
            new_nonce,
            dc,
            expires_in,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/server_DH_params_ok>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct ServerDhParamsOkConstructor {
    ///TL field `nonce`.
    pub nonce: bnum::types::I128,
    ///TL field `server_nonce`.
    pub server_nonce: bnum::types::I128,
    ///TL field `encrypted_answer`.
    pub encrypted_answer: Vec<u8>,
}
impl ServerDhParamsOkConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xd0e8_075c;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "server_DH_params_ok";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Server_DH_Params";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for ServerDhParamsOkConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.server_nonce, encoder)?;
        crate::codec::encode_bytes(&self.encrypted_answer, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for ServerDhParamsOkConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for ServerDhParamsOkConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let nonce = crate::codec::TlDecode::decode(decoder)?;
        let server_nonce = crate::codec::TlDecode::decode(decoder)?;
        let encrypted_answer = crate::codec::decode_bytes(decoder)?;
        Ok(Self {
            nonce,
            server_nonce,
            encrypted_answer,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/server_DH_inner_data>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct ServerDhInnerDataConstructor {
    ///TL field `nonce`.
    pub nonce: bnum::types::I128,
    ///TL field `server_nonce`.
    pub server_nonce: bnum::types::I128,
    ///TL field `g`.
    pub g: i32,
    ///TL field `dh_prime`.
    pub dh_prime: Vec<u8>,
    ///TL field `g_a`.
    pub g_a: Vec<u8>,
    ///TL field `server_time`.
    pub server_time: i32,
}
impl ServerDhInnerDataConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xb589_0dba;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "server_DH_inner_data";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Server_DH_inner_data";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for ServerDhInnerDataConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.server_nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.g, encoder)?;
        crate::codec::encode_bytes(&self.dh_prime, encoder)?;
        crate::codec::encode_bytes(&self.g_a, encoder)?;
        crate::codec::TlEncode::encode(&self.server_time, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for ServerDhInnerDataConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for ServerDhInnerDataConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let nonce = crate::codec::TlDecode::decode(decoder)?;
        let server_nonce = crate::codec::TlDecode::decode(decoder)?;
        let g = crate::codec::TlDecode::decode(decoder)?;
        let dh_prime = crate::codec::decode_bytes(decoder)?;
        let g_a = crate::codec::decode_bytes(decoder)?;
        let server_time = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            nonce,
            server_nonce,
            g,
            dh_prime,
            g_a,
            server_time,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/client_DH_inner_data>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct ClientDhInnerDataConstructor {
    ///TL field `nonce`.
    pub nonce: bnum::types::I128,
    ///TL field `server_nonce`.
    pub server_nonce: bnum::types::I128,
    ///TL field `retry_id`.
    pub retry_id: i64,
    ///TL field `g_b`.
    pub g_b: Vec<u8>,
}
impl ClientDhInnerDataConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x6643_b654;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "client_DH_inner_data";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Client_DH_Inner_Data";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for ClientDhInnerDataConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.server_nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.retry_id, encoder)?;
        crate::codec::encode_bytes(&self.g_b, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for ClientDhInnerDataConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for ClientDhInnerDataConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let nonce = crate::codec::TlDecode::decode(decoder)?;
        let server_nonce = crate::codec::TlDecode::decode(decoder)?;
        let retry_id = crate::codec::TlDecode::decode(decoder)?;
        let g_b = crate::codec::decode_bytes(decoder)?;
        Ok(Self {
            nonce,
            server_nonce,
            retry_id,
            g_b,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/dh_gen_ok>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DhGenOkConstructor {
    ///TL field `nonce`.
    pub nonce: bnum::types::I128,
    ///TL field `server_nonce`.
    pub server_nonce: bnum::types::I128,
    ///TL field `new_nonce_hash1`.
    pub new_nonce_hash1: bnum::types::I128,
}
impl DhGenOkConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x3bcb_f734;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "dh_gen_ok";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Set_client_DH_params_answer";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for DhGenOkConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.server_nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.new_nonce_hash1, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DhGenOkConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DhGenOkConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let nonce = crate::codec::TlDecode::decode(decoder)?;
        let server_nonce = crate::codec::TlDecode::decode(decoder)?;
        let new_nonce_hash1 = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            nonce,
            server_nonce,
            new_nonce_hash1,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/dh_gen_retry>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DhGenRetryConstructor {
    ///TL field `nonce`.
    pub nonce: bnum::types::I128,
    ///TL field `server_nonce`.
    pub server_nonce: bnum::types::I128,
    ///TL field `new_nonce_hash2`.
    pub new_nonce_hash2: bnum::types::I128,
}
impl DhGenRetryConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x46dc_1fb9;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "dh_gen_retry";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Set_client_DH_params_answer";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for DhGenRetryConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.server_nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.new_nonce_hash2, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DhGenRetryConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DhGenRetryConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let nonce = crate::codec::TlDecode::decode(decoder)?;
        let server_nonce = crate::codec::TlDecode::decode(decoder)?;
        let new_nonce_hash2 = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            nonce,
            server_nonce,
            new_nonce_hash2,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/dh_gen_fail>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DhGenFailConstructor {
    ///TL field `nonce`.
    pub nonce: bnum::types::I128,
    ///TL field `server_nonce`.
    pub server_nonce: bnum::types::I128,
    ///TL field `new_nonce_hash3`.
    pub new_nonce_hash3: bnum::types::I128,
}
impl DhGenFailConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xa69d_ae02;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "dh_gen_fail";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Set_client_DH_params_answer";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for DhGenFailConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.server_nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.new_nonce_hash3, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DhGenFailConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DhGenFailConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let nonce = crate::codec::TlDecode::decode(decoder)?;
        let server_nonce = crate::codec::TlDecode::decode(decoder)?;
        let new_nonce_hash3 = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            nonce,
            server_nonce,
            new_nonce_hash3,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/bind_auth_key_inner>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct BindAuthKeyInnerConstructor {
    ///TL field `nonce`.
    pub nonce: i64,
    ///TL field `temp_auth_key_id`.
    pub temp_auth_key_id: i64,
    ///TL field `perm_auth_key_id`.
    pub perm_auth_key_id: i64,
    ///TL field `temp_session_id`.
    pub temp_session_id: i64,
    ///TL field `expires_at`.
    pub expires_at: i32,
}
impl BindAuthKeyInnerConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x75a3_f765;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "bind_auth_key_inner";
    /// Original TL result expression.
    pub const RESULT: &'static str = "BindAuthKeyInner";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for BindAuthKeyInnerConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.temp_auth_key_id, encoder)?;
        crate::codec::TlEncode::encode(&self.perm_auth_key_id, encoder)?;
        crate::codec::TlEncode::encode(&self.temp_session_id, encoder)?;
        crate::codec::TlEncode::encode(&self.expires_at, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for BindAuthKeyInnerConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for BindAuthKeyInnerConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let nonce = crate::codec::TlDecode::decode(decoder)?;
        let temp_auth_key_id = crate::codec::TlDecode::decode(decoder)?;
        let perm_auth_key_id = crate::codec::TlDecode::decode(decoder)?;
        let temp_session_id = crate::codec::TlDecode::decode(decoder)?;
        let expires_at = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            nonce,
            temp_auth_key_id,
            perm_auth_key_id,
            temp_session_id,
            expires_at,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/rpc_result>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct RpcResultConstructor {
    ///TL field `req_msg_id`.
    pub req_msg_id: i64,
    ///TL field `result`.
    pub result: Box<Object>,
}
impl RpcResultConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xf35c_6d01;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "rpc_result";
    /// Original TL result expression.
    pub const RESULT: &'static str = "RpcResult";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for RpcResultConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.req_msg_id, encoder)?;
        crate::codec::TlEncode::encode(&self.result, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for RpcResultConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for RpcResultConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let req_msg_id = crate::codec::TlDecode::decode(decoder)?;
        let result = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { req_msg_id, result })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/rpc_error>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct RpcErrorConstructor {
    ///TL field `error_code`.
    pub error_code: i32,
    ///TL field `error_message`.
    pub error_message: String,
}
impl RpcErrorConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x2144_ca19;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "rpc_error";
    /// Original TL result expression.
    pub const RESULT: &'static str = "RpcError";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for RpcErrorConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.error_code, encoder)?;
        crate::codec::TlEncode::encode(&self.error_message, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for RpcErrorConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for RpcErrorConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let error_code = crate::codec::TlDecode::decode(decoder)?;
        let error_message = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { error_code, error_message })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/rpc_answer_unknown>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct RpcAnswerUnknownConstructor {}
impl RpcAnswerUnknownConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x5e2a_d36e;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "rpc_answer_unknown";
    /// Original TL result expression.
    pub const RESULT: &'static str = "RpcDropAnswer";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for RpcAnswerUnknownConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for RpcAnswerUnknownConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for RpcAnswerUnknownConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/rpc_answer_dropped_running>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct RpcAnswerDroppedRunningConstructor {}
impl RpcAnswerDroppedRunningConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xcd78_e586;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "rpc_answer_dropped_running";
    /// Original TL result expression.
    pub const RESULT: &'static str = "RpcDropAnswer";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for RpcAnswerDroppedRunningConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for RpcAnswerDroppedRunningConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for RpcAnswerDroppedRunningConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/rpc_answer_dropped>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct RpcAnswerDroppedConstructor {
    ///TL field `msg_id`.
    pub msg_id: i64,
    ///TL field `seq_no`.
    pub seq_no: i32,
    ///TL field `bytes`.
    pub bytes: i32,
}
impl RpcAnswerDroppedConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xa43a_d8b7;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "rpc_answer_dropped";
    /// Original TL result expression.
    pub const RESULT: &'static str = "RpcDropAnswer";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for RpcAnswerDroppedConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.msg_id, encoder)?;
        crate::codec::TlEncode::encode(&self.seq_no, encoder)?;
        crate::codec::TlEncode::encode(&self.bytes, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for RpcAnswerDroppedConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for RpcAnswerDroppedConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let msg_id = crate::codec::TlDecode::decode(decoder)?;
        let seq_no = crate::codec::TlDecode::decode(decoder)?;
        let bytes = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { msg_id, seq_no, bytes })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/future_salt>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct FutureSaltConstructor {
    ///TL field `valid_since`.
    pub valid_since: i32,
    ///TL field `valid_until`.
    pub valid_until: i32,
    ///TL field `salt`.
    pub salt: i64,
}
impl FutureSaltConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x0949_d9dc;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "future_salt";
    /// Original TL result expression.
    pub const RESULT: &'static str = "FutureSalt";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for FutureSaltConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.valid_since, encoder)?;
        crate::codec::TlEncode::encode(&self.valid_until, encoder)?;
        crate::codec::TlEncode::encode(&self.salt, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for FutureSaltConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for FutureSaltConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let valid_since = crate::codec::TlDecode::decode(decoder)?;
        let valid_until = crate::codec::TlDecode::decode(decoder)?;
        let salt = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            valid_since,
            valid_until,
            salt,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/future_salts>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct FutureSaltsConstructor {
    ///TL field `req_msg_id`.
    pub req_msg_id: i64,
    ///TL field `now`.
    pub now: i32,
    ///TL field `salts`.
    pub salts: Box<Vector<Box<FutureSalt>>>,
}
impl FutureSaltsConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xae50_0895;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "future_salts";
    /// Original TL result expression.
    pub const RESULT: &'static str = "FutureSalts";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for FutureSaltsConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.req_msg_id, encoder)?;
        crate::codec::TlEncode::encode(&self.now, encoder)?;
        crate::codec::TlEncode::encode(&self.salts, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for FutureSaltsConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for FutureSaltsConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let req_msg_id = crate::codec::TlDecode::decode(decoder)?;
        let now = crate::codec::TlDecode::decode(decoder)?;
        let salts = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { req_msg_id, now, salts })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/pong>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct PongConstructor {
    ///TL field `msg_id`.
    pub msg_id: i64,
    ///TL field `ping_id`.
    pub ping_id: i64,
}
impl PongConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x3477_73c5;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "pong";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Pong";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for PongConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.msg_id, encoder)?;
        crate::codec::TlEncode::encode(&self.ping_id, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for PongConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for PongConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let msg_id = crate::codec::TlDecode::decode(decoder)?;
        let ping_id = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { msg_id, ping_id })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/destroy_session_ok>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DestroySessionOkConstructor {
    ///TL field `session_id`.
    pub session_id: i64,
}
impl DestroySessionOkConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xe220_45fc;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "destroy_session_ok";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DestroySessionRes";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for DestroySessionOkConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.session_id, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DestroySessionOkConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DestroySessionOkConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let session_id = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { session_id })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/destroy_session_none>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DestroySessionNoneConstructor {
    ///TL field `session_id`.
    pub session_id: i64,
}
impl DestroySessionNoneConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x62d3_50c9;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "destroy_session_none";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DestroySessionRes";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for DestroySessionNoneConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.session_id, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DestroySessionNoneConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DestroySessionNoneConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let session_id = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { session_id })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/new_session_created>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct NewSessionCreatedConstructor {
    ///TL field `first_msg_id`.
    pub first_msg_id: i64,
    ///TL field `unique_id`.
    pub unique_id: i64,
    ///TL field `server_salt`.
    pub server_salt: i64,
}
impl NewSessionCreatedConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x9ec2_0908;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "new_session_created";
    /// Original TL result expression.
    pub const RESULT: &'static str = "NewSession";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for NewSessionCreatedConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.first_msg_id, encoder)?;
        crate::codec::TlEncode::encode(&self.unique_id, encoder)?;
        crate::codec::TlEncode::encode(&self.server_salt, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for NewSessionCreatedConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for NewSessionCreatedConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let first_msg_id = crate::codec::TlDecode::decode(decoder)?;
        let unique_id = crate::codec::TlDecode::decode(decoder)?;
        let server_salt = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            first_msg_id,
            unique_id,
            server_salt,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/msg_container>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MsgContainerConstructor {
    ///TL field `messages`.
    pub messages: Box<Vector<Box<Message>>>,
}
impl MsgContainerConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x73f1_f8dc;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "msg_container";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MessageContainer";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for MsgContainerConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.messages, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MsgContainerConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MsgContainerConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let messages = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { messages })
    }
}
///A message
///Official Telegram documentation: <https://core.telegram.org/constructor/message>.
///Related protocol documentation: <https://core.telegram.org/api/ai#summarize-messages>.
///Related protocol documentation: <https://core.telegram.org/api/boost>.
///Related protocol documentation: <https://core.telegram.org/api/bots/connected-business-bots>.
///Related protocol documentation: <https://core.telegram.org/api/bots/guest-mode#guest-messages>.
///Related protocol documentation: <https://core.telegram.org/api/business#quick-reply-shortcuts>.
///Related protocol documentation: <https://core.telegram.org/api/channel#supergroups>.
///Related protocol documentation: <https://core.telegram.org/api/effects>.
///Related protocol documentation: <https://core.telegram.org/api/entities>.
///Related protocol documentation: <https://core.telegram.org/api/factcheck>.
///Related protocol documentation: <https://core.telegram.org/api/files#albums-grouped-media>.
///Related protocol documentation: <https://core.telegram.org/api/files#video-qualities>.
///Related protocol documentation: <https://core.telegram.org/api/mentions>.
///Related protocol documentation: <https://core.telegram.org/api/monoforum>.
///Related protocol documentation: <https://core.telegram.org/api/paid-messages>.
///Related protocol documentation: <https://core.telegram.org/api/pin>.
///Related protocol documentation: <https://core.telegram.org/api/rank>.
///Related protocol documentation: <https://core.telegram.org/api/saved-messages>.
///Related protocol documentation: <https://core.telegram.org/api/scheduled-messages>.
///Related protocol documentation: <https://core.telegram.org/api/scheduled-messages#repeating-scheduled-messages>.
///Related protocol documentation: <https://core.telegram.org/api/stars>.
///Related protocol documentation: <https://core.telegram.org/api/suggested-posts>.
///Related protocol documentation: <https://core.telegram.org/api/threads>.
///Related protocol documentation: <https://core.telegram.org/api/views#read-message-contents>.
///Related protocol documentation: <https://core.telegram.org/constructor/channelFull>.
///Related protocol documentation: <https://core.telegram.org/constructor/inputPeerSelf>.
///Related protocol documentation: <https://core.telegram.org/constructor/messageMediaDocument>.
///Related protocol documentation: <https://core.telegram.org/constructor/true>.
///Related protocol documentation: <https://core.telegram.org/method/messages.reportMessagesDelivery>.
///Related protocol documentation: <https://core.telegram.org/method/messages.sendMultiMedia>.
///Related protocol documentation: <https://core.telegram.org/mtproto/TL-combinators#conditional-fields>.
///Related protocol documentation: <https://core.telegram.org/type/%23>.
///Related protocol documentation: <https://core.telegram.org/type/FactCheck>.
///Related protocol documentation: <https://core.telegram.org/type/MessageEntity>.
///Related protocol documentation: <https://core.telegram.org/type/MessageFwdHeader>.
///Related protocol documentation: <https://core.telegram.org/type/MessageMedia>.
///Related protocol documentation: <https://core.telegram.org/type/MessageReactions>.
///Related protocol documentation: <https://core.telegram.org/type/MessageReplies>.
///Related protocol documentation: <https://core.telegram.org/type/MessageReplyHeader>.
///Related protocol documentation: <https://core.telegram.org/type/Peer>.
///Related protocol documentation: <https://core.telegram.org/type/ReplyMarkup>.
///Related protocol documentation: <https://core.telegram.org/type/RestrictionReason>.
///Related protocol documentation: <https://core.telegram.org/type/SuggestedPost>.
///Related protocol documentation: <https://core.telegram.org/type/Vector%20t>.
///Related protocol documentation: <https://core.telegram.org/type/int>.
///Related protocol documentation: <https://core.telegram.org/type/long>.
///Related protocol documentation: <https://core.telegram.org/type/string>.
///Related protocol documentation: <https://telegram.org/blog/content-protection-delete-by-date-and-more>.
///Related protocol documentation: <https://telegram.org/blog/star-messages-gateway-2-0-and-more#save-even-more-on-user-verification>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MessageConstructor {
    ///TL field `msg_id`.
    pub msg_id: i64,
    ///TL field `seqno`.
    pub seqno: i32,
    ///TL field `bytes`.
    pub bytes: i32,
    ///TL field `body`.
    pub body: Box<Object>,
}
impl MessageConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x5bb8_e511;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = false;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "message";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Message";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for MessageConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.msg_id, encoder)?;
        crate::codec::TlEncode::encode(&self.seqno, encoder)?;
        crate::codec::TlEncode::encode(&self.bytes, encoder)?;
        crate::codec::TlEncode::encode(&self.body, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MessageConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MessageConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let msg_id = crate::codec::TlDecode::decode(decoder)?;
        let seqno = crate::codec::TlDecode::decode(decoder)?;
        let bytes = crate::codec::TlDecode::decode(decoder)?;
        let body = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { msg_id, seqno, bytes, body })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/msg_copy>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MsgCopyConstructor {
    ///TL field `orig_message`.
    pub orig_message: Box<Message>,
}
impl MsgCopyConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xe060_46b2;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "msg_copy";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MessageCopy";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for MsgCopyConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.orig_message, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MsgCopyConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MsgCopyConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let orig_message = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { orig_message })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/gzip_packed>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct GzipPackedConstructor {
    ///TL field `packed_data`.
    pub packed_data: Vec<u8>,
}
impl GzipPackedConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x3072_cfa1;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "gzip_packed";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Object";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for GzipPackedConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::encode_bytes(&self.packed_data, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for GzipPackedConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for GzipPackedConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let packed_data = crate::codec::decode_bytes(decoder)?;
        Ok(Self { packed_data })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/msgs_ack>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MsgsAckConstructor {
    ///TL field `msg_ids`.
    pub msg_ids: Box<Vector<i64>>,
}
impl MsgsAckConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x62d6_b459;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "msgs_ack";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MsgsAck";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for MsgsAckConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.msg_ids, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MsgsAckConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MsgsAckConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let msg_ids = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { msg_ids })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/bad_msg_notification>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct BadMsgNotificationConstructor {
    ///TL field `bad_msg_id`.
    pub bad_msg_id: i64,
    ///TL field `bad_msg_seqno`.
    pub bad_msg_seqno: i32,
    ///TL field `error_code`.
    pub error_code: i32,
}
impl BadMsgNotificationConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xa7ef_f811;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "bad_msg_notification";
    /// Original TL result expression.
    pub const RESULT: &'static str = "BadMsgNotification";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for BadMsgNotificationConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.bad_msg_id, encoder)?;
        crate::codec::TlEncode::encode(&self.bad_msg_seqno, encoder)?;
        crate::codec::TlEncode::encode(&self.error_code, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for BadMsgNotificationConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for BadMsgNotificationConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let bad_msg_id = crate::codec::TlDecode::decode(decoder)?;
        let bad_msg_seqno = crate::codec::TlDecode::decode(decoder)?;
        let error_code = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            bad_msg_id,
            bad_msg_seqno,
            error_code,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/bad_server_salt>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct BadServerSaltConstructor {
    ///TL field `bad_msg_id`.
    pub bad_msg_id: i64,
    ///TL field `bad_msg_seqno`.
    pub bad_msg_seqno: i32,
    ///TL field `error_code`.
    pub error_code: i32,
    ///TL field `new_server_salt`.
    pub new_server_salt: i64,
}
impl BadServerSaltConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xedab_447b;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "bad_server_salt";
    /// Original TL result expression.
    pub const RESULT: &'static str = "BadMsgNotification";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for BadServerSaltConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.bad_msg_id, encoder)?;
        crate::codec::TlEncode::encode(&self.bad_msg_seqno, encoder)?;
        crate::codec::TlEncode::encode(&self.error_code, encoder)?;
        crate::codec::TlEncode::encode(&self.new_server_salt, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for BadServerSaltConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for BadServerSaltConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let bad_msg_id = crate::codec::TlDecode::decode(decoder)?;
        let bad_msg_seqno = crate::codec::TlDecode::decode(decoder)?;
        let error_code = crate::codec::TlDecode::decode(decoder)?;
        let new_server_salt = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            bad_msg_id,
            bad_msg_seqno,
            error_code,
            new_server_salt,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/msg_resend_req>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MsgResendReqConstructor {
    ///TL field `msg_ids`.
    pub msg_ids: Box<Vector<i64>>,
}
impl MsgResendReqConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x7d86_1a08;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "msg_resend_req";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MsgResendReq";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for MsgResendReqConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.msg_ids, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MsgResendReqConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MsgResendReqConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let msg_ids = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { msg_ids })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/msgs_state_req>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MsgsStateReqConstructor {
    ///TL field `msg_ids`.
    pub msg_ids: Box<Vector<i64>>,
}
impl MsgsStateReqConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xda69_fb52;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "msgs_state_req";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MsgsStateReq";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for MsgsStateReqConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.msg_ids, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MsgsStateReqConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MsgsStateReqConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let msg_ids = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { msg_ids })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/msgs_state_info>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MsgsStateInfoConstructor {
    ///TL field `req_msg_id`.
    pub req_msg_id: i64,
    ///TL field `info`.
    pub info: Vec<u8>,
}
impl MsgsStateInfoConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x04de_b57d;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "msgs_state_info";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MsgsStateInfo";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for MsgsStateInfoConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.req_msg_id, encoder)?;
        crate::codec::encode_bytes(&self.info, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MsgsStateInfoConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MsgsStateInfoConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let req_msg_id = crate::codec::TlDecode::decode(decoder)?;
        let info = crate::codec::decode_bytes(decoder)?;
        Ok(Self { req_msg_id, info })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/msgs_all_info>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MsgsAllInfoConstructor {
    ///TL field `msg_ids`.
    pub msg_ids: Box<Vector<i64>>,
    ///TL field `info`.
    pub info: Vec<u8>,
}
impl MsgsAllInfoConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x8cc0_d131;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "msgs_all_info";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MsgsAllInfo";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for MsgsAllInfoConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.msg_ids, encoder)?;
        crate::codec::encode_bytes(&self.info, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MsgsAllInfoConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MsgsAllInfoConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let msg_ids = crate::codec::TlDecode::decode(decoder)?;
        let info = crate::codec::decode_bytes(decoder)?;
        Ok(Self { msg_ids, info })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/msg_detailed_info>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MsgDetailedInfoConstructor {
    ///TL field `msg_id`.
    pub msg_id: i64,
    ///TL field `answer_msg_id`.
    pub answer_msg_id: i64,
    ///TL field `bytes`.
    pub bytes: i32,
    ///TL field `status`.
    pub status: i32,
}
impl MsgDetailedInfoConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x276d_3ec6;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "msg_detailed_info";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MsgDetailedInfo";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for MsgDetailedInfoConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.msg_id, encoder)?;
        crate::codec::TlEncode::encode(&self.answer_msg_id, encoder)?;
        crate::codec::TlEncode::encode(&self.bytes, encoder)?;
        crate::codec::TlEncode::encode(&self.status, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MsgDetailedInfoConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MsgDetailedInfoConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let msg_id = crate::codec::TlDecode::decode(decoder)?;
        let answer_msg_id = crate::codec::TlDecode::decode(decoder)?;
        let bytes = crate::codec::TlDecode::decode(decoder)?;
        let status = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            msg_id,
            answer_msg_id,
            bytes,
            status,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/msg_new_detailed_info>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct MsgNewDetailedInfoConstructor {
    ///TL field `answer_msg_id`.
    pub answer_msg_id: i64,
    ///TL field `bytes`.
    pub bytes: i32,
    ///TL field `status`.
    pub status: i32,
}
impl MsgNewDetailedInfoConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x809d_b6df;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "msg_new_detailed_info";
    /// Original TL result expression.
    pub const RESULT: &'static str = "MsgDetailedInfo";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for MsgNewDetailedInfoConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.answer_msg_id, encoder)?;
        crate::codec::TlEncode::encode(&self.bytes, encoder)?;
        crate::codec::TlEncode::encode(&self.status, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for MsgNewDetailedInfoConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for MsgNewDetailedInfoConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let answer_msg_id = crate::codec::TlDecode::decode(decoder)?;
        let bytes = crate::codec::TlDecode::decode(decoder)?;
        let status = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            answer_msg_id,
            bytes,
            status,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/destroy_auth_key_ok>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DestroyAuthKeyOkConstructor {}
impl DestroyAuthKeyOkConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xf660_e1d4;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "destroy_auth_key_ok";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DestroyAuthKeyRes";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for DestroyAuthKeyOkConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for DestroyAuthKeyOkConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DestroyAuthKeyOkConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/destroy_auth_key_none>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DestroyAuthKeyNoneConstructor {}
impl DestroyAuthKeyNoneConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x0a9f_2259;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "destroy_auth_key_none";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DestroyAuthKeyRes";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for DestroyAuthKeyNoneConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for DestroyAuthKeyNoneConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DestroyAuthKeyNoneConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/destroy_auth_key_fail>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DestroyAuthKeyFailConstructor {}
impl DestroyAuthKeyFailConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0xea10_9b13;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "destroy_auth_key_fail";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DestroyAuthKeyRes";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for DestroyAuthKeyFailConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for DestroyAuthKeyFailConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DestroyAuthKeyFailConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
///Official Telegram documentation: <https://core.telegram.org/constructor/http_wait>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct HttpWaitConstructor {
    ///TL field `max_delay`.
    pub max_delay: i32,
    ///TL field `wait_after`.
    pub wait_after: i32,
    ///TL field `max_wait`.
    pub max_wait: i32,
}
impl HttpWaitConstructor {
    /// TL constructor number.
    pub const ID: u32 = 0x9299_359f;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "http_wait";
    /// Original TL result expression.
    pub const RESULT: &'static str = "HttpWait";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for HttpWaitConstructor {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.max_delay, encoder)?;
        crate::codec::TlEncode::encode(&self.wait_after, encoder)?;
        crate::codec::TlEncode::encode(&self.max_wait, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for HttpWaitConstructor {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for HttpWaitConstructor {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let max_delay = crate::codec::TlDecode::decode(decoder)?;
        let wait_after = crate::codec::TlDecode::decode(decoder)?;
        let max_wait = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self {
            max_delay,
            wait_after,
            max_wait,
        })
    }
}
///Official Telegram documentation: <https://core.telegram.org/method/req_pq_multi>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct ReqPqMultiRequest {
    ///TL field `nonce`.
    pub nonce: bnum::types::I128,
}
impl ReqPqMultiRequest {
    /// TL constructor number.
    pub const ID: u32 = 0xbe7e_8ef1;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "req_pq_multi";
    /// Original TL result expression.
    pub const RESULT: &'static str = "ResPQ";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for ReqPqMultiRequest {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.nonce, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for ReqPqMultiRequest {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for ReqPqMultiRequest {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let nonce = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { nonce })
    }
}
impl crate::codec::RpcMethod for ReqPqMultiRequest
where
    Box<ResPq>: crate::codec::TlDecode,
{
    type Response = Box<ResPq>;
    const METHOD_NAME: &'static str = Self::NAME;
    const RESULT_TYPE: &'static str = Self::RESULT;
}
///Official Telegram documentation: <https://core.telegram.org/method/req_DH_params>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct ReqDhParamsRequest {
    ///TL field `nonce`.
    pub nonce: bnum::types::I128,
    ///TL field `server_nonce`.
    pub server_nonce: bnum::types::I128,
    ///TL field `p`.
    pub p: Vec<u8>,
    ///TL field `q`.
    pub q: Vec<u8>,
    ///TL field `public_key_fingerprint`.
    pub public_key_fingerprint: i64,
    ///TL field `encrypted_data`.
    pub encrypted_data: Vec<u8>,
}
impl ReqDhParamsRequest {
    /// TL constructor number.
    pub const ID: u32 = 0xd712_e4be;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "req_DH_params";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Server_DH_Params";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for ReqDhParamsRequest {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.server_nonce, encoder)?;
        crate::codec::encode_bytes(&self.p, encoder)?;
        crate::codec::encode_bytes(&self.q, encoder)?;
        crate::codec::TlEncode::encode(&self.public_key_fingerprint, encoder)?;
        crate::codec::encode_bytes(&self.encrypted_data, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for ReqDhParamsRequest {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for ReqDhParamsRequest {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let nonce = crate::codec::TlDecode::decode(decoder)?;
        let server_nonce = crate::codec::TlDecode::decode(decoder)?;
        let p = crate::codec::decode_bytes(decoder)?;
        let q = crate::codec::decode_bytes(decoder)?;
        let public_key_fingerprint = crate::codec::TlDecode::decode(decoder)?;
        let encrypted_data = crate::codec::decode_bytes(decoder)?;
        Ok(Self {
            nonce,
            server_nonce,
            p,
            q,
            public_key_fingerprint,
            encrypted_data,
        })
    }
}
impl crate::codec::RpcMethod for ReqDhParamsRequest
where
    Box<ServerDhParams>: crate::codec::TlDecode,
{
    type Response = Box<ServerDhParams>;
    const METHOD_NAME: &'static str = Self::NAME;
    const RESULT_TYPE: &'static str = Self::RESULT;
}
///Official Telegram documentation: <https://core.telegram.org/method/set_client_DH_params>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct SetClientDhParamsRequest {
    ///TL field `nonce`.
    pub nonce: bnum::types::I128,
    ///TL field `server_nonce`.
    pub server_nonce: bnum::types::I128,
    ///TL field `encrypted_data`.
    pub encrypted_data: Vec<u8>,
}
impl SetClientDhParamsRequest {
    /// TL constructor number.
    pub const ID: u32 = 0xf504_5f1f;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "set_client_DH_params";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Set_client_DH_params_answer";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for SetClientDhParamsRequest {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.nonce, encoder)?;
        crate::codec::TlEncode::encode(&self.server_nonce, encoder)?;
        crate::codec::encode_bytes(&self.encrypted_data, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for SetClientDhParamsRequest {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for SetClientDhParamsRequest {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let nonce = crate::codec::TlDecode::decode(decoder)?;
        let server_nonce = crate::codec::TlDecode::decode(decoder)?;
        let encrypted_data = crate::codec::decode_bytes(decoder)?;
        Ok(Self {
            nonce,
            server_nonce,
            encrypted_data,
        })
    }
}
impl crate::codec::RpcMethod for SetClientDhParamsRequest
where
    Box<SetClientDhParamsAnswer>: crate::codec::TlDecode,
{
    type Response = Box<SetClientDhParamsAnswer>;
    const METHOD_NAME: &'static str = Self::NAME;
    const RESULT_TYPE: &'static str = Self::RESULT;
}
///Official Telegram documentation: <https://core.telegram.org/method/rpc_drop_answer>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct RpcDropAnswerRequest {
    ///TL field `req_msg_id`.
    pub req_msg_id: i64,
}
impl RpcDropAnswerRequest {
    /// TL constructor number.
    pub const ID: u32 = 0x58e4_a740;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "rpc_drop_answer";
    /// Original TL result expression.
    pub const RESULT: &'static str = "RpcDropAnswer";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for RpcDropAnswerRequest {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.req_msg_id, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for RpcDropAnswerRequest {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for RpcDropAnswerRequest {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let req_msg_id = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { req_msg_id })
    }
}
impl crate::codec::RpcMethod for RpcDropAnswerRequest
where
    Box<RpcDropAnswer>: crate::codec::TlDecode,
{
    type Response = Box<RpcDropAnswer>;
    const METHOD_NAME: &'static str = Self::NAME;
    const RESULT_TYPE: &'static str = Self::RESULT;
}
///Official Telegram documentation: <https://core.telegram.org/method/get_future_salts>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct GetFutureSaltsRequest {
    ///TL field `num`.
    pub num: i32,
}
impl GetFutureSaltsRequest {
    /// TL constructor number.
    pub const ID: u32 = 0xb921_bd04;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "get_future_salts";
    /// Original TL result expression.
    pub const RESULT: &'static str = "FutureSalts";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for GetFutureSaltsRequest {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.num, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for GetFutureSaltsRequest {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for GetFutureSaltsRequest {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let num = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { num })
    }
}
impl crate::codec::RpcMethod for GetFutureSaltsRequest
where
    Box<FutureSalts>: crate::codec::TlDecode,
{
    type Response = Box<FutureSalts>;
    const METHOD_NAME: &'static str = Self::NAME;
    const RESULT_TYPE: &'static str = Self::RESULT;
}
///Official Telegram documentation: <https://core.telegram.org/method/ping>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct PingRequest {
    ///TL field `ping_id`.
    pub ping_id: i64,
}
impl PingRequest {
    /// TL constructor number.
    pub const ID: u32 = 0x7abe_77ec;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "ping";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Pong";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for PingRequest {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.ping_id, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for PingRequest {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for PingRequest {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let ping_id = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { ping_id })
    }
}
impl crate::codec::RpcMethod for PingRequest
where
    Box<Pong>: crate::codec::TlDecode,
{
    type Response = Box<Pong>;
    const METHOD_NAME: &'static str = Self::NAME;
    const RESULT_TYPE: &'static str = Self::RESULT;
}
///Official Telegram documentation: <https://core.telegram.org/method/ping_delay_disconnect>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct PingDelayDisconnectRequest {
    ///TL field `ping_id`.
    pub ping_id: i64,
    ///TL field `disconnect_delay`.
    pub disconnect_delay: i32,
}
impl PingDelayDisconnectRequest {
    /// TL constructor number.
    pub const ID: u32 = 0xf342_7b8c;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "ping_delay_disconnect";
    /// Original TL result expression.
    pub const RESULT: &'static str = "Pong";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for PingDelayDisconnectRequest {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.ping_id, encoder)?;
        crate::codec::TlEncode::encode(&self.disconnect_delay, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for PingDelayDisconnectRequest {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for PingDelayDisconnectRequest {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let ping_id = crate::codec::TlDecode::decode(decoder)?;
        let disconnect_delay = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { ping_id, disconnect_delay })
    }
}
impl crate::codec::RpcMethod for PingDelayDisconnectRequest
where
    Box<Pong>: crate::codec::TlDecode,
{
    type Response = Box<Pong>;
    const METHOD_NAME: &'static str = Self::NAME;
    const RESULT_TYPE: &'static str = Self::RESULT;
}
///Official Telegram documentation: <https://core.telegram.org/method/destroy_session>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DestroySessionRequest {
    ///TL field `session_id`.
    pub session_id: i64,
}
impl DestroySessionRequest {
    /// TL constructor number.
    pub const ID: u32 = 0xe751_2126;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "destroy_session";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DestroySessionRes";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for DestroySessionRequest {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        crate::codec::TlEncode::encode(&self.session_id, encoder)?;
        Ok(())
    }
}
impl crate::codec::Boxed for DestroySessionRequest {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DestroySessionRequest {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        let session_id = crate::codec::TlDecode::decode(decoder)?;
        Ok(Self { session_id })
    }
}
impl crate::codec::RpcMethod for DestroySessionRequest
where
    Box<DestroySessionRes>: crate::codec::TlDecode,
{
    type Response = Box<DestroySessionRes>;
    const METHOD_NAME: &'static str = Self::NAME;
    const RESULT_TYPE: &'static str = Self::RESULT;
}
///Official Telegram documentation: <https://core.telegram.org/method/destroy_auth_key>.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
pub struct DestroyAuthKeyRequest {}
impl DestroyAuthKeyRequest {
    /// TL constructor number.
    pub const ID: u32 = 0xd143_5160;
    /// Whether [`Self::ID`] was written explicitly in the source schema.
    pub const ID_EXPLICIT: bool = true;
    /// Original namespace-qualified TL name.
    pub const NAME: &'static str = "destroy_auth_key";
    /// Original TL result expression.
    pub const RESULT: &'static str = "DestroyAuthKeyRes";
    /// Layer of the containing schema, when declared by the input.
    pub const SCHEMA_LAYER: Option<u32> = None;
    /// First layer, only when explicitly supplied by source metadata.
    pub const INTRODUCED_LAYER: Option<u32> = None;
}
impl crate::codec::TlEncode for DestroyAuthKeyRequest {
    fn encode(
        &self,
        encoder: &mut crate::codec::Encoder,
    ) -> Result<(), crate::codec::Error> {
        let _ = encoder;
        Ok(())
    }
}
impl crate::codec::Boxed for DestroyAuthKeyRequest {
    const CONSTRUCTOR_ID: u32 = Self::ID;
}
impl crate::codec::TlDecode for DestroyAuthKeyRequest {
    fn decode(
        decoder: &mut crate::codec::Decoder<'_>,
    ) -> Result<Self, crate::codec::Error> {
        let _ = decoder;
        Ok(Self {})
    }
}
impl crate::codec::RpcMethod for DestroyAuthKeyRequest
where
    Box<DestroyAuthKeyRes>: crate::codec::TlDecode,
{
    type Response = Box<DestroyAuthKeyRes>;
    const METHOD_NAME: &'static str = Self::NAME;
    const RESULT_TYPE: &'static str = Self::RESULT;
}
