use super::*;
use mono_proto::codec::{Boxed, Decoder, Limits, TlDecode};
use mono_proto::latest::api::{
    HelpAppConfigConstructor, JsonObjectConstructor, JsonObjectValue, JsonObjectValueConstructor,
    JsonStringConstructor, JsonValue,
};

fn peer(kind: PeerKind, id: i64) -> CachedPeer {
    CachedPeer {
        kind,
        id,
        access_hash: 77,
        min_hash: false,
    }
}

fn decode<Req: Boxed + TlDecode>(request: &Req, expected_id: u32) -> Req {
    let bytes = mono_proto_runtime::rpc::encode_boxed_bytes(request).unwrap();
    let mut decoder = Decoder::new(&bytes, Limits::default()).unwrap();
    assert_eq!(decoder.read_u32().unwrap(), expected_id);
    let decoded = Req::decode(&mut decoder).unwrap();
    decoder.finish().unwrap();
    decoded
}

fn text(value: &str) -> TextToolTextDto {
    TextToolTextDto {
        text: value.into(),
        entities_json: None,
    }
}

// ------------------------------------------------------------------ requests

#[test]
fn translate_messages_request_carries_peer_ids_and_language() {
    let request = translate_messages_request(&peer(PeerKind::Channel, 42), &[3, 9], "de").unwrap();
    assert_eq!(
        request.flags & MessagesTranslateTextRequest::PEER_FLAG,
        MessagesTranslateTextRequest::PEER_FLAG
    );
    assert_eq!(
        request.flags & MessagesTranslateTextRequest::ID_FLAG,
        MessagesTranslateTextRequest::ID_FLAG
    );
    assert!(request.text.is_none());
    let decoded = decode(&request, MessagesTranslateTextRequest::ID);
    assert_eq!(decoded.to_lang, "de");
    let Vector::Vector(ids) = decoded.id.as_deref().unwrap();
    assert_eq!(ids.field_1, vec![3, 9]);
    assert!(decoded.peer.is_some());
    assert!(decoded.tone.is_none());
}

#[test]
fn translate_messages_request_rejects_empty_oversized_and_duplicate_ids() {
    let channel = peer(PeerKind::Channel, 42);
    assert!(translate_messages_request(&channel, &[], "de").is_err());
    let too_many: Vec<i32> = (1..=21).collect();
    assert!(translate_messages_request(&channel, &too_many, "de").is_err());
    assert!(translate_messages_request(&channel, &[0], "de").is_err());
    assert!(translate_messages_request(&channel, &[5, 5], "de").is_err());
    assert!(translate_messages_request(&channel, &[5], "  ").is_err());
    assert!(translate_messages_request(&channel, &(1..=20).collect::<Vec<_>>(), "de").is_ok());
}

#[test]
fn translate_texts_request_sends_rich_texts_without_peer() {
    let request = translate_texts_request(&[text("hallo"), text("welt")], "en").unwrap();
    assert_eq!(
        request.flags & MessagesTranslateTextRequest::TEXT_FLAG,
        MessagesTranslateTextRequest::TEXT_FLAG
    );
    assert!(request.peer.is_none());
    assert!(request.id.is_none());
    let decoded = decode(&request, MessagesTranslateTextRequest::ID);
    let Vector::Vector(texts) = decoded.text.as_deref().unwrap();
    assert_eq!(texts.field_1.len(), 2);
    let TextWithEntities::TextWithEntities(first) = texts.field_1[0].as_ref();
    assert_eq!(first.text, "hallo");
    assert_eq!(decoded.to_lang, "en");
}

#[test]
fn translate_texts_request_enforces_android_batch_limits() {
    assert!(translate_texts_request(&[], "en").is_err());
    let many: Vec<TextToolTextDto> = (0..21).map(|_| text("x")).collect();
    assert!(translate_texts_request(&many, "en").is_err());
    // 25000 UTF-16 units are allowed, one more is not.
    let at_limit = text(&"a".repeat(MAX_TRANSLATION_UTF16_UNITS));
    assert!(translate_texts_request(std::slice::from_ref(&at_limit), "en").is_ok());
    let over_limit = text(&"a".repeat(MAX_TRANSLATION_UTF16_UNITS + 1));
    assert!(translate_texts_request(std::slice::from_ref(&over_limit), "en").is_err());
    assert!(translate_texts_request(&[text("   ")], "en").is_err());
    assert!(translate_texts_request(&[text("hi")], "").is_err());
}

