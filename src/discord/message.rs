//! 送信メッセージ構造体
use reqwest::{
  Client,
  multipart::{Form, Part},
};
use serde_json::Value;

use crate::discord::attachment::Attachment;
use crate::discord::embed::Embed;
use crate::discord::embed::EmbedWithFiles;
use crate::discord::error::DiscordError;
use crate::discord::plain::Plain;

/// Discord message enum
pub enum DiscordMessage {
  /// Plain message
  Plain(Plain),
  /// Embed message
  Embed(Embed),
  /// Embed message with files
  EmbedWithFiles(EmbedWithFiles),
}

impl DiscordMessage {
  /// send message to discord
  ///
  /// # Example
  /// Returns an error if:
  /// - `webhook_url` is invalid.
  /// - `plain` is invalid.
  /// - `embed` is invalid.
  /// - `embed_with_files` is invalid.
  pub async fn send(self, webhook_url: &str) -> Result<(), DiscordError> {
    match self {
      DiscordMessage::Plain(plain) => DiscordMessage::send_json(webhook_url, plain.build()).await,
      DiscordMessage::Embed(embed) => DiscordMessage::send_json(webhook_url, embed.build()?).await,
      DiscordMessage::EmbedWithFiles(embed_with_files) => {
        DiscordMessage::send_multipart(
          webhook_url,
          embed_with_files.build()?,
          embed_with_files.get_attachments(),
        )
        .await
      }
    }
  }

  // send message to discord
  async fn send_json(webhook_url: &str, value: Value) -> Result<(), DiscordError> {
    // send
    let client = Client::new();
    let response = match client.post(webhook_url).json(&value).send().await {
      Ok(response) => response,
      Err(error) => return Err(DiscordError::Request(error.to_string())),
    };
    // check status
    if !response.status().is_success() {
      return Err(DiscordError::Request(response.status().to_string()));
    }
    Ok(())
  }

  // send message with files to discord
  async fn send_multipart(
    webhook_url: &str,
    value: Value,
    attachments: Vec<&Attachment>,
  ) -> Result<(), DiscordError> {
    let mut form = Form::new();
    // add payload_json
    form = form.text("payload_json", value.to_string());
    // add attachments
    for (index, attachment) in attachments.iter().enumerate() {
      let part = Part::bytes(attachment.get_data().to_vec())
        .file_name(attachment.get_filename().to_string());
      let part = match part.mime_str(attachment.get_content_type()) {
        Ok(part) => part,
        Err(_) => return Err(DiscordError::InvalidContentType("invalid content type")),
      };
      form = form.part(format!("files[{}]", index), part);
    }

    // send
    let client = Client::new();
    let response = match client.post(webhook_url).multipart(form).send().await {
      Ok(response) => response,
      Err(error) => return Err(DiscordError::Request(error.to_string())),
    };
    // check status
    if !response.status().is_success() {
      return Err(DiscordError::Request(response.status().to_string()));
    }

    Ok(())
  }
}
