// tests/integration_test.rs

use cargo_auto_template_new_cli::*;

#[test]
fn integration_test_01() {
    assert_eq!(format_hello_phrase("abcd"), "Hello abcd!");
    assert_eq!(format_upper_hello_phrase("abcd").expect("error"), "Hello ABCD!");
}

#[test]
fn integration_test_02_error_check() {
    assert!(format_upper_hello_phrase("ABCD").is_err());
}
