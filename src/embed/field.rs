//! Field

use serde::Serialize;

use crate::error::DiscordError;

/// 部分表示
#[derive(Serialize)]
pub struct Field {
  /// title
  name: String,
  /// value
  value: String,
  /// inline(place next to each other)
  #[serde(skip_serializing_if = "Option::is_none")]
  inline: Option<bool>,
}

impl Field {
  /// create a new field
  ///
  /// The field is not inline by default.
  ///
  /// # Errors
  /// Returns an error if:
  /// - `name` is longer than 256 characters.
  /// - `value` is longer than 1024 characters.
  pub fn new(name: &str, value: &str) -> Result<Self, DiscordError> {
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
      name: name.to_string(),
      value: value.to_string(),
      inline: None,
    })
  }

  /// set inline
  pub fn inline(mut self, inline: bool) -> Self {
    self.inline = Some(inline);
    self
  }

  /// get character count
  pub(super) fn get_character_count(&self) -> usize {
    self.name.len() + self.value.len()
  }
}
