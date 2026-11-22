//! Text translation, message summaries and the AI composer.
//!
//! * translation: <https://core.telegram.org/method/messages.translateText>
//! * summaries: <https://core.telegram.org/method/messages.summarizeText>
//! * AI composer + tones: <https://core.telegram.org/api/ai>
//!
//! Message/quote bodies never appear in logs; only counts and constructor ids.

use crate::access::{ensure_ready, with_client_mut};
use crate::media_download::with_peer_refresh;
use crate::session::call_with_migrate;
use crate::state::ClientState;
use mono_proto::latest::api::*;
use mono_proto_runtime::api_invoke::invoke_api;
use mono_proto_types::{
    error::MtprotoError,
    message_entities::{entities_from_json, entities_to_json},
    peers::{
        CachedPeer, PeerKind, input_peer_from_cached, require_usable_peer, vector_boxed_items,
    },
};

/// Android batches at most 20 messages per `messages.translateText` call.
pub const MAX_TRANSLATION_MESSAGES: usize = 20;
/// Android caps a translation batch at 25000 UTF-16 code units.
pub const MAX_TRANSLATION_UTF16_UNITS: usize = 25_000;
/// `TranslateController.isSummarizable` only offers summaries above 100 chars.
pub const MIN_SUMMARY_CHARS: usize = 100;

/// Rich text plus its `{kind,offset,length,url?,text?}` entity JSON.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextToolTextDto {
    pub text: String,
    pub entities_json: Option<String>,
}

/// AI composer output: the transformed text and, for proofreading, the diff.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AiTextResultDto {
    pub result: TextToolTextDto,
    pub diff: Option<TextToolTextDto>,
}

/// One AI compose tone. `key` is `default:<tone>` for server defaults and the
/// slug for saved custom tones.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AiTextToneDto {
    pub key: String,
    pub title: String,
    pub emoji_id: i64,
    pub creator: bool,
    pub prompt: Option<String>,
}

/// `translations_manual_enabled` / `translations_auto_enabled` values from
/// `help.getAppConfig` (<https://core.telegram.org/api/config>).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TranslationMode {
    /// `enabled`: server-side translation is available.
    #[default]
    Enabled,
    /// `alternative`: device-side/alternative provider only.
    Alternative,
    /// `system`: OS-provided translator only.
    System,
    /// `disabled`: no translation at all.
    Disabled,
}

impl TranslationMode {
    pub fn parse(raw: Option<&str>) -> Self {
        match raw.map(str::trim) {
            Some("alternative") => Self::Alternative,
            Some("system") => Self::System,
            Some("disabled") => Self::Disabled,
            // `enabled` and unknown/absent values behave like Android.
            _ => Self::Enabled,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Alternative => "alternative",
            Self::System => "system",
            Self::Disabled => "disabled",
        }
    }

    pub fn is_enabled(self) -> bool {
        !matches!(self, Self::Disabled)
    }
}

/// Translation availability published by the server.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextToolsConfig {
    pub translations_manual_enabled: TranslationMode,
    pub translations_auto_enabled: TranslationMode,
}

impl TextToolsConfig {
    pub fn manual_enabled(&self) -> bool {
        self.translations_manual_enabled.is_enabled()
    }

    pub fn auto_enabled(&self) -> bool {
        self.translations_auto_enabled.is_enabled()
    }

    /// Normalized `enabled|alternative|system|disabled` values for the bridge.
    pub fn dto(&self) -> TextToolsConfigDto {
        TextToolsConfigDto {
            translations_manual_enabled: self.translations_manual_enabled.as_str().to_string(),
            translations_auto_enabled: self.translations_auto_enabled.as_str().to_string(),
        }
    }
}

/// `translations_manual_enabled` / `translations_auto_enabled` as the bridge
/// consumes them: always one of `enabled`, `alternative`, `system`, `disabled`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextToolsConfigDto {
    pub translations_manual_enabled: String,
    pub translations_auto_enabled: String,
}

