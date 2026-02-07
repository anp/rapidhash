#[test]
fn test_fast_random_state_debug() {
    use rapidhash::fast::RandomState;
    let state = RandomState::default();
    let debug_str = format!("{:?}", state);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_quality_random_state_debug() {
    use rapidhash::quality::RandomState;
    let state = RandomState::default();
    let debug_str = format!("{:?}", state);
    assert!(!debug_str.is_empty());
}
