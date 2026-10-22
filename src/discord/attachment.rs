//! Attachment
use crate::discord::error::DiscordError;

use bytes::Bytes;
use serde_json::{Map, Value};

/// 添付ファイル
pub struct Attachment {
  /// ファイル名
  filename: String,
  /// image/png など
  content_type: &'static str,
  /// 説明
  description: Option<String>,
  /// データ本体
  data: Bytes,
}

impl Attachment {
  /// create a new attachment
  ///
  /// The content type is inferred from `data`.
  ///
  /// # Errors
  ///
  /// Returns an error if:
  /// - `filename` is empty or longer than 256 bytes.
  /// - `data` is empty or larger than 20 MiB.
  /// - `data`'s content type is not an image.
  pub fn new(filename: String, data: Bytes) -> Result<Self, DiscordError> {
    // infer content type
    let content_type = match infer::get(&data) {
      Some(kind) => kind.mime_type(),
      None => "unknown",
    };

    // verify
    if filename.is_empty() {
      return Err(DiscordError::EmptyContent);
    }
    if filename.len() > 256 {
      return Err(DiscordError::TooLongContent {
        actual: filename.len(),
        max: 256,
      });
    }
    if data.is_empty() {
      return Err(DiscordError::EmptyContent);
    }
    if data.len() > 1024 * 1024 * 20 {
      return Err(DiscordError::TooLargeFile {
        actual: data.len(),
        max: 1024 * 1024 * 20,
      });
    }
    // only images can be attached
    match content_type {
      "image/png" | "image/jpeg" | "image/gif" | "image/webp" | "image/svg+xml" | "image/ico" => {
        content_type
      }
      kind => return Err(DiscordError::InvalidContentType(kind)),
    };

    Ok(Self {
      filename,
      content_type,
      description: None,
      data,
    })
  }

  /// set description
  ///
  /// # Errors
  /// Returns an error if:
  /// - `description` is longer than 2048 bytes.
  pub fn description(mut self, description: String) -> Result<Self, DiscordError> {
    // verify
    if description.len() > 2048 {
      return Err(DiscordError::TooLongContent {
        actual: description.len(),
        max: 2048,
      });
    }

    self.description = Some(description);
    Ok(self)
  }

  /// get filename
  pub fn get_filename(&self) -> &str {
    &self.filename
  }

  /// get content type
  pub fn get_content_type(&self) -> &'static str {
    self.content_type
  }

  /// get description
  pub fn get_description(&self) -> Option<&str> {
    self.description.as_deref()
  }

  /// get data
  pub fn get_data(&self) -> &Bytes {
    &self.data
  }

  /// build
  pub fn build(&self, id: u64) -> Result<Value, DiscordError> {
    let mut map = Map::new();
    map.insert("id".to_string(), Value::Number(id.into()));
    if let Some(description) = &self.description {
      map.insert(
        "description".to_string(),
        Value::String(description.clone()),
      );
    }
    map.insert("filename".to_string(), Value::String(self.filename.clone()));

    Ok(Value::Object(map))
  }
}
