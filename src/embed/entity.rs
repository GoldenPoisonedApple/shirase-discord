//! Embed
use serde::Serialize;
use time::OffsetDateTime;

use crate::embed::author::Author;
use crate::embed::color::Color;
use crate::embed::field::Field;
use crate::embed::footer::Footer;
use crate::embed::image_url::ImageUrl;
use crate::error::DiscordError;

/// embed media
#[derive(Serialize)]
struct EmbedMedia {
  url: ImageUrl,
}

impl EmbedMedia {
  fn get_url(&self) -> &ImageUrl {
    &self.url
  }
}

/// embed
#[derive(Default, Serialize)]
pub struct Embed {
  /// タイトル
  #[serde(skip_serializing_if = "Option::is_none")]
  title: Option<String>,
  /// 説明
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  /// 作者
  #[serde(skip_serializing_if = "Option::is_none")]
  author: Option<Author>,
  /// 横傍線部色
  #[serde(skip_serializing_if = "Option::is_none")]
  color: Option<Color>,
  /// 部分表示
  #[serde(skip_serializing_if = "Vec::is_empty")]
  fields: Vec<Field>,
  /// フッター
  #[serde(skip_serializing_if = "Option::is_none")]
  footer: Option<Footer>,
  /// タイムスタンプ
  #[serde(
    skip_serializing_if = "Option::is_none",
    with = "time::serde::rfc3339::option"
  )]
  timestamp: Option<OffsetDateTime>,
  /// 画像
  #[serde(skip_serializing_if = "Option::is_none")]
  image: Option<EmbedMedia>,
  /// サムネイル
  #[serde(skip_serializing_if = "Option::is_none")]
  thumbnail: Option<EmbedMedia>,
}

impl Embed {
  /// create a new embed
  ///
  /// # Errors
  /// Returns an error if:
  /// - `title` is longer than 256 characters.
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  /// set title
  ///
  /// # Errors
  /// Returns an error if:
  /// - `title` is longer than 256 characters.
  pub fn title(mut self, title: &str) -> Result<Self, DiscordError> {
    // verify
    if title.len() > 256 {
      return Err(DiscordError::TooLongContent {
        actual: title.len(),
        max: 256,
      });
    }

    self.title = Some(title.to_string());
    Ok(self)
  }

  /// set description
  ///
  /// # Errors
  /// Returns an error if:
  /// - `description` is longer than 4096 characters.
  pub fn description(mut self, description: &str) -> Result<Self, DiscordError> {
    // verify
    if description.len() > 4096 {
      return Err(DiscordError::TooLongContent {
        actual: description.len(),
        max: 4096,
      });
    }

    self.description = Some(description.to_string());
    Ok(self)
  }

  /// set author
  pub fn author(mut self, author: Author) -> Self {
    self.author = Some(author);
    self
  }

  /// set color
  pub fn color(mut self, color: Color) -> Self {
    self.color = Some(color);
    self
  }

  /// add field
  ///
  /// # Errors
  /// Returns an error if:
  /// - `fields` count is more than 25.
  pub fn add_field(mut self, field: Field) -> Result<Self, DiscordError> {
    // verify
    if self.fields.len() >= 25 {
      return Err(DiscordError::TooManyFields {
        actual: self.fields.len(),
        max: 25,
      });
    }

    self.fields.push(field);
    Ok(self)
  }

  /// set footer
  pub fn footer(mut self, footer: Footer) -> Self {
    self.footer = Some(footer);
    self
  }

  /// set timestamp
  pub fn timestamp(mut self, timestamp: OffsetDateTime) -> Self {
    self.timestamp = Some(timestamp);
    self
  }

  /// set image
  pub fn image(mut self, image: ImageUrl) -> Self {
    self.image = Some(EmbedMedia { url: image });
    self
  }

  /// set thumbnail
  pub fn thumbnail(mut self, thumbnail: ImageUrl) -> Self {
    self.thumbnail = Some(EmbedMedia { url: thumbnail });
    self
  }

  /// get character count
  ///
  /// title + description + field.name + field.value + footer.text + author.name
  pub(crate) fn get_character_count(&self) -> usize {
    let mut length = 0;
    if let Some(title) = &self.title {
      length += title.len();
    }
    if let Some(description) = &self.description {
      length += description.len();
    }
    for field in &self.fields {
      length += field.get_character_count();
    }
    if let Some(footer) = &self.footer {
      length += footer.get_character_count();
    }
    if let Some(author) = &self.author {
      length += author.get_character_count();
    }
    length
  }

  /// get image urls
  pub(crate) fn get_image_urls(&self) -> Vec<&ImageUrl> {
    let mut image_urls = Vec::new();
    if let Some(image) = &self.image {
      image_urls.push(image.get_url());
    }
    if let Some(thumbnail) = &self.thumbnail {
      image_urls.push(thumbnail.get_url());
    }
    if let Some(author) = &self.author
      && let Some(icon_url) = author.get_icon_url()
    {
      image_urls.push(icon_url);
    }
    if let Some(footer) = &self.footer
      && let Some(icon_url) = footer.get_icon_url()
    {
      image_urls.push(icon_url);
    }
    image_urls
  }

  /// Verify that this embed contains at least one content element.
  ///
  /// # Errors
  /// Returns an error if:
  /// - embed contains no text content and has neither an `image` nor a `thumbnail`.
  pub(crate) fn verify(&self) -> Result<(), DiscordError> {
    if self.image.is_none() && self.thumbnail.is_none() && self.get_character_count() == 0 {
      return Err(DiscordError::EmptyContent);
    }

    Ok(())
  }
}