/// What the UI may offer for one chat. Field order is the Kotlin positional
/// constructor order, keep it stable.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextToolsAvailabilityDto {
    pub manual_translation: bool,
    pub chat_translation: bool,
    pub peer_translations_disabled: bool,
    pub channel_auto_translation: bool,
}

fn vector<T>(items: Vec<T>) -> Box<Vector<T>> {
    Box::new(Vector::Vector(VectorConstructor {
        field_0: items.len() as u32,
        field_1: items,
    }))
}

fn true_flag() -> Box<True> {
    Box::new(True::True(TrueConstructor {}))
}

fn tl_bool(value: bool) -> Box<Bool> {
    Box::new(if value {
        Bool::BoolTrue(BoolTrueConstructor {})
    } else {
        Bool::BoolFalse(BoolFalseConstructor {})
    })
}

// ---------------------------------------------------------------- conversions

pub fn text_to_tl(value: &TextToolTextDto) -> Result<Box<TextWithEntities>, MtprotoError> {
    let (_, entities) = entities_from_json(value.entities_json.as_deref())?;
    Ok(Box::new(TextWithEntities::TextWithEntities(
        TextWithEntitiesConstructor {
            text: value.text.clone(),
            entities: entities.unwrap_or_else(|| vector(vec![])),
        },
    )))
}

pub fn text_from_tl(value: &TextWithEntities) -> TextToolTextDto {
    let TextWithEntities::TextWithEntities(value) = value;
    TextToolTextDto {
        text: value.text.clone(),
        entities_json: entities_to_json(Some(&value.entities)),
    }
}

/// `default:<tone>` addresses `inputAiComposeToneDefault`; anything else is a
/// slug (<https://core.telegram.org/constructor/inputAiComposeToneSlug>).
pub fn input_tone(key: &str) -> Result<Box<InputAiComposeTone>, MtprotoError> {
    let key = key.trim();
    if key.is_empty() {
        return Err(MtprotoError::Message("AI tone key required".into()));
    }
    if let Some(tone) = key.strip_prefix("default:") {
        let tone = tone.trim();
        if tone.is_empty() {
            return Err(MtprotoError::Message("AI tone name required".into()));
        }
        return Ok(Box::new(InputAiComposeTone::InputAiComposeToneDefault(
            InputAiComposeToneDefaultConstructor {
                tone: tone.to_string(),
            },
        )));
    }
    Ok(Box::new(InputAiComposeTone::InputAiComposeToneSlug(
        InputAiComposeToneSlugConstructor {
            slug: key.to_string(),
        },
    )))
}

pub fn tone_from_tl(tone: &AiComposeTone) -> AiTextToneDto {
    match tone {
        AiComposeTone::AiComposeToneDefault(t) => AiTextToneDto {
            key: format!("default:{}", t.tone),
            title: t.title.clone(),
            emoji_id: t.emoji_id,
            creator: false,
            prompt: None,
        },
        AiComposeTone::AiComposeTone(t) => AiTextToneDto {
            key: t.slug.clone(),
            title: t.title.clone(),
            emoji_id: t.emoji_id.unwrap_or_default(),
            creator: t.creator.is_some(),
            prompt: t.prompt.clone(),
        },
    }
}

// ------------------------------------------------------------ request builders

pub fn translate_messages_request(
    peer: &CachedPeer,
    message_ids: &[i32],
    to_lang: &str,
) -> Result<MessagesTranslateTextRequest, MtprotoError> {
    if message_ids.is_empty() || message_ids.len() > MAX_TRANSLATION_MESSAGES {
        return Err(MtprotoError::Message(
            "translation requires 1..=20 message ids".into(),
        ));
    }
    if message_ids.iter().any(|id| *id <= 0) {
        return Err(MtprotoError::Message(
            "translation message ids must be positive".into(),
        ));
    }
    if to_lang.trim().is_empty() {
        return Err(MtprotoError::Message(
            "translation language required".into(),
        ));
    }
    let mut unique = message_ids.to_vec();
    unique.sort_unstable();
    unique.dedup();
    if unique.len() != message_ids.len() {
        return Err(MtprotoError::Message(
            "translation message ids must be unique".into(),
        ));
    }
    Ok(MessagesTranslateTextRequest {
        flags: MessagesTranslateTextRequest::PEER_FLAG,
        peer: Some(Box::new(input_peer_from_cached(peer))),
        id: Some(vector(message_ids.to_vec())),
        text: None,
        to_lang: to_lang.to_string(),
        tone: None,
    })
}

