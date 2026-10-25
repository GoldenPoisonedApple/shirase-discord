//! 送信メッセージ構造体
use reqwest::{
  Client, Response,
  multipart::{Form, Part},
};

use crate::discord::attachment::Attachment;
use crate::discord::error::DiscordError;
use crate::discord::message::Message;

/// send message to discord
pub async fn send(webhook_url: &str, message: Message) -> Result<Response, DiscordError> {
  message.verify()?;

  // create payload
  let payload_json = serde_json::to_string(&message)?;

  if message.get_attachments_count() > 0 {
    // multipart/form-data
    send_multipart(webhook_url, payload_json, message.take_attachments()).await
  } else {
    // application/json
    send_json(webhook_url, payload_json).await
  }
}

// send message with files to discord
async fn send_multipart(
  webhook_url: &str,
  payload_json: String,
  attachments: Vec<Attachment>,
) -> Result<Response, DiscordError> {
  let mut form = Form::new();

  // add payload_json
  form = form.text("payload_json", payload_json);

  // add attachments
  for (index, attachment) in attachments.into_iter().enumerate() {
    let (filename, data) = attachment.into_multipart_parts();
    let part = Part::bytes(data)
      .file_name(filename)
      .mime_str("application/octet-stream")
      .map_err(|e| DiscordError::Request(e.to_string()))?;
    form = form.part(format!("files[{}]", index), part)
  }

  // send
  let client = Client::new();
  let response = match client.post(webhook_url).multipart(form).send().await {
    Ok(response) => response,
    Err(error) => return Err(DiscordError::Request(error.to_string())),
  };

  Ok(response)
}

// send message with json to discord
async fn send_json(webhook_url: &str, payload_json: String) -> Result<Response, DiscordError> {
  let client = Client::new();
  let response = match client.post(webhook_url).json(&payload_json).send().await {
    Ok(response) => response,
    Err(error) => return Err(DiscordError::Request(error.to_string())),
  };
  Ok(response)
}
