//! 送信メッセージ構造体
use std::time::Duration;

use reqwest::{
  StatusCode,
  header::{HeaderMap, RETRY_AFTER},
};

use crate::attachment::Attachment;
use crate::error::DiscordError;
use crate::message::Message;

// shared by the async and blocking clients
const USER_AGENT: &str = concat!("shirase-discord/", env!("CARGO_PKG_VERSION"));

/// Asynchronous Discord webhook client.
///
/// This client is available with the `async` feature. Enable the `async` feature to use this client.
///
/// This crate does not bundle a runtime. You need to use a runtime such as tokio.
///
/// # Examples
///
/// ```no_run
/// use shirase_discord::Message;
/// use shirase_discord::AsyncWebhook;
///
/// # async fn example() -> Result<(), shirase_discord::DiscordError> {
/// let webhook = AsyncWebhook::new(10, 30)?;
/// let message = Message::new().content("Hello, world!")?.build()?;
/// webhook.send("https://discord.com/api/webhooks/...", &message).await?;
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct AsyncWebhook {
  client: reqwest::Client,
}

#[cfg(feature = "async")]
impl AsyncWebhook {
  /// create a new webhook client
  /// # Arguments
  /// * `connect_timeout` - Connect timeout in seconds(recommended: 10)
  /// * `timeout` - Request timeout in seconds(recommended: 30)
  pub fn new(connect_timeout: u64, timeout: u64) -> Result<Self, DiscordError> {
    let client = reqwest::Client::builder()
      .user_agent(USER_AGENT)
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
    let mut form = reqwest::multipart::Form::new();

    // add payload_json
    form = form.text("payload_json", payload_json.to_string());

    // add attachments
    for (index, attachment) in attachments.iter().enumerate() {
      let filename = attachment.get_filename();
      let data = attachment.get_data();
      let part = reqwest::multipart::Part::stream(data.clone())
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
    AsyncWebhook::ensure_success(response).await
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
    AsyncWebhook::ensure_success(response).await
  }

  // analyze response
  async fn ensure_success(response: reqwest::Response) -> Result<(), DiscordError> {
    let status = response.status();
    let headers = response.headers().clone();
    let body = response
      .text()
      .await
      .map_err(|error| DiscordError::Request(error.to_string()))?;

    classify_response(status, &headers, body)
  }
}

/// Synchronous Discord webhook client.
///
/// This client is available with the `blocking` feature. This feature is enabled by default.
///
/// # Examples
///
/// ```no_run
/// use shirase_discord::Webhook;
/// use shirase_discord::Message;
///
/// fn main() -> Result<(), shirase_discord::DiscordError> {
///   let webhook = Webhook::new(10, 30)?;
///   let message = Message::new().content("Hello, world!")?.build()?;
///   webhook.send("https://discord.com/api/webhooks/...", &message)?;
///   Ok(())
/// }
/// ```
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct Webhook {
  client: reqwest::blocking::Client,
}

#[cfg(feature = "blocking")]
impl Webhook {
  /// create a new webhook client
  /// # Arguments
  /// * `connect_timeout` - Connect timeout in seconds(recommended: 10)
  /// * `timeout` - Request timeout in seconds(recommended: 30)
  ///
  /// # Panics
  ///
  /// Panics when called from within an asynchronous runtime.
  pub fn new(connect_timeout: u64, timeout: u64) -> Result<Self, DiscordError> {
    let client = reqwest::blocking::Client::builder()
      .user_agent(USER_AGENT)
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
  pub fn send(&self, webhook_url: &str, message: &Message) -> Result<(), DiscordError> {
    if message.get_attachments_count() > 0 {
      // multipart/form-data
      self.send_multipart(
        webhook_url,
        message.get_payload_json()?,
        message.get_attachments(),
      )
    } else {
      // application/json
      self.send_json(webhook_url, message.get_payload_json()?)
    }
  }

  // send message with files to discord
  fn send_multipart(
    &self,
    webhook_url: &str,
    payload_json: &str,
    attachments: &[Attachment],
  ) -> Result<(), DiscordError> {
    let mut form = reqwest::blocking::multipart::Form::new();

    // add payload_json
    form = form.text("payload_json", payload_json.to_string());

    // add attachments
    for (index, attachment) in attachments.iter().enumerate() {
      let filename = attachment.get_filename();
      let data = attachment.get_data().clone();
      let length = data.len() as u64;
      // Bytes の clone は参照カウントだけ増える。Part::bytes は所有バッファを要求する
      let part =
        reqwest::blocking::multipart::Part::reader_with_length(std::io::Cursor::new(data), length)
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
      .map_err(|error| DiscordError::Request(error.to_string()))?;

    // analyze response
    Webhook::ensure_success(response)
  }

  // send message with json to discord
  fn send_json(&self, webhook_url: &str, payload_json: &str) -> Result<(), DiscordError> {
    let response = self
      .client
      .post(webhook_url)
      .header(reqwest::header::CONTENT_TYPE, "application/json")
      .body(payload_json.to_string())
      .send()
      .map_err(|error| DiscordError::Request(error.to_string()))?;

    // analyze response
    Webhook::ensure_success(response)
  }

  // analyze response
  fn ensure_success(response: reqwest::blocking::Response) -> Result<(), DiscordError> {
    let status = response.status();
    let headers = response.headers().clone();
    let body = response
      .text()
      .map_err(|error| DiscordError::Request(error.to_string()))?;

    classify_response(status, &headers, body)
  }
}

// analyze response
fn classify_response(
  status: StatusCode,
  headers: &HeaderMap,
  body: String,
) -> Result<(), DiscordError> {
  // success(2xx)
  if status.is_success() {
    return Ok(());
  }

  let body = if body.is_empty() { None } else { Some(body) };

  // rate limited(429)
  if status == StatusCode::TOO_MANY_REQUESTS {
    let retry_after = parse_retry_after(headers, body.as_deref());
    return Err(DiscordError::RateLimited { retry_after, body });
  }

  // other errors
  Err(DiscordError::Api { status, body })
}

// get retry sec after from headers or body
fn parse_retry_after(headers: &HeaderMap, body: Option<&str>) -> Option<Duration> {
  body
    // body
    .and_then(retry_after_from_json)
    // header
    .or_else(|| retry_after_from_header(headers))
}
// from json body
// based on discord api response specification
// "retry_after": sec
fn retry_after_from_json(body: &str) -> Option<Duration> {
  let value: serde_json::Value = serde_json::from_str(body).ok()?;
  value
    .get("retry_after")
    .and_then(serde_json::Value::as_f64)
    .and_then(secs_to_duration)
}
// from header
// based on http header specification
fn retry_after_from_header(headers: &HeaderMap) -> Option<Duration> {
  headers
    .get(RETRY_AFTER)
    .and_then(|value| value.to_str().ok())
    .and_then(|value| value.parse::<f64>().ok())
    .and_then(secs_to_duration)
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Message;
  use reqwest::header::{HeaderMap, HeaderValue};

  #[cfg(feature = "async")]
  #[tokio::test]
  async fn not_yet_built_message_should_return_error() {
    let webhook = AsyncWebhook::new(5, 5).unwrap();
    let message = Message::new().content("hello").unwrap();
    let error = webhook
      .send("https://example.invalid/webhook", &message)
      .await
      .expect_err("failed to return error when sending not yet built message");
    assert!(matches!(error, DiscordError::NotBuilt));
  }

  #[cfg(feature = "blocking")]
  #[test]
  fn blocking_not_yet_built_message_should_return_error() {
    let webhook = Webhook::new(5, 5).unwrap();
    let message = Message::new().content("hello").unwrap();
    let error = webhook
      .send("https://example.invalid/webhook", &message)
      .expect_err("failed to return error when sending not yet built message");
    assert!(matches!(error, DiscordError::NotBuilt));
  }

  #[test]
  fn retry_after_from_json_body() {
    let body = r#"{"message":"rate limited","retry_after":1.5}"#;
    let duration = retry_after_from_json(body).expect("retry_after");
    assert_eq!(duration, Duration::from_millis(1500));
  }

  #[test]
  fn parses_retry_after_header() {
    let mut headers = HeaderMap::new();
    headers.insert(RETRY_AFTER, HeaderValue::from_static("2"));
    assert_eq!(
      retry_after_from_header(&headers),
      Some(Duration::from_secs(2))
    );
  }
}
