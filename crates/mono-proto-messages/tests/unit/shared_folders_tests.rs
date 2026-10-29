use super::*;

#[test]
fn rejects_reserved_folder_ids_and_blank_slugs() {
    assert!(chatlist(1).is_err());
    assert!(chatlist(2).is_ok());
    assert!(validate_slug("  ").is_err());
    assert!(validate_slug("invite").is_ok());
}

#[test]
fn empty_peer_selection_is_valid() {
    let peers = HashMap::default();
    let Vector::Vector(selection) = *peer_list(&peers, &[]).unwrap();
    assert_eq!(selection.field_0, 0);
    assert!(selection.field_1.is_empty());
}