pub fn translate_texts_request(
    texts: &[TextToolTextDto],
    to_lang: &str,
) -> Result<MessagesTranslateTextRequest, MtprotoError> {
    if texts.is_empty() || texts.len() > MAX_TRANSLATION_MESSAGES {
        return Err(MtprotoError::Message(
            "translation requires 1..=20 texts".into(),
        ));
    }
    let units: usize = texts
        .iter()
        .map(|text| text.text.encode_utf16().count())
        .sum();
    if units > MAX_TRANSLATION_UTF16_UNITS {
        return Err(MtprotoError::Message(format!(
            "translation batch of {units} UTF-16 units exceeds {MAX_TRANSLATION_UTF16_UNITS}"
        )));
    }
    if texts.iter().any(|text| text.text.trim().is_empty()) {
        return Err(MtprotoError::Message(
            "translation texts must not be blank".into(),
        ));
    }
    if to_lang.trim().is_empty() {
        return Err(MtprotoError::Message(
            "translation language required".into(),
        ));
    }
    let mut payload = Vec::with_capacity(texts.len());
    for text in texts {
        payload.push(text_to_tl(text)?);
    }
    Ok(MessagesTranslateTextRequest {
        flags: MessagesTranslateTextRequest::TEXT_FLAG,
        peer: None,
        id: None,
        text: Some(vector(payload)),
        to_lang: to_lang.to_string(),
        tone: None,
    })
}

pub fn summarize_request(
    peer: &CachedPeer,
    message_id: i32,
    to_lang: Option<&str>,
    tone: Option<&str>,
) -> Result<MessagesSummarizeTextRequest, MtprotoError> {
    if message_id <= 0 {
        return Err(MtprotoError::Message("summary message id required".into()));
    }
    let to_lang = to_lang
        .map(str::trim)
        .filter(|lang| !lang.is_empty())
        .map(str::to_string);
    let tone = tone
        .map(str::trim)
        .filter(|tone| !tone.is_empty())
        .map(str::to_string);
    let mut flags = 0;
    if to_lang.is_some() {
        flags |= MessagesSummarizeTextRequest::TO_LANG_FLAG;
    }
    if tone.is_some() {
        flags |= MessagesSummarizeTextRequest::TONE_FLAG;
    }
    Ok(MessagesSummarizeTextRequest {
        flags,
        peer: Box::new(input_peer_from_cached(peer)),
        id: message_id,
        to_lang,
        tone,
    })
}

/// Android parity with `TranslateController.isSummarizable`: the server must
/// have marked the message summarizable, it must not be ours/restricted/
/// sponsored and it must carry more than [`MIN_SUMMARY_CHARS`] characters.
pub fn is_summarizable(
    summary_from_language: Option<&str>,
    text: Option<&str>,
    outgoing: bool,
    restricted: bool,
    sponsored: bool,
) -> bool {
    if outgoing || restricted || sponsored {
        return false;
    }
    if summary_from_language.is_none_or(|lang| lang.trim().is_empty()) {
        return false;
    }
    text.is_some_and(|text| text.chars().count() > MIN_SUMMARY_CHARS)
}

