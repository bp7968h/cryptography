extern crate cryptography;

use cryptography::SHA1;

#[test]
fn test_sha1_empty_string() {
    let mut sha1 = SHA1::new();
    let result = sha1.hash("");
    assert_eq!(result, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
}

#[test]
fn test_sha1_abc() {
    let mut sha1 = SHA1::new();
    let result = sha1.hash("abc");
    assert_eq!(result, "a9993e364706816aba3e25717850c26c9cd0d89d");
}

#[test]
fn test_sha1_longer_message() {
    let mut sha1 = SHA1::new();
    let result = sha1.hash("The quick brown fox jumps over the lazy dog");
    assert_eq!(result, "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
}

#[test]
fn test_sha1_longer_message_with_period() {
    let mut sha1 = SHA1::new();
    let result = sha1.hash("The quick brown fox jumps over the lazy dog.");
    assert_eq!(result, "408d94384216f890ff7a0c3528e8bed1e0b01621");
}

#[test]
fn test_sha1_repeated_characters() {
    let mut sha1 = SHA1::new();
    let result = sha1.hash("aaaaaaaaaa");
    assert_eq!(result, "3495ff69d34671d1e15b33a63c1379fdedd3a32a");
}

#[test]
fn test_sha1_alphanumeric_string() {
    let mut sha1 = SHA1::new();
    let result = sha1.hash("1234567890abcdefghijklmnopqrstuvwxyz");
    assert_eq!(result, "5471d5e4e91d0c0d87249d5873d7fcb5a141a582");
}

#[test]
fn test_sha1_non_ascii_characters() {
    let mut sha1 = SHA1::new();
    let result = sha1.hash("こんにちは世界");
    assert_eq!(result, "a4d9dd44b0951a008fa84865df14d5b6c6f7ecdb");
}