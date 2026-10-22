//! Field

use serde_json::Value;

use crate::discord::error::DiscordError;

/// 部分表示
pub struct Field {
  /// タイトル
  name: String,
  /// 値
  value: String,
  /// 横に並べるかどうか
  inline: bool,
}

impl Field {
  /// create a new field
  ///
  /// The field is inline by default.
  ///
  /// # Errors
  /// Returns an error if:
  /// - `name` is longer than 256 bytes.
  /// - `value` is longer than 1024 bytes.
  pub fn new(name: String, value: String) -> Result<Self, DiscordError> {
    // verify
    if name.len() > 256 {
      return Err(DiscordError::TooLongContent {
        actual: name.len(),
        max: 256,
      });
    }
    if value.len() > 1024 {
      return Err(DiscordError::TooLongContent {
        actual: value.len(),
        max: 1024,
      });
    }

    Ok(Self {
      name,
      value,
      inline: true,
    })
  }

  /// set inline
  pub fn inline(mut self, inline: bool) -> Self {
    self.inline = inline;
    self
  }

  /// build
  pub fn build(&self) -> Result<Value, DiscordError> {
    let mut obj = serde_json::Map::new();
    obj.insert("name".into(), Value::String(self.name.clone()));
    obj.insert("value".into(), Value::String(self.value.clone()));
    obj.insert("inline".into(), Value::Bool(self.inline));

    Ok(Value::Object(obj))
  }
}
