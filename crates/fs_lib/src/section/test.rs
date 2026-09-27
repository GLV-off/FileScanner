use super::*;

#[test]
fn builder_create_valid_section() {
    let s = Section::builder()
        .name("asdas".into())
        .start(12)
        .stop(24)
        .size(12)
        .build();
    assert!(s.is_ok());

    let s = s.unwrap();
    assert_eq!(s.description, "asdas");
    assert_eq!(s.dimensions.start(), 12);
    assert_eq!(s.dimensions.stop(), 24);
    assert_eq!(s.dimensions.size(), 12);
}

#[test]
fn builder_rejects_missing_fields() {
    let err = Section::builder()
        .start(12)
        .stop(24)
        .size(12)
        .build()
        .err()
        .unwrap();
    assert_eq!(err, "name not ready");

    let err = Section::builder()
        .name("asdas".into())
        .stop(24)
        .size(12)
        .build()
        .err()
        .unwrap();
    assert_eq!(err, "start not ready");

    let err = Section::builder()
        .name("asdas".into())
        .start(12)
        .size(12)
        .build()
        .err()
        .unwrap();
    assert_eq!(err, "stop not set");

    let err = Section::builder()
        .name("asdas".into())
        .start(12)
        .stop(24)
        .build()
        .err()
        .unwrap();
    assert_eq!(err, "size not set");
}

#[test]
fn builder_rejects_start_greater_than_stop() {
    let err = Section::builder()
        .name("asdas".into())
        .start(24)
        .stop(12)
        .size(0)
        .build()
        .err()
        .unwrap();
    assert_eq!(err, "start (24) must not exceed stop (12)");
}

#[test]
fn builder_rejects_size_mismatch() {
    let err = Section::builder()
        .name("asdas".into())
        .start(12)
        .stop(24)
        .size(36)
        .build()
        .err()
        .unwrap();
    assert_eq!(err, "size mismatch: stop (24) - start (12) = 12, but size is 36");
}

#[test]
fn builder_rejects_out_of_range_size() {
    let err = Section::builder()
        .name("asdas".into())
        .start(i32::MIN)
        .stop(i32::MAX)
        .size(0)
        .build()
        .err()
        .unwrap();
    assert_eq!(
        err,
        "size mismatch: stop (2147483647) - start (-2147483648) = 4294967295, but size is 0"
    );
}
