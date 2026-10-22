//! Embed
use serde_json::{Map, Value};
use time::OffsetDateTime;
use url::Url;

use crate::discord::attachment::Attachment;
use crate::discord::author::Author;
use crate::discord::color::Color;
use crate::discord::error::DiscordError;
use crate::discord::field::Field;
use crate::discord::footer::Footer;

/// リッチメッセージ
#[derive(Default)]
pub struct Embed {
  /// 送信者名(上書き)
  sender_name: Option<String>,
  /// 送信者アイコンURL(上書き)
  sender_icon_url: Option<Url>,
  /// 読み上げるか
  tts: Option<bool>,

  // Embed
  /// タイトル
  title: String,
  /// 説明
  description: Option<String>,
  /// 作者
  author: Option<Author>,
  /// 横傍線部色
  color: Option<Color>,
  /// 部分表示
  fields: Vec<Field>,
  /// フッター
  footer: Option<Footer>,
  /// タイムスタンプ群
  timestamp: Option<OffsetDateTime>,
}

impl Embed {
  /// create a new embed
  ///
  /// # Errors
  /// Returns an error if:
  /// - `title` is longer than 256 bytes.
  pub fn new(title: String) -> Result<Self, DiscordError> {
    // verify
    if title.len() > 256 {
      return Err(DiscordError::TooLongContent {
        actual: title.len(),
        max: 256,
      });
    }

    Ok(Self {
      title,
      ..Default::default()
    })
  }

  /// set description
  ///
  /// # Errors
  /// Returns an error if:
  /// - `description` is longer than 4096 bytes.
  pub fn description(mut self, description: String) -> Result<Self, DiscordError> {
    // verify
    if description.len() > 4096 {
      return Err(DiscordError::TooLongContent {
        actual: description.len(),
        max: 4096,
      });
    }

    self.description = Some(description);
    Ok(self)
  }

  /// set sender_name
  pub fn sender_name(mut self, sender_name: String) -> Self {
    self.sender_name = Some(sender_name);
    self
  }

  /// set sender_icon_url
  pub fn sender_icon_url(mut self, sender_icon_url: Url) -> Self {
    self.sender_icon_url = Some(sender_icon_url);
    self
  }

  /// set tts
  pub fn tts(mut self, tts: bool) -> Self {
    self.tts = Some(tts);
    self
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
  pub fn add_field(mut self, field: Field) -> Self {
    self.fields.push(field);
    self
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

  /// verify
  fn verify(&self) -> Result<(), DiscordError> {
    if self.fields.len() > 25 {
      return Err(DiscordError::TooManyFields {
        actual: self.fields.len(),
        max: 25,
      });
    }

    Ok(())
  }

  /// build
  pub fn build(&self) -> Result<Value, DiscordError> {
    self.verify()?;

    let mut map = Map::new();
    if let Some(sender_name) = &self.sender_name {
      map.insert("username".to_string(), Value::String(sender_name.clone()));
    }
    if let Some(sender_icon_url) = &self.sender_icon_url {
      map.insert(
        "icon_url".to_string(),
        Value::String(sender_icon_url.to_string()),
      );
    }
    if let Some(tts) = &self.tts {
      map.insert("tts".to_string(), Value::Bool(*tts));
    }
    map.insert("title".to_string(), Value::String(self.title.clone()));
    if let Some(description) = &self.description {
      map.insert(
        "description".to_string(),
        Value::String(description.clone()),
      );
    }
    if let Some(author) = &self.author {
      map.insert("author".to_string(), author.build()?);
    }
    if let Some(color) = &self.color {
      map.insert("color".to_string(), Value::Number(color.build().into()));
    }
    for field in &self.fields {
      map.insert("fields".to_string(), field.build()?);
    }
    if let Some(footer) = &self.footer {
      map.insert("footer".to_string(), footer.build()?);
    }
    if let Some(timestamp) = &self.timestamp {
      map.insert(
        "timestamp".to_string(),
        Value::String(timestamp.to_string()),
      );
    }

    Ok(Value::Object(map))
  }
}

/// ファイル付きリッチメッセージ
pub struct EmbedWithFiles {
  /// リッチメッセージ
  embed: Embed,
  /// 画像
  image: Option<Attachment>,
  /// サムネイル
  thumbnail: Option<Attachment>,
}

impl EmbedWithFiles {
  /// create a new embed with files
  pub fn new(embed: Embed) -> Self {
    Self {
      embed,
      image: None,
      thumbnail: None,
    }
  }

  /// set image
  pub fn image(mut self, image: Attachment) -> Self {
    self.image = Some(image);
    self
  }

  /// set thumbnail
  pub fn thumbnail(mut self, thumbnail: Attachment) -> Self {
    self.thumbnail = Some(thumbnail);
    self
  }
}

impl EmbedWithFiles {
  /// build
  ///
  /// - if only image is set, `image-id: 0`.
  /// - if only thumbnail is set, `thumbnail-id: 0`.
  /// - if image & thumbnail are set, `image-id: 0`, `thumbnail-id: 1`.
  pub fn build(&self) -> Result<Value, DiscordError> {
    self.embed.build()?;
    let mut map = Map::new();
    map.insert("embed".to_string(), self.embed.build()?);
    match map["embed"].as_object_mut() {
      Some(arr) => {
        if let Some(image) = &self.image {
          arr.insert(
            "image".to_string(),
            Value::String(image.get_filename().to_string()),
          );
        }
        if let Some(thumbnail) = &self.thumbnail {
          arr.insert(
            "thumbnail".to_string(),
            Value::String(thumbnail.get_filename().to_string()),
          );
        }
      }
      None => return Err(DiscordError::MissingEmbedField),
    }
    let mut attachments = Vec::new();
    let mut id = 0;
    if let Some(image) = &self.image {
      attachments.push(image.build(id)?);
      id += 1;
    }
    if let Some(thumbnail) = &self.thumbnail {
      attachments.push(thumbnail.build(id)?);
    }
    map.insert("attachments".to_string(), Value::Array(attachments));

    Ok(Value::Object(map))
  }

  /// get attachments
  ///
  /// - if image & thumbnail are set, `Vec[0]: image`, `Vec[1]: thumbnail`.
  /// - if only image is set, `Vec[0]: image`.
  /// - if only thumbnail is set, `Vec[0]: thumbnail`.
  pub fn get_attachments(&self) -> Vec<&Attachment> {
    let mut attachments = Vec::new();
    if let Some(image) = &self.image {
      attachments.push(image);
    }
    if let Some(thumbnail) = &self.thumbnail {
      attachments.push(thumbnail);
    }
    attachments
  }
}
