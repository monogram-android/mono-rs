use super::*;

#[test]
fn handles_remain_unique_after_removal() {
    let registry = ClientRegistry::default();
    let first = registry.next_handle();
    registry.lock().insert(first, Arc::new(7));
    assert_eq!(*registry.lock().remove(&first).unwrap(), 7);
    let second = registry.next_handle();
    assert_ne!(first, second);
    assert!(!registry.lock().contains_key(&first));
}