#[test]
fn translate_texts_request_counts_utf16_units_not_chars() {
    // Four astral chars are eight UTF-16 units.
    let astral = text(&"😀".repeat(4));
    assert_eq!(astral.text.encode_utf16().count(), 8);
    assert!(translate_texts_request(&[astral], "en").is_ok());
}

#[test]
fn summarize_request_sets_language_flag_only_when_requested() {
    let plain = summarize_request(&peer(PeerKind::User, 5), 9, None, None).unwrap();
    assert_eq!(plain.flags, 0);
    assert!(plain.to_lang.is_none());
    let translated = summarize_request(&peer(PeerKind::User, 5), 9, Some(" en "), None).unwrap();
    assert_eq!(translated.flags, MessagesSummarizeTextRequest::TO_LANG_FLAG);
    let decoded = decode(&translated, MessagesSummarizeTextRequest::ID);
    assert_eq!(decoded.id, 9);
    assert_eq!(decoded.to_lang.as_deref(), Some("en"));
    assert!(summarize_request(&peer(PeerKind::User, 5), 0, None, None).is_err());
}

#[test]
fn summary_eligibility_matches_translate_controller() {
    let long = "a".repeat(MIN_SUMMARY_CHARS + 1);
    assert!(is_summarizable(
        Some("en"),
        Some(&long),
        false,
        false,
        false
    ));
    // Server must have flagged the message.
    assert!(!is_summarizable(None, Some(&long), false, false, false));
    assert!(!is_summarizable(Some(""), Some(&long), false, false, false));
    // Own, restricted and sponsored messages are never summarizable.
    assert!(!is_summarizable(
        Some("en"),
        Some(&long),
        true,
        false,
        false
    ));
    assert!(!is_summarizable(
        Some("en"),
        Some(&long),
        false,
        true,
        false
    ));
    assert!(!is_summarizable(
        Some("en"),
        Some(&long),
        false,
        false,
        true
    ));
    // Exactly 100 characters is still too short (Android uses `> 100`).
    let exactly = "a".repeat(MIN_SUMMARY_CHARS);
    assert!(!is_summarizable(
        Some("en"),
        Some(&exactly),
        false,
        false,
        false
    ));
    assert!(!is_summarizable(Some("en"), None, false, false, false));
}

#[test]
fn compose_request_sets_each_flag_independently() {
    let plain = compose_request(&text("hi"), true, false, None, None).unwrap();
    assert_eq!(
        plain.flags,
        MessagesComposeMessageWithAiRequest::PROOFREAD_FLAG
    );
    assert!(plain.proofread.is_some());
    assert!(plain.emojify.is_none());
    assert!(plain.translate_to_lang.is_none());
    assert!(plain.tone.is_none());

    let full =
        compose_request(&text("hi"), true, true, Some("de"), Some("default:formal")).unwrap();
    assert_eq!(
        full.flags,
        MessagesComposeMessageWithAiRequest::PROOFREAD_FLAG
            | MessagesComposeMessageWithAiRequest::EMOJIFY_FLAG
            | MessagesComposeMessageWithAiRequest::TRANSLATE_TO_LANG_FLAG
            | MessagesComposeMessageWithAiRequest::TONE_FLAG
    );
    let decoded = decode(&full, MessagesComposeMessageWithAiRequest::ID);
    assert_eq!(decoded.translate_to_lang.as_deref(), Some("de"));
    match decoded.tone.as_deref().unwrap() {
        InputAiComposeTone::InputAiComposeToneDefault(tone) => assert_eq!(tone.tone, "formal"),
        other => panic!("unexpected tone: {other:?}"),
    }
    assert!(compose_request(&text("  "), true, false, None, None).is_err());
    assert!(compose_request(&text("hi"), false, false, None, None).is_err());
}

