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

#[cfg(test)]
mod verify_tests {
  use super::*;
  use crate::attachment::Attachment;
  use crate::embed::Embed;
  use crate::embed::image_url::ImageUrl;
  use crate::{Author, Color, Field, Footer};
  use bytes::Bytes;
  use serde_json::Value;
  use time::OffsetDateTime;
  use url::Url;

  fn tiny_attachment(name: &str) -> Attachment {
    Attachment::new(name, Bytes::from_static(b"x")).unwrap()
  }

  #[test]
  fn no_sendable_body_message_should_return_error() {
    let error = Message::new()
      .username("username")
      .unwrap()
      .avatar_url(Url::parse("https://example.com/avatar.png").unwrap())
      .unwrap()
      .tts(true)
      .build()
      .err()
      .expect("failed to return error when building a message with no sendable body");

    assert!(matches!(error, DiscordError::EmptyContent));
  }

  #[test]
  fn total_embed_text_should_not_exceed_6000_characters() {
    let embed_a = Embed::new()
      .title(&"a".repeat(100))
      .unwrap()
      .description(&"a".repeat(4000))
      .unwrap()
      .add_field(Field::new(&"a".repeat(100), &"a".repeat(500)).unwrap())
      .unwrap()
      .add_field(Field::new(&"a".repeat(100), &"a".repeat(500)).unwrap())
      .unwrap();
    let embed_b = Embed::new()
      .author(
        Author::new(&"b".repeat(50))
          .unwrap()
          .url(Url::parse("https://example.com").unwrap())
          .unwrap()
          .icon_url(ImageUrl::attachment("author_icon.png")),
      )
      .footer(
        Footer::new(&"b".repeat(1000))
          .unwrap()
          .icon_url(ImageUrl::attachment("footer-icon.png")),
      )
      .color(Color::RED)
      .image(ImageUrl::url(
        Url::parse("https://example.com/image.png").unwrap(),
      ))
      .thumbnail(ImageUrl::url(
        Url::parse("https://example.com/thumbnail.png").unwrap(),
      ))
      .timestamp(OffsetDateTime::UNIX_EPOCH);
    let error = Message::new()
      .add_embed(embed_a)
      .unwrap()
      .add_embed(embed_b)
      .unwrap()
      .build()
      .err()
      .expect(
        "failed to return error when building a message with total embed text over 6000 characters",
      );
    assert!(matches!(
      error,
      DiscordError::TooLongContent {
        actual,
        max: 6000
      } if actual == 6350
    ));
  }

  #[test]
  fn attachment_filenames_should_be_unique() {
    let error = Message::new()
      .content("ok")
      .unwrap()
      .add_attachment(tiny_attachment("dup.png"))
      .add_attachment(tiny_attachment("dup.png"))
      .build()
      .err()
      .expect("failed to return error when building a message with duplicate attachment filenames");
    assert!(matches!(
      error,
      DiscordError::DuplicateAttachmentFilename(_)
    ));
  }

  #[test]
  fn attachment_imageurl_without_matching_attachment_filename_should_return_error() {
    let embed = Embed::new().image(ImageUrl::attachment("missing.png"));
    let error = Message::new()
      .add_embed(embed)
      .unwrap()
      .add_attachment(tiny_attachment("image.png"))
      .build()
      .err()
      .expect("failed to return error when building a message with attachment image url without matching attachment filename");
    assert!(matches!(error, DiscordError::AttachmentUrlNotFound(_)));
  }

  #[test]
  fn attachment_imageurl_extension_should_be_image_extensions() {
    let error = Message::new()
      .add_embed(
        Embed::new()
          .image(ImageUrl::attachment("notes.txt")),
      )
      .unwrap()
      .add_attachment(tiny_attachment("notes.txt"))
      .build()
      .err()
      .expect("failed to return error when building a message with attachment imageurl extension(not image extensions)");
    assert!(matches!(error, DiscordError::InvalidExtension(_)));
  }

