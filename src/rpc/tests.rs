use serde::{Deserialize, Serialize};

use super::{decode_message, encode_message};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestMessage {
    testing: bool,
}
#[test]
fn test_encode() {
    let expected = "Content-Length: 16\r\n\r\n{\"testing\":true}";
    let actual = encode_message(&TestMessage { testing: true });
    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let expected = TestMessage { testing: true };

    let actual: TestMessage = decode_message(b"Content-Length: 16\r\n\r\n{\"testing\":true}").unwrap();

    assert_eq!(expected, actual)
}
