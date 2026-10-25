//! Discord message struct

use serde::Serialize;
use std::collections::HashSet;
use url::Url;

use crate::attachment::Attachment;
use crate::embed::Embed;
use crate::embed::image_url::ImageUrl;
use crate::error::DiscordError;

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
  /// payload
  #[serde(skip)]
  payload_json: Option<String>,
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
  /// # Errors
  /// Returns an error if:
  /// - `embed` count is more than 10.
  pub fn add_embed(mut self, embed: Embed) -> Result<Self, DiscordError> {
    // verify
    if self.embeds.len() >= 10 {
      return Err(DiscordError::TooManyEmbeds {
        actual: self.embeds.len(),
        max: 10,
      });
    }
    embed.verify()?;

    self.embeds.push(embed);
    Ok(self)
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
  fn verify(&self) -> Result<(), DiscordError> {
    // A message must contain at least one of the following:
    // content
    // attachments
    // embed
    if self.content.is_none() && self.attachments.is_empty() && self.embeds.is_empty() {
      return Err(DiscordError::EmptyContent);
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

    // embeds attachment image url must refer to an attachment(library original restriction)
    // and attachment image's filename must be (*.png, *.jpg, *.jpeg, *.gif, *.webp).(discord api restriction)
    for embed in &self.embeds {
      for image_url in embed.get_image_urls() {
        if let ImageUrl::Attachment(filename) = image_url {
          // verify extension
          if !filename.ends_with(".png")
            && !filename.ends_with(".jpg")
            && !filename.ends_with(".jpeg")
            && !filename.ends_with(".gif")
            && !filename.ends_with(".webp")
          {
            return Err(DiscordError::InvalidExtension(
              filename.rsplit('.').next().unwrap().to_string(),
            ));
          }
          // verify attachment filename reference
          if !attachment_filenames.contains(filename.as_str()) {
            return Err(DiscordError::AttachmentUrlNotFound(filename.clone()));
          }
        }
      }
    }

    Ok(())
  }

  /// build the message
  ///
  /// It must be built before sending.
  pub fn build(mut self) -> Result<Self, DiscordError> {
    self.verify()?;

    self.payload_json = Some(serde_json::to_string(&self)?);
    Ok(self)
  }

  /// get attachments count
  pub(super) fn get_attachments_count(&self) -> usize {
    self.attachments_count
  }

  /// get attachments
  pub(super) fn get_attachments(&self) -> &[Attachment] {
    &self.attachments
  }

  /// get payload json
  pub(super) fn get_payload_json(&self) -> Result<&str, DiscordError> {
    if let Some(payload_json) = &self.payload_json {
      Ok(payload_json)
    } else {
      Err(DiscordError::NotBuilt)
    }
  }
}