#[test]
fn tone_keys_map_to_default_and_slug_inputs() {
    match input_tone("default:funny").unwrap().as_ref() {
        InputAiComposeTone::InputAiComposeToneDefault(tone) => assert_eq!(tone.tone, "funny"),
        other => panic!("unexpected tone: {other:?}"),
    }
    match input_tone("my-tone").unwrap().as_ref() {
        InputAiComposeTone::InputAiComposeToneSlug(tone) => assert_eq!(tone.slug, "my-tone"),
        other => panic!("unexpected tone: {other:?}"),
    }
    assert!(input_tone("").is_err());
    assert!(input_tone("default:  ").is_err());
}

#[test]
fn toggle_peer_translations_request_sets_disabled_flag() {
    let enabled = toggle_peer_translations_request(&peer(PeerKind::Chat, 11), false);
    assert_eq!(enabled.flags, 0);
    assert!(enabled.disabled.is_none());
    let disabled = toggle_peer_translations_request(&peer(PeerKind::Chat, 11), true);
    assert_eq!(
        disabled.flags,
        MessagesTogglePeerTranslationsRequest::DISABLED_FLAG
    );
    let decoded = decode(&disabled, MessagesTogglePeerTranslationsRequest::ID);
    assert!(decoded.disabled.is_some());
}

#[test]
fn tone_lifecycle_requests_encode_expected_constructors() {
    let created = create_tone_request("Polite", "be polite", 123, true).unwrap();
    let decoded = decode(&created, AicomposeCreateToneRequest::ID);
    assert_eq!(decoded.title, "Polite");
    assert_eq!(decoded.emoji_id, 123);
    assert!(decoded.display_author.is_some());
    assert!(create_tone_request(" ", "prompt", 0, false).is_err());
    assert!(create_tone_request("Title", " ", 0, false).is_err());

    let updated = update_tone_request("my-tone", "Polite", "be polite", 5, false).unwrap();
    let decoded = decode(&updated, AicomposeUpdateToneRequest::ID);
    assert_eq!(decoded.title.as_deref(), Some("Polite"));
    assert_eq!(decoded.emoji_id, Some(5));
    assert_eq!(decoded.prompt.as_deref(), Some("be polite"));
    assert_eq!(
        decoded.display_author.as_deref(),
        Some(&Bool::BoolFalse(BoolFalseConstructor {}))
    );

    let saved = save_tone_request("default:formal", false).unwrap();
    assert_eq!(
        saved.unsave.as_ref(),
        &Bool::BoolFalse(BoolFalseConstructor {})
    );
    decode(&saved, AicomposeSaveToneRequest::ID);
    let unsaved = save_tone_request("default:formal", true).unwrap();
    assert_eq!(
        unsaved.unsave.as_ref(),
        &Bool::BoolTrue(BoolTrueConstructor {})
    );

    decode(
        &delete_tone_request("my-tone").unwrap(),
        AicomposeDeleteToneRequest::ID,
    );
    decode(
        &get_tone_request("my-tone").unwrap(),
        AicomposeGetToneRequest::ID,
    );
    let example = get_tone_example_request("my-tone", 0).unwrap();
    assert_eq!(example.num, 0);
    decode(&example, AicomposeGetToneExampleRequest::ID);
}

// ---------------------------------------------------------------- conversions

#[test]
fn tone_from_tl_distinguishes_defaults_and_custom_tones() {
    let default = tone_from_tl(&AiComposeTone::AiComposeToneDefault(
        AiComposeToneDefaultConstructor {
            tone: "formal".into(),
            emoji_id: 9,
            title: "Formal".into(),
        },
    ));
    assert_eq!(
        default,
        AiTextToneDto {
            key: "default:formal".into(),
            title: "Formal".into(),
            emoji_id: 9,
            creator: false,
            prompt: None,
        }
    );

    let custom = tone_from_tl(&AiComposeTone::AiComposeTone(AiComposeToneConstructor {
        flags: AiComposeToneConstructor::CREATOR_FLAG,
        creator: Some(Box::new(True::True(TrueConstructor {}))),
        id: 1,
        access_hash: 2,
        slug: "polite".into(),
        title: "Polite".into(),
        emoji_id: Some(4),
        prompt: Some("be polite".into()),
        installs_count: None,
        author_id: None,
        example_english: None,
    }));
    assert_eq!(custom.key, "polite");
    assert!(custom.creator);
    assert_eq!(custom.emoji_id, 4);
    assert_eq!(custom.prompt.as_deref(), Some("be polite"));
}

