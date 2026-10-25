//! Author

use serde::Serialize;
use url::Url;

use crate::embed::image_url::ImageUrl;
use crate::error::DiscordError;

/// Author
/// Displayed at the top of the embed
#[derive(Serialize)]
pub struct Author {
  /// name
  name: String,
  /// url(clickable)
  #[serde(skip_serializing_if = "Option::is_none")]
  url: Option<Url>,
  /// icon url
  #[serde(skip_serializing_if = "Option::is_none")]
  icon_url: Option<ImageUrl>,
}

impl Author {
  /// # Errors
  /// Returns an error if:
  /// - `name` is longer than 256 characters.
  pub fn new(name: &str) -> Result<Self, DiscordError> {
    // verify
    if name.len() > 256 {
      return Err(DiscordError::TooLongContent {
        actual: name.len(),
        max: 256,
      });
    }

    Ok(Self {
      name: name.to_string(),
      url: None,
      icon_url: None,
    })
  }

  /// set url
  ///
  /// # Errors
  /// Returns an error if:
  /// - `url`'s scheme is not http or https.(library original restriction)
  pub fn url(mut self, url: Url) -> Result<Self, DiscordError> {
    if url.scheme() != "http" && url.scheme() != "https" {
      return Err(DiscordError::InvalidUrl(url.to_string()));
    }
    self.url = Some(url);
    Ok(self)
  }

  /// set icon_url
  pub fn icon_url(mut self, icon_url: ImageUrl) -> Self {
    self.icon_url = Some(icon_url);
    self
  }

  /// get character count
  pub(super) fn get_character_count(&self) -> usize {
    self.name.len()
  }

  /// get icon url
  pub(super) fn get_icon_url(&self) -> Option<&ImageUrl> {
    self.icon_url.as_ref()
  }
}
