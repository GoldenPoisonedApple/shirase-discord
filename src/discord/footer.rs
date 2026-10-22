//! Footer
use serde_json::Value;
use url::Url;

use crate::discord::error::DiscordError;

/// Footer
/// Embedの一番下に表示
pub struct Footer {
  /// テキスト
  text: String,
  /// アイコンURL
  icon_url: Option<Url>,
}

impl Footer {
  /// require: text
  pub fn new(text: String) -> Result<Self, DiscordError> {
    // verify
    if text.len() > 2048 {
      return Err(DiscordError::TooLongContent {
        actual: text.len(),
        max: 2048,
      });
    }

    Ok(Self {
      text,
      icon_url: None,
    })
  }

  /// set icon_url
  pub fn icon_url(mut self, icon_url: Url) -> Self {
    self.icon_url = Some(icon_url);
    self
  }

  /// build
  pub fn build(&self) -> Result<Value, DiscordError> {
    let mut obj = serde_json::Map::new();
    obj.insert("text".into(), Value::String(self.text.clone()));
    if let Some(icon_url) = &self.icon_url {
      obj.insert("icon_url".into(), Value::String(icon_url.to_string()));
    }

    Ok(Value::Object(obj))
  }
}