#[test]
fn translation_mode_parses_server_values() {
    assert_eq!(
        TranslationMode::parse(Some("enabled")),
        TranslationMode::Enabled
    );
    assert_eq!(
        TranslationMode::parse(Some(" alternative ")),
        TranslationMode::Alternative
    );
    assert_eq!(
        TranslationMode::parse(Some("system")),
        TranslationMode::System
    );
    assert_eq!(
        TranslationMode::parse(Some("disabled")),
        TranslationMode::Disabled
    );
    assert_eq!(TranslationMode::parse(None), TranslationMode::Enabled);
    assert_eq!(
        TranslationMode::parse(Some("garbage")),
        TranslationMode::Enabled
    );
    assert!(!TranslationMode::Disabled.supports_telegram_api());
    for mode in ["system", "alternative"] {
        let config = config_from_app_config(&app_config(vec![
            ("translations_manual_enabled", "enabled"),
            ("translations_auto_enabled", mode),
        ]));
        assert!(config.manual_enabled());
        assert!(!config.auto_enabled());
    }
    assert_eq!(TranslationMode::Alternative.as_str(), "alternative");
}

fn app_config(entries: Vec<(&str, &str)>) -> HelpAppConfig {
    let values = entries
        .into_iter()
        .map(|(key, value)| {
            Box::new(JsonObjectValue::JsonObjectValue(
                JsonObjectValueConstructor {
                    key: key.into(),
                    value: Box::new(JsonValue::JsonString(JsonStringConstructor {
                        value: value.into(),
                    })),
                },
            ))
        })
        .collect::<Vec<_>>();
    HelpAppConfig::HelpAppConfig(HelpAppConfigConstructor {
        hash: 1,
        config: Box::new(JsonValue::JsonObject(JsonObjectConstructor {
            value: Box::new(Vector::Vector(VectorConstructor {
                field_0: values.len() as u32,
                field_1: values,
            })),
        })),
    })
}

#[test]
fn config_from_app_config_reads_translation_modes() {
    let config = app_config(vec![
        ("translations_manual_enabled", "alternative"),
        ("translations_auto_enabled", "disabled"),
    ]);
    let parsed = config_from_app_config(&config);
    assert_eq!(
        parsed.translations_manual_enabled,
        TranslationMode::Alternative
    );
    assert_eq!(parsed.translations_auto_enabled, TranslationMode::Disabled);
    assert!(!parsed.manual_enabled());
    assert!(!parsed.auto_enabled());
}

#[test]
fn config_from_app_config_defaults_when_keys_are_missing() {
    let parsed = config_from_app_config(&app_config(vec![("unrelated", "disabled")]));
    assert_eq!(parsed, TextToolsConfig::default());
    assert_eq!(
        parsed.dto(),
        TextToolsConfigDto {
            translations_manual_enabled: "enabled".into(),
            translations_auto_enabled: "enabled".into(),
        }
    );
    assert!(parsed.manual_enabled() && parsed.auto_enabled());
}

#[test]
fn text_round_trip_keeps_rich_entities_including_proofread_diffs() {
    let entities = vec![
        Box::new(MessageEntity::MessageEntityBold(
            MessageEntityBoldConstructor {
                offset: 0,
                length: 2,
            },
        )),
        Box::new(MessageEntity::MessageEntityDiffInsert(
            MessageEntityDiffInsertConstructor {
                offset: 2,
                length: 3,
            },
        )),
        Box::new(MessageEntity::MessageEntityDiffReplace(
            MessageEntityDiffReplaceConstructor {
                offset: 5,
                length: 4,
                old_text: "alt".into(),
            },
        )),
        Box::new(MessageEntity::MessageEntityDiffDelete(
            MessageEntityDiffDeleteConstructor {
                offset: 9,
                length: 1,
            },
        )),
    ];
    let value = TextWithEntities::TextWithEntities(TextWithEntitiesConstructor {
        text: "hello world".into(),
        entities: vector(entities),
    });
    let dto = text_from_tl(&value);
    let json = dto.entities_json.as_deref().unwrap();
    assert!(json.contains("diff_insert"));
    assert!(json.contains("diff_replace"));
    assert!(json.contains("diff_delete"));
    assert!(json.contains("\"text\":\"alt\""));

    let back = text_to_tl(&dto).unwrap();
    let TextWithEntities::TextWithEntities(back) = back.as_ref();
    assert_eq!(back.text, "hello world");
    let Vector::Vector(items) = back.entities.as_ref();
    assert_eq!(items.field_1.len(), 4);
    match items.field_1[2].as_ref() {
        MessageEntity::MessageEntityDiffReplace(entity) => assert_eq!(entity.old_text, "alt"),
        other => panic!("unexpected entity: {other:?}"),
    }
}