pub fn compose_request(
    text: &TextToolTextDto,
    proofread: bool,
    emojify: bool,
    to_lang: Option<&str>,
    tone: Option<&str>,
) -> Result<MessagesComposeMessageWithAiRequest, MtprotoError> {
    if text.text.trim().is_empty() {
        return Err(MtprotoError::Message("AI text required".into()));
    }
    let to_lang = to_lang
        .map(str::trim)
        .filter(|lang| !lang.is_empty())
        .map(str::to_string);
    let tone = tone.map(str::trim).filter(|tone| !tone.is_empty());
    if !proofread && !emojify && to_lang.is_none() && tone.is_none() {
        return Err(MtprotoError::Message("AI transformation required".into()));
    }
    let mut flags = 0;
    if proofread {
        flags |= MessagesComposeMessageWithAiRequest::PROOFREAD_FLAG;
    }
    if emojify {
        flags |= MessagesComposeMessageWithAiRequest::EMOJIFY_FLAG;
    }
    if to_lang.is_some() {
        flags |= MessagesComposeMessageWithAiRequest::TRANSLATE_TO_LANG_FLAG;
    }
    if tone.is_some() {
        flags |= MessagesComposeMessageWithAiRequest::TONE_FLAG;
    }
    Ok(MessagesComposeMessageWithAiRequest {
        flags,
        proofread: proofread.then(true_flag),
        emojify: emojify.then(true_flag),
        text: text_to_tl(text)?,
        translate_to_lang: to_lang,
        tone: tone.map(input_tone).transpose()?,
    })
}

pub fn toggle_peer_translations_request(
    peer: &CachedPeer,
    disabled: bool,
) -> MessagesTogglePeerTranslationsRequest {
    MessagesTogglePeerTranslationsRequest {
        flags: if disabled {
            MessagesTogglePeerTranslationsRequest::DISABLED_FLAG
        } else {
            0
        },
        disabled: disabled.then(true_flag),
        peer: Box::new(input_peer_from_cached(peer)),
    }
}

pub fn create_tone_request(
    title: &str,
    prompt: &str,
    emoji_id: i64,
    display_author: bool,
) -> Result<AicomposeCreateToneRequest, MtprotoError> {
    if title.trim().is_empty() {
        return Err(MtprotoError::Message("AI tone title required".into()));
    }
    if prompt.trim().is_empty() {
        return Err(MtprotoError::Message("AI tone prompt required".into()));
    }
    Ok(AicomposeCreateToneRequest {
        flags: if display_author {
            AicomposeCreateToneRequest::DISPLAY_AUTHOR_FLAG
        } else {
            0
        },
        display_author: display_author.then(true_flag),
        emoji_id,
        title: title.to_string(),
        prompt: prompt.to_string(),
    })
}

pub fn update_tone_request(
    key: &str,
    title: &str,
    prompt: &str,
    emoji_id: i64,
    display_author: bool,
) -> Result<AicomposeUpdateToneRequest, MtprotoError> {
    if title.trim().is_empty() {
        return Err(MtprotoError::Message("AI tone title required".into()));
    }
    if prompt.trim().is_empty() {
        return Err(MtprotoError::Message("AI tone prompt required".into()));
    }
    Ok(AicomposeUpdateToneRequest {
        flags: AicomposeUpdateToneRequest::DISPLAY_AUTHOR_FLAG
            | AicomposeUpdateToneRequest::EMOJI_ID_FLAG
            | AicomposeUpdateToneRequest::TITLE_FLAG
            | AicomposeUpdateToneRequest::PROMPT_FLAG,
        tone: input_tone(key)?,
        display_author: Some(tl_bool(display_author)),
        emoji_id: Some(emoji_id),
        title: Some(title.to_string()),
        prompt: Some(prompt.to_string()),
    })
}

pub fn save_tone_request(
    key: &str,
    unsave: bool,
) -> Result<AicomposeSaveToneRequest, MtprotoError> {
    Ok(AicomposeSaveToneRequest {
        tone: input_tone(key)?,
        unsave: tl_bool(unsave),
    })
}

pub fn delete_tone_request(key: &str) -> Result<AicomposeDeleteToneRequest, MtprotoError> {
    Ok(AicomposeDeleteToneRequest {
        tone: input_tone(key)?,
    })
}

pub fn get_tone_request(key: &str) -> Result<AicomposeGetToneRequest, MtprotoError> {
    Ok(AicomposeGetToneRequest {
        tone: input_tone(key)?,
    })
}

pub fn get_tone_example_request(
    key: &str,
    num: i32,
) -> Result<AicomposeGetToneExampleRequest, MtprotoError> {
    Ok(AicomposeGetToneExampleRequest {
        tone: input_tone(key)?,
        num,
    })
}

