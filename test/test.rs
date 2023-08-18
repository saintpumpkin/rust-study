pub mod week6;
use week6::init;

#[test]
fn test_init() {
    assert!(init().is_ok());
}