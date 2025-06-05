#[cfg(test)]
mod tests {
    use protocol::{MAX_BUFFER_SIZE, MessageProtocol, ProtocolError};

    #[test]
    fn roundtrip_ok() {
        let original = MessageProtocol {
            user_name: "bob".into(),
            body: "こんにちは、世界！🌏".into(),
        };
        let frame = original.serialize().expect("serialise");
        let decoded = MessageProtocol::deserialize(&frame).expect("deserialise");
        assert_eq!(decoded, original);
    }

    #[test]
    fn username_too_long_error() {
        let long_name = "x".repeat(256); // 256 > 255
        let msg = MessageProtocol {
            user_name: long_name,
            body: String::new(),
        };
        let err = msg.serialize().unwrap_err();
        assert!(matches!(err, ProtocolError::UsernameTooLong(256)));
    }

    #[test]
    fn buffer_too_large_error() {
        let msg = MessageProtocol {
            user_name: "u".into(),
            body: "a".repeat(MAX_BUFFER_SIZE), // 1(name_len)+1(username)+4096(body) => 4098
        };
        let err = msg.serialize().unwrap_err();
        assert!(matches!(err, ProtocolError::BufferTooLarge(_)));
    }

    #[test]
    fn truncated_buffer_error() {
        // username length byte says 5 but only 3 bytes of data present
        let frame = vec![5u8, b'a', b'b', b'c'];
        let err = MessageProtocol::deserialize(&frame).unwrap_err();
        assert!(matches!(err, ProtocolError::Truncated { .. }));
    }
}
