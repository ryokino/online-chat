//! Protocol for the UDP transmission.
//!
//! * Max frame size : 4096bytes
//! * Byte 0 : user-name length (`u8`, 0 - 255)
//! * Byte 1 - 1 + user-name length : user-name
//! * Byte user-name length + 1 -: message data

use std::fmt;

pub const MAX_BUFFER_SIZE: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageProtocol {
    pub user_name: String,
    pub body: String,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ProtocolError {
    #[error("username too long: {0} bytes (max 255)")]
    UsernameTooLong(usize),

    #[error("frame exceeds maximum size of {MAX_BUFFER_SIZE} bytes: {0}")]
    BufferTooLarge(usize),

    #[error("frame truncated: expected {expected} bytes, have {actual}")]
    Truncated { expected: usize, actual: usize },

    #[error("invalid UTF‑8 in username: {0}")]
    UsernameUtf8(#[from] std::string::FromUtf8Error),

    #[error("invalid UTF‑8 in body: {0}")]
    BodyUtf8(#[from] std::str::Utf8Error),
}

impl MessageProtocol {
    /// Serialise a [`MessageProtocol`] into a wire‑format byte vector.
    pub fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let name_bytes = self.user_name.as_bytes();
        if name_bytes.len() > u8::MAX as usize {
            return Err(ProtocolError::UsernameTooLong(name_bytes.len()));
        }

        let mut buf = Vec::with_capacity(1 + name_bytes.len() + self.body.len());
        buf.push(name_bytes.len() as u8);
        buf.extend_from_slice(name_bytes);
        buf.extend_from_slice(self.body.as_bytes());

        if buf.len() > MAX_BUFFER_SIZE {
            return Err(ProtocolError::BufferTooLarge(buf.len()));
        }
        Ok(buf)
    }

    /// Deserialise a wire‑format byte vector into a [`MessageProtocol`].
    pub fn deserialize(buf: &[u8]) -> Result<Self, ProtocolError> {
        if buf.len() > MAX_BUFFER_SIZE {
            return Err(ProtocolError::BufferTooLarge(buf.len()));
        }

        if buf.is_empty() {
            return Err(ProtocolError::Truncated {
                expected: 1,
                actual: 0,
            });
        }

        let name_len = buf[0] as usize;
        let expected_min = 1 + name_len;
        if buf.len() < expected_min {
            return Err(ProtocolError::Truncated {
                expected: expected_min,
                actual: buf.len(),
            });
        }

        let username = String::from_utf8(buf[1..1 + name_len].to_vec())?;
        let body_bytes = &buf[1 + name_len..];
        let body = std::str::from_utf8(body_bytes)?.to_owned();
        Ok(MessageProtocol {
            user_name: username,
            body,
        })
    }
}

impl fmt::Display for MessageProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>: {}", self.user_name, self.body)
    }
}
