use super::*;

#[test]
fn unsupported_draft_is_rejected_before_client_lookup() {
    let draft = CloudDraft {
        chat_id: 1,
        thread_id: 0,
        text: String::new(),
        entities_json: None,
        reply_to_message_id: 0,
        date: 0,
        no_webpage: false,
        has_unsupported_content: true,
    };
    assert!(
        save_cloud_draft(0, draft)
            .unwrap_err()
            .to_string()
            .contains("content")
    );
}