#[test]
fn translated_texts_maps_every_result_in_order() {
    let response =
        MessagesTranslatedText::MessagesTranslateResult(MessagesTranslateResultConstructor {
            result: vector(vec![
                Box::new(TextWithEntities::TextWithEntities(
                    TextWithEntitiesConstructor {
                        text: "eins".into(),
                        entities: vector(vec![]),
                    },
                )),
                Box::new(TextWithEntities::TextWithEntities(
                    TextWithEntitiesConstructor {
                        text: "zwei".into(),
                        entities: vector(vec![]),
                    },
                )),
            ]),
        });
    let texts = translated_texts(&response);
    assert_eq!(texts.len(), 2);
    assert_eq!(texts[0].text, "eins");
    assert_eq!(texts[1].text, "zwei");
}

#[test]
fn composed_text_maps_result_and_optional_diff() {
    let with_diff = MessagesComposedMessageWithAi::MessagesComposedMessageWithAi(
        MessagesComposedMessageWithAiConstructor {
            flags: MessagesComposedMessageWithAiConstructor::DIFF_TEXT_FLAG,
            result_text: Box::new(TextWithEntities::TextWithEntities(
                TextWithEntitiesConstructor {
                    text: "fixed".into(),
                    entities: vector(vec![]),
                },
            )),
            diff_text: Some(Box::new(TextWithEntities::TextWithEntities(
                TextWithEntitiesConstructor {
                    text: "fxed".into(),
                    entities: vector(vec![Box::new(MessageEntity::MessageEntityDiffReplace(
                        MessageEntityDiffReplaceConstructor {
                            offset: 1,
                            length: 1,
                            old_text: "i".into(),
                        },
                    ))]),
                },
            ))),
        },
    );
    let composed = composed_text(&with_diff);
    assert_eq!(composed.result.text, "fixed");
    let diff = composed.diff.expect("diff expected");
    assert!(diff.entities_json.unwrap().contains("diff_replace"));

    let without_diff = MessagesComposedMessageWithAi::MessagesComposedMessageWithAi(
        MessagesComposedMessageWithAiConstructor {
            flags: 0,
            result_text: Box::new(TextWithEntities::TextWithEntities(
                TextWithEntitiesConstructor {
                    text: "kept".into(),
                    entities: vector(vec![]),
                },
            )),
            diff_text: None,
        },
    );
    assert!(composed_text(&without_diff).diff.is_none());
}

#[test]
fn tones_from_tl_handles_not_modified() {
    let modified = AicomposeTones::AicomposeTones(AicomposeTonesConstructor {
        hash: 5,
        tones: vector(vec![Box::new(AiComposeTone::AiComposeToneDefault(
            AiComposeToneDefaultConstructor {
                tone: "formal".into(),
                emoji_id: 0,
                title: "Formal".into(),
            },
        ))]),
        users: vector(vec![]),
    });
    let tones = tones_from_tl(&modified);
    assert_eq!(tones.len(), 1);
    assert_eq!(tones[0].key, "default:formal");
    assert!(
        tones_from_tl(&AicomposeTones::AicomposeTonesNotModified(
            AicomposeTonesNotModifiedConstructor {}
        ))
        .is_empty()
    );
}

#[test]
fn tone_example_maps_the_output_side() {
    let example = AiComposeToneExample::AiComposeToneExample(AiComposeToneExampleConstructor {
        from: Box::new(TextWithEntities::TextWithEntities(
            TextWithEntitiesConstructor {
                text: "src".into(),
                entities: vector(vec![]),
            },
        )),
        to: Box::new(TextWithEntities::TextWithEntities(
            TextWithEntitiesConstructor {
                text: "dst".into(),
                entities: vector(vec![]),
            },
        )),
    });
    assert_eq!(tone_example_from_tl(&example).text, "dst");
}