// ----------------------------------------------------------- response mapping

pub fn translated_texts(response: &MessagesTranslatedText) -> Vec<TextToolTextDto> {
    let MessagesTranslatedText::MessagesTranslateResult(result) = response;
    vector_boxed_items(&result.result)
        .map(text_from_tl)
        .collect()
}

pub fn composed_text(response: &MessagesComposedMessageWithAi) -> AiTextResultDto {
    let MessagesComposedMessageWithAi::MessagesComposedMessageWithAi(result) = response;
    AiTextResultDto {
        result: text_from_tl(&result.result_text),
        diff: result.diff_text.as_deref().map(text_from_tl),
    }
}

pub fn tones_from_tl(response: &AicomposeTones) -> Vec<AiTextToneDto> {
    match response {
        AicomposeTones::AicomposeTones(tones) => {
            vector_boxed_items(&tones.tones).map(tone_from_tl).collect()
        }
        AicomposeTones::AicomposeTonesNotModified(_) => Vec::new(),
    }
}

pub fn tone_example_from_tl(example: &AiComposeToneExample) -> TextToolTextDto {
    let AiComposeToneExample::AiComposeToneExample(example) = example;
    text_from_tl(&example.to)
}

fn json_string(value: &JsonValue) -> Option<&str> {
    match value {
        JsonValue::JsonString(value) => Some(value.value.as_str()),
        _ => None,
    }
}

/// Reads `translations_manual_enabled` / `translations_auto_enabled` from the
/// `help.getAppConfig` object.
pub fn config_from_app_config(config: &HelpAppConfig) -> TextToolsConfig {
    let mut result = TextToolsConfig::default();
    let HelpAppConfig::HelpAppConfig(config) = config else {
        return result;
    };
    let JsonValue::JsonObject(object) = config.config.as_ref() else {
        return result;
    };
    let Vector::Vector(entries) = object.value.as_ref();
    for entry in &entries.field_1 {
        let JsonObjectValue::JsonObjectValue(entry) = entry.as_ref();
        let Some(raw) = json_string(&entry.value) else {
            continue;
        };
        match entry.key.as_str() {
            "translations_manual_enabled" => {
                result.translations_manual_enabled = TranslationMode::parse(Some(raw));
            }
            "translations_auto_enabled" => {
                result.translations_auto_enabled = TranslationMode::parse(Some(raw));
            }
            _ => {}
        }
    }
    result
}

// ------------------------------------------------------------------ RPC calls

pub fn translate_message_texts(
    handle: u64,
    chat_id: i64,
    message_ids: &[i32],
    to_lang: &str,
) -> Result<Vec<TextToolTextDto>, MtprotoError> {
    with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            with_peer_refresh(state, chat_id, |state| {
                let peer = require_usable_peer(&state.peers, chat_id)?.clone();
                let request = translate_messages_request(&peer, message_ids, to_lang)?;
                let response: MessagesTranslatedText =
                    invoke_api(&mut state.snapshot, state.api_id, request)?;
                Ok(translated_texts(&response))
            })
        })
    })
}

pub fn translate_texts(
    handle: u64,
    texts: &[TextToolTextDto],
    to_lang: &str,
) -> Result<Vec<TextToolTextDto>, MtprotoError> {
    let request = translate_texts_request(texts, to_lang)?;
    with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            let response: MessagesTranslatedText =
                invoke_api(&mut state.snapshot, state.api_id, request.clone())?;
            Ok(translated_texts(&response))
        })
    })
}

pub fn summarize_message_text(
    handle: u64,
    chat_id: i64,
    message_id: i32,
    to_lang: Option<&str>,
    tone: Option<&str>,
) -> Result<TextToolTextDto, MtprotoError> {
    with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            with_peer_refresh(state, chat_id, |state| {
                let peer = require_usable_peer(&state.peers, chat_id)?.clone();
                let request = summarize_request(&peer, message_id, to_lang, tone)?;
                let response: TextWithEntities =
                    invoke_api(&mut state.snapshot, state.api_id, request)?;
                Ok(text_from_tl(&response))
            })
        })
    })
}

