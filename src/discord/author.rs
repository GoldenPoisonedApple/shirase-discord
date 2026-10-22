//! Author

use serde_json::Value;
use url::Url;

use crate::discord::error::DiscordError;

/// Author
/// Embedの一番上に表示
pub struct Author {
  /// 作者名
  name: String,
  /// クリックした時のURL
  url: Option<Url>,
  /// アイコンURL
  icon_url: Option<Url>,
}

impl Author {
  /// # Errors
  /// Returns an error if:
  /// - `name` is longer than 256 bytes.
  pub fn new(name: String) -> Result<Self, DiscordError> {
    // verify
    if name.len() > 256 {
      return Err(DiscordError::TooLongContent {
        actual: name.len(),
        max: 256,
      });
    }

    Ok(Self {
      name,
      url: None,
      icon_url: None,
    })
  }

  /// set url
  pub fn url(mut self, url: Url) -> Self {
    self.url = Some(url);
    self
  }

  /// set icon_url
  pub fn icon_url(mut self, icon_url: Url) -> Self {
    self.icon_url = Some(icon_url);
    self
  }

  /// build
  pub fn build(&self) -> Result<Value, DiscordError> {
    let mut obj = serde_json::Map::new();
    obj.insert("name".into(), Value::String(self.name.clone()));
    if let Some(url) = &self.url {
      obj.insert("url".into(), Value::String(url.to_string()));
    }
    if let Some(icon_url) = &self.icon_url {
      obj.insert("icon_url".into(), Value::String(icon_url.to_string()));
    }

    Ok(Value::Object(obj))
  }
}
