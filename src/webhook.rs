//! 送信メッセージ構造体
use std::time::Duration;

use reqwest::{
  Client, Response, StatusCode,
  header::{HeaderMap, RETRY_AFTER},
  multipart::{Form, Part},
};

use crate::attachment::Attachment;
use crate::error::DiscordError;
use crate::message::Message;

/// Discord Webhook client
#[derive(Clone)]
pub struct Webhook {
  client: Client,
}

impl Webhook {
  /// create a new webhook client
  /// # Arguments
  /// * `connect_timeout` - Connect timeout in seconds(recommended: 10)
  /// * `timeout` - Request timeout in seconds(recommended: 30)
  pub fn new(connect_timeout: u64, timeout: u64) -> Result<Self, DiscordError> {
    let client = Client::builder()
      .user_agent("shiras-rs/0.1.0")
      .connect_timeout(Duration::from_secs(connect_timeout))
      .timeout(Duration::from_secs(timeout))
      .build()
      .map_err(|error| DiscordError::Request(error.to_string()))?;

    Ok(Self { client })
  }

  /// send message to discord
  /// # Arguments
  /// * `webhook_url` - Webhook URL
  /// * `message` - Message to send (message must be built before sending)
  pub async fn send(&self, webhook_url: &str, message: &Message) -> Result<(), DiscordError> {
    if message.get_attachments_count() > 0 {
      // multipart/form-data
      self
        .send_multipart(
          webhook_url,
          message.get_payload_json()?,
          message.get_attachments(),
        )
        .await
    } else {
      // application/json
      self
        .send_json(webhook_url, message.get_payload_json()?)
        .await
    }
  }

  // send message with files to discord
  async fn send_multipart(
    &self,
    webhook_url: &str,
    payload_json: &str,
    attachments: &[Attachment],
  ) -> Result<(), DiscordError> {
    let mut form = Form::new();

    // add payload_json
    form = form.text("payload_json", payload_json.to_string());

    // add attachments
    for (index, attachment) in attachments.iter().enumerate() {
      let filename = attachment.get_filename();
      let data = attachment.get_data();
      let part = Part::stream(data.clone())
        .file_name(filename.to_string())
        .mime_str("application/octet-stream")
        .map_err(|e| DiscordError::Request(e.to_string()))?;
      form = form.part(format!("files[{}]", index), part)
    }

    // send
    let response = self
      .client
      .post(webhook_url)
      .multipart(form)
      .send()
      .await
      .map_err(|error| DiscordError::Request(error.to_string()))?;

    // analyze response
    Webhook::ensure_success(response).await
  }

  // send message with json to discord
  async fn send_json(&self, webhook_url: &str, payload_json: &str) -> Result<(), DiscordError> {
    let response = self
      .client
      .post(webhook_url)
      .header(reqwest::header::CONTENT_TYPE, "application/json")
      .body(payload_json.to_string())
      .send()
      .await
      .map_err(|error| DiscordError::Request(error.to_string()))?;

    // analyze response
    Webhook::ensure_success(response).await
  }

  // analyze response
  async fn ensure_success(response: Response) -> Result<(), DiscordError> {
    let status = response.status();

    // success(2xx)
    if status.is_success() {
      return Ok(());
    }

    let headers = response.headers().clone();
    let body = response
      .text()
      .await
      .map_err(|error| DiscordError::Request(error.to_string()))?;
    let body = if body.is_empty() { None } else { Some(body) };

    // rate limited(429)
    if status == StatusCode::TOO_MANY_REQUESTS {
      let retry_after = Webhook::parse_retry_after(&headers, body.as_deref());
      return Err(DiscordError::RateLimited { retry_after, body });
    }

    // other errors
    Err(DiscordError::Api { status, body })
  }

  // get retry sec after from headers or body
  fn parse_retry_after(headers: &HeaderMap, body: Option<&str>) -> Option<Duration> {
    body
      // body
      .and_then(Webhook::retry_after_from_json)
      // header
      .or_else(|| Webhook::retry_after_from_header(headers))
  }
  // from json body
  // based on discord api response specification
  // "retry_after": sec
  fn retry_after_from_json(body: &str) -> Option<Duration> {
    let value: serde_json::Value = serde_json::from_str(body).ok()?;
    value
      .get("retry_after")
      .and_then(serde_json::Value::as_f64)
      .and_then(Webhook::secs_to_duration)
  }
  // from header
  // based on http header specification
  fn retry_after_from_header(headers: &HeaderMap) -> Option<Duration> {
    headers
      .get(RETRY_AFTER)
      .and_then(|value| value.to_str().ok())
      .and_then(|value| value.parse::<f64>().ok())
      .and_then(Webhook::secs_to_duration)
  }
  // verify seconds
  // 0 <= secs < infinity
  fn secs_to_duration(secs: f64) -> Option<Duration> {
    if secs.is_finite() && secs >= 0.0 {
      Some(Duration::from_secs_f64(secs))
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Message;
  use reqwest::header::{HeaderMap, HeaderValue};

  #[tokio::test]
  async fn not_yet_built_message_should_return_error() {
    let webhook = Webhook::new(5, 5).unwrap();
    let message = Message::new().content("hello").unwrap();
    let error = webhook
      .send("https://example.invalid/webhook", &message)
      .await
      .expect_err("failed to return error when sending not yet built message");
    assert!(matches!(error, DiscordError::NotBuilt));
  }

  #[test]
  fn retry_after_from_json_body() {
    let body = r#"{"message":"rate limited","retry_after":1.5}"#;
    let duration = Webhook::retry_after_from_json(body).expect("retry_after");
    assert_eq!(duration, Duration::from_millis(1500));
  }

  #[test]
  fn parses_retry_after_header() {
    let mut headers = HeaderMap::new();
    headers.insert(RETRY_AFTER, HeaderValue::from_static("2"));
    assert_eq!(
      Webhook::retry_after_from_header(&headers),
      Some(Duration::from_secs(2))
    );
  }
}
