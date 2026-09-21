use super::*;

#[test]
fn builder_create_valid_section() {
    let s = Section::builder()
        .name("asdas".into())
        .start(12)
        .stop(24)
        .size(36)
        .build();
    assert!(s.is_ok());
    assert!(!s.is_err());
}