pub fn compose_text_with_ai(
    handle: u64,
    text: &TextToolTextDto,
    proofread: bool,
    emojify: bool,
    to_lang: Option<&str>,
    tone: Option<&str>,
) -> Result<AiTextResultDto, MtprotoError> {
    let request = compose_request(text, proofread, emojify, to_lang, tone)?;
    with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            let response: MessagesComposedMessageWithAi =
                invoke_api(&mut state.snapshot, state.api_id, request.clone())?;
            Ok(composed_text(&response))
        })
    })
}

pub fn get_ai_text_tones(handle: u64) -> Result<Vec<AiTextToneDto>, MtprotoError> {
    with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            let response: AicomposeTones = invoke_api(
                &mut state.snapshot,
                state.api_id,
                AicomposeGetTonesRequest { hash: 0 },
            )?;
            Ok(tones_from_tl(&response))
        })
    })
}

pub fn toggle_peer_text_translations(
    handle: u64,
    chat_id: i64,
    disabled: bool,
) -> Result<(), MtprotoError> {
    with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            with_peer_refresh(state, chat_id, |state| {
                let peer = require_usable_peer(&state.peers, chat_id)?.clone();
                let request = toggle_peer_translations_request(&peer, disabled);
                let _: Bool = invoke_api(&mut state.snapshot, state.api_id, request)?;
                Ok(())
            })
        })
    })
}

pub fn create_ai_text_tone(
    handle: u64,
    title: &str,
    prompt: &str,
    emoji_id: i64,
    display_author: bool,
) -> Result<AiTextToneDto, MtprotoError> {
    let request = create_tone_request(title, prompt, emoji_id, display_author)?;
    with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            let response: AiComposeTone =
                invoke_api(&mut state.snapshot, state.api_id, request.clone())?;
            Ok(tone_from_tl(&response))
        })
    })
}

pub fn update_ai_text_tone(
    handle: u64,
    key: &str,
    title: &str,
    prompt: &str,
    emoji_id: i64,
    display_author: bool,
) -> Result<AiTextToneDto, MtprotoError> {
    let request = update_tone_request(key, title, prompt, emoji_id, display_author)?;
    with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            let response: AiComposeTone =
                invoke_api(&mut state.snapshot, state.api_id, request.clone())?;
            Ok(tone_from_tl(&response))
        })
    })
}

pub fn save_ai_text_tone(handle: u64, key: &str, unsave: bool) -> Result<(), MtprotoError> {
    let request = save_tone_request(key, unsave)?;
    with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            let _: Bool = invoke_api(&mut state.snapshot, state.api_id, request.clone())?;
            Ok(())
        })
    })
}

pub fn delete_ai_text_tone(handle: u64, key: &str) -> Result<(), MtprotoError> {
    let request = delete_tone_request(key)?;
    with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            let _: Bool = invoke_api(&mut state.snapshot, state.api_id, request.clone())?;
            Ok(())
        })
    })
}

pub fn get_ai_text_tone(handle: u64, key: &str) -> Result<AiTextToneDto, MtprotoError> {
    let request = get_tone_request(key)?;
    with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            let response: AicomposeTones =
                invoke_api(&mut state.snapshot, state.api_id, request.clone())?;
            tones_from_tl(&response)
                .into_iter()
                .next()
                .ok_or_else(|| MtprotoError::Message("AI tone not found".into()))
        })
    })
}

pub fn get_ai_text_tone_example(handle: u64, key: &str) -> Result<TextToolTextDto, MtprotoError> {
    let request = get_tone_example_request(key, 0)?;
    with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            let response: AiComposeToneExample =
                invoke_api(&mut state.snapshot, state.api_id, request.clone())?;
            Ok(tone_example_from_tl(&response))
        })
    })
}

