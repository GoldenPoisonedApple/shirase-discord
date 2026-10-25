//! Attachment
use crate::error::DiscordError;

use bytes::Bytes;
use serde::Serialize;

/// 添付ファイル
#[derive(Serialize)]
pub struct Attachment {
  /// id
  id: usize,
  /// ファイル名
  filename: String,
  /// for ALT
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  /// ネタばれ防止
  #[serde(skip_serializing_if = "Option::is_none")]
  is_spoiler: Option<bool>,

  /// データ本体
  #[serde(skip)]
  data: Bytes,
}

impl Attachment {
  /// create a new attachment
  ///
  /// `filename` must be unique.(library original restriction)
  ///
  /// # Errors
  ///
  /// Returns an error if:
  /// - `filename` is empty or longer than 1024 characters.
  /// - `data` is empty or larger than 20 MiB.
  pub fn new(filename: &str, data: Bytes) -> Result<Self, DiscordError> {
    // verify
    if filename.is_empty() {
      return Err(DiscordError::EmptyContent);
    }
    if filename.len() > 1024 {
      return Err(DiscordError::TooLongContent {
        actual: filename.len(),
        max: 1024,
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

    Ok(Self {
      id: usize::MIN,
      filename: filename.to_string(),
      description: None,
      is_spoiler: None,
      data,
    })
  }

  /// set id
  pub fn set_id(mut self, id: usize) -> Self {
    self.id = id;
    self
  }

  /// set description
  ///
  /// # Errors
  /// Returns an error if:
  /// - `description` is longer than 1024 characters.
  pub fn description(mut self, description: &str) -> Result<Self, DiscordError> {
    // verify
    if description.len() > 1024 {
      return Err(DiscordError::TooLongContent {
        actual: description.len(),
        max: 1024,
      });
    }

    self.description = Some(description.to_string());
    Ok(self)
  }

  /// set spoiler
  ///
  /// it is ignored in embed.
  pub fn spoiler(mut self, is_spoiler: bool) -> Self {
    self.is_spoiler = Some(is_spoiler);
    self
  }

  /// get filename
  pub(super) fn get_filename(&self) -> &str {
    &self.filename
  }

  /// get data
  pub(super) fn get_data(&self) -> &Bytes {
    &self.data
  }
}
