//! Footer
use serde::Serialize;

use crate::embed::image_url::ImageUrl;
use crate::error::DiscordError;

/// Footer
/// Embedの一番下に表示
#[derive(Serialize)]
pub struct Footer {
  /// テキスト
  text: String,
  /// アイコンURL
  #[serde(skip_serializing_if = "Option::is_none")]
  icon_url: Option<ImageUrl>,
}

impl Footer {
  /// Create a new footer struct
  ///
  /// # Errors
  /// Returns an error if:
  /// - `text` is longer than 2048 characters.
  pub fn new(text: &str) -> Result<Self, DiscordError> {
    // verify
    if text.len() > 2048 {
      return Err(DiscordError::TooLongContent {
        actual: text.len(),
        max: 2048,
      });
    }

    Ok(Self {
      text: text.to_string(),
      icon_url: None,
    })
  }

  /// set icon_url
  pub fn icon_url(mut self, icon_url: ImageUrl) -> Self {
    self.icon_url = Some(icon_url);
    self
  }

  /// get character count
  pub(super) fn get_character_count(&self) -> usize {
    self.text.len()
  }

  /// get icon url
  pub(super) fn get_icon_url(&self) -> Option<&ImageUrl> {
    self.icon_url.as_ref()
  }
}