/// `translations_manual_enabled` / `translations_auto_enabled` from
/// `help.getAppConfig`. Failures fall back to the permissive default.
pub fn get_text_tools_config(handle: u64) -> Result<TextToolsConfigDto, MtprotoError> {
    with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            let config: HelpAppConfig = invoke_api(
                &mut state.snapshot,
                state.api_id,
                HelpGetAppConfigRequest { hash: 0 },
            )?;
            Ok(config_from_app_config(&config).dto())
        })
    })
}

/// `translations_disabled` from the peer full info and `channel.autotranslation`
/// for channels. Network failures degrade to "not restricted".
fn fetch_peer_translation_flags(state: &mut ClientState, peer: &CachedPeer) -> (bool, bool) {
    match peer.kind {
        PeerKind::User => {
            let request = UsersGetFullUserRequest {
                id: Box::new(InputUser::InputUser(InputUserConstructor {
                    user_id: peer.id,
                    access_hash: peer.access_hash,
                })),
            };
            let Ok(response) =
                invoke_api::<_, UsersUserFull>(&mut state.snapshot, state.api_id, request)
            else {
                return (false, false);
            };
            let UsersUserFull::UsersUserFull(full) = response;
            let UserFull::UserFull(user) = *full.full_user;
            (user.translations_disabled.is_some(), false)
        }
        PeerKind::Chat => {
            let request = MessagesGetFullChatRequest { chat_id: peer.id };
            let Ok(response) =
                invoke_api::<_, MessagesChatFull>(&mut state.snapshot, state.api_id, request)
            else {
                return (false, false);
            };
            let MessagesChatFull::MessagesChatFull(full) = response;
            match full.full_chat.as_ref() {
                ChatFull::ChatFull(chat) => (chat.translations_disabled.is_some(), false),
                ChatFull::ChannelFull(channel) => (channel.translations_disabled.is_some(), false),
                _ => (false, false),
            }
        }
        PeerKind::Channel => {
            let request = ChannelsGetFullChannelRequest {
                channel: Box::new(InputChannel::InputChannel(InputChannelConstructor {
                    channel_id: peer.id,
                    access_hash: peer.access_hash,
                })),
            };
            let Ok(response) =
                invoke_api::<_, MessagesChatFull>(&mut state.snapshot, state.api_id, request)
            else {
                return (false, false);
            };
            let MessagesChatFull::MessagesChatFull(full) = response;
            let disabled = match full.full_chat.as_ref() {
                ChatFull::ChannelFull(channel) => channel.translations_disabled.is_some(),
                ChatFull::ChatFull(chat) => chat.translations_disabled.is_some(),
                _ => false,
            };
            let auto = vector_boxed_items(&full.chats).any(|chat| {
                matches!(chat, Chat::Channel(channel) if channel.id == peer.id && channel.autotranslation.is_some())
            });
            (disabled, auto)
        }
    }
}

/// Manual/auto translation availability plus the per-chat restrictions the UI
/// needs to hide translate/summary affordances.
pub fn get_text_tools_availability(
    handle: u64,
    chat_id: i64,
) -> Result<TextToolsAvailabilityDto, MtprotoError> {
    let config = with_client_mut(handle, |state| {
        ensure_ready(state)?;
        call_with_migrate(state, |state| {
            let config: HelpAppConfig = invoke_api(
                &mut state.snapshot,
                state.api_id,
                HelpGetAppConfigRequest { hash: 0 },
            )?;
            Ok(config_from_app_config(&config))
        })
    })?;
    let (peer_translations_disabled, channel_auto_translation) =
        with_client_mut(handle, |state| {
            ensure_ready(state)?;
            let peer = match require_usable_peer(&state.peers, chat_id) {
                Ok(peer) => peer.clone(),
                Err(_) => return Ok((false, false)),
            };
            call_with_migrate(state, |state| {
                Ok(fetch_peer_translation_flags(state, &peer))
            })
        })?;
    Ok(TextToolsAvailabilityDto {
        manual_translation: config.manual_enabled(),
        chat_translation: config.auto_enabled() || config.manual_enabled(),
        peer_translations_disabled,
        channel_auto_translation,
    })
}

#[cfg(test)]
#[path = "../tests/unit/text_tools_tests.rs"]
mod tests;
