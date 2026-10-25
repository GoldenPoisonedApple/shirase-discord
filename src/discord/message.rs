//! Discord message struct

use serde::Serialize;
use std::collections::HashSet;
use url::Url;

use crate::discord::attachment::Attachment;
use crate::discord::embed::Embed;
use crate::discord::embed::image_url::ImageUrl;
use crate::discord::error::DiscordError;

/// Discord message struct
#[derive(Default, Serialize)]
pub struct Message {
  /// content
  #[serde(skip_serializing_if = "Option::is_none")]
  content: Option<String>,
  /// discord username(override)
  #[serde(skip_serializing_if = "Option::is_none")]
  username: Option<String>,
  /// discord user icon url(override)
  #[serde(skip_serializing_if = "Option::is_none")]
  avatar_url: Option<Url>,
  /// tts
  #[serde(skip_serializing_if = "Option::is_none")]
  tts: Option<bool>,
  /// embeds
  #[serde(skip_serializing_if = "Vec::is_empty")]
  embeds: Vec<Embed>,
  /// attachments
  #[serde(skip_serializing_if = "Vec::is_empty")]
  attachments: Vec<Attachment>,

  /// attachments count
  #[serde(skip)]
  attachments_count: usize,
}

impl Message {
  /// Create a new base struct
  pub fn new() -> Self {
    Self {
      attachments_count: usize::MIN,
      ..Default::default()
    }
  }

  /// Set the content
  ///
  /// # Errors
  /// Returns an error if:
  /// - `content` is longer than 2000 characters.
  pub fn content(mut self, content: &str) -> Result<Self, DiscordError> {
    // verify
    if content.len() > 2000 {
      return Err(DiscordError::TooLongContent {
        actual: content.len(),
        max: 2000,
      });
    }

    self.content = Some(content.to_string());
    Ok(self)
  }

  /// Set the username
  ///
  /// # Errors
  /// Returns an error if:
  /// - `username` is empty or longer than 80 characters.
  pub fn username(mut self, username: &str) -> Result<Self, DiscordError> {
    // verify
    if username.is_empty() {
      return Err(DiscordError::EmptyContent);
    }
    if username.len() > 80 {
      return Err(DiscordError::TooLongContent {
        actual: username.len(),
        max: 80,
      });
    }
    self.username = Some(username.to_string());
    Ok(self)
  }

  /// Set the avatar_url
  ///
  /// # Errors
  /// Returns an error if:
  /// - `avatar_url`'s scheme is not http or https.(library original restriction)
  pub fn avatar_url(mut self, avatar_url: Url) -> Result<Self, DiscordError> {
    // verify
    if avatar_url.scheme() != "http" && avatar_url.scheme() != "https" {
      return Err(DiscordError::InvalidUrl(avatar_url.to_string()));
    }
    self.avatar_url = Some(avatar_url);
    Ok(self)
  }

  /// Set the tts
  pub fn tts(mut self, tts: bool) -> Self {
    self.tts = Some(tts);
    self
  }

  /// Add an embed
  pub fn add_embed(mut self, embed: Embed) -> Self {
    self.embeds.push(embed);
    self
  }

  /// Add an attachment
  pub fn add_attachment(mut self, attachment: Attachment) -> Self {
    self
      .attachments
      .push(attachment.set_id(self.attachments_count));
    self.attachments_count += 1;
    self
  }

  /// verify
  pub(super) fn verify(&self) -> Result<(), DiscordError> {
    // A message must contain at least one of the following:
    // content
    // attachments
    // embed
    if self.content.is_none() && self.attachments.is_empty() && self.embeds.is_empty() {
      return Err(DiscordError::EmptyContent);
    }

    // verify each embed
    for embed in &self.embeds {
      embed.verify()?;
    }

    // all embeds character count <= 6000
    let mut total_character_count = 0;
    for embed in &self.embeds {
      total_character_count += embed.get_character_count();
    }
    if total_character_count > 6000 {
      return Err(DiscordError::TooLongContent {
        actual: total_character_count,
        max: 6000,
      });
    }

    // attachments filename must be unique.(library original restriction)
    let mut attachment_filenames = HashSet::new();
    for attachment in &self.attachments {
      let filename = attachment.get_filename();
      if !attachment_filenames.insert(filename) {
        return Err(DiscordError::DuplicateAttachmentFilename(
          filename.to_string(),
        ));
      }
    }

    // embeds attachment url must refer to an attachment
    for embed in &self.embeds {
      for image_url in embed.get_image_urls() {
        if let ImageUrl::Attachment(filename) = image_url
          && !attachment_filenames.contains(filename.as_str())
        {
          return Err(DiscordError::AttachmentUrlNotFound(filename.clone()));
        }
      }
    }

    Ok(())
  }

  /// get attachments count
  pub(super) fn get_attachments_count(&self) -> usize {
    self.attachments_count
  }

  /// take attachments
  pub(super) fn take_attachments(self) -> Vec<Attachment> {
    self.attachments
  }
}
