// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
use super::*;
use crate::adapter::config::Config;

#[test]
fn test_config() {
    let config = Config::new();
    assert!(config.is_ok());
}