  #[test]
  fn builds_rich_message_like_full_example() {
    let author_icon = Bytes::from_static(b"author-icon");
    let image = Bytes::from_static(b"image-bytes");
    let thumbnail = image.clone();

    let message = Message::new()
      .content("Hello, world!")
      .unwrap()
      .username("username")
      .unwrap()
      .avatar_url(Url::parse("https://example.com/avatar.png").unwrap())
      .unwrap()
      .tts(true)
      .add_embed(
        Embed::new()
          .title("Hello, world!")
          .unwrap()
          .description("This is a test embed.")
          .unwrap()
          .author(
            Author::new("John Doe")
              .unwrap()
              .url(Url::parse("https://example.com").unwrap())
              .unwrap()
              .icon_url(ImageUrl::attachment("author_icon.png")),
          )
          .color(Color::RED)
          .add_field(Field::new("Field 1", "Value 1").unwrap().inline(true))
          .unwrap()
          .add_field(Field::new("Field 2", "Value 2").unwrap().inline(true))
          .unwrap()
          .add_field(Field::new("Field 3", "Value 3").unwrap().inline(false))
          .unwrap()
          .footer(Footer::new("footer").unwrap().icon_url(ImageUrl::url(
            Url::parse("https://example.com/footer-icon.png").unwrap(),
          )))
          .timestamp(OffsetDateTime::UNIX_EPOCH)
          .image(ImageUrl::attachment("image.png"))
          .thumbnail(ImageUrl::attachment("thumbnail.png")),
      )
      .unwrap()
      .add_attachment(Attachment::new("author_icon.png", author_icon).unwrap())
      .add_attachment(Attachment::new("image.png", image).unwrap())
      .add_attachment(Attachment::new("thumbnail.png", thumbnail).unwrap())
      .add_attachment(Attachment::new("file.txt", Bytes::from_static(b"Hello, world!")).unwrap())
      .build()
      .unwrap();

    assert_eq!(message.get_attachments_count(), 4);

    let payload = message.get_payload_json().unwrap();
    let json: Value = serde_json::from_str(payload).unwrap();

    assert_eq!(json["content"], "Hello, world!");
    assert_eq!(json["username"], "username");
    assert_eq!(json["tts"], true);
    assert_eq!(json["embeds"].as_array().unwrap().len(), 1);

    let embed = &json["embeds"][0];
    assert_eq!(embed["title"], "Hello, world!");
    assert_eq!(embed["description"], "This is a test embed.");
    assert_eq!(embed["color"], 16711680);
    assert_eq!(embed["fields"].as_array().unwrap().len(), 3);
    assert_eq!(embed["fields"][0]["name"], "Field 1");
    assert_eq!(embed["fields"][0]["value"], "Value 1");
    assert_eq!(embed["fields"][0]["inline"], true);
    assert_eq!(embed["fields"][1]["name"], "Field 2");
    assert_eq!(embed["fields"][1]["value"], "Value 2");
    assert_eq!(embed["fields"][1]["inline"], true);
    assert_eq!(embed["fields"][2]["name"], "Field 3");
    assert_eq!(embed["fields"][2]["value"], "Value 3");
    assert_eq!(embed["fields"][2]["inline"], false);
    assert_eq!(embed["footer"]["text"], "footer");
    assert_eq!(
      embed["footer"]["icon_url"],
      "https://example.com/footer-icon.png"
    );
    assert_eq!(embed["timestamp"], "1970-01-01T00:00:00Z");
    assert_eq!(embed["image"]["url"], "attachment://image.png");
    assert_eq!(embed["thumbnail"]["url"], "attachment://thumbnail.png");
    assert_eq!(embed["author"]["icon_url"], "attachment://author_icon.png");

    let attachments = json["attachments"].as_array().unwrap();
    assert_eq!(attachments.len(), 4);
    let filenames: Vec<&str> = attachments
      .iter()
      .map(|item| item["filename"].as_str().unwrap())
      .collect();
    assert!(filenames.contains(&"image.png"));
    assert!(filenames.contains(&"file.txt"));
  }
}
