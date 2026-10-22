//! Plain message struct

use serde_json::{Map, Value};

use crate::discord::error::DiscordError;

/// Plain message struct
pub struct Plain {
  content: String,
}
impl Plain {
  /// create a new plain message
  ///
  /// # Errors
  /// Returns an error if:
  /// - `content` is longer than 2000 bytes.
  pub fn new(content: String) -> Result<Self, DiscordError> {
    // verify
    if content.len() > 2000 {
      return Err(DiscordError::TooLongContent {
        actual: content.len(),
        max: 2000,
      });
    }

    Ok(Self { content })
  }

  /// build
  pub fn build(&self) -> Value {
    let mut map = Map::new();
    map.insert("content".to_string(), Value::String(self.content.clone()));
    Value::Object(map)
  }
}
