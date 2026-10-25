//! discord error

use reqwest::StatusCode;
use std::time::Duration;
use thiserror::Error;

/// discord error
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum DiscordError {
  #[error("content is empty")]
  EmptyContent,
  #[error("content length is too long: actual: {actual}, max: {max}")]
  TooLongContent { actual: usize, max: usize },
  #[error("file size is too large: actual: {actual}, max: {max}")]
  TooLargeFile { actual: usize, max: usize },
  #[error("invalid extension: {0}")]
  InvalidExtension(String),
  #[error("invalid color: value: {value}, max: {max}")]
  InvalidColor { value: u32, max: u32 },
  #[error("too many fields: actual: {actual}, max: {max}")]
  TooManyFields { actual: usize, max: usize },
  #[error("too many embeds: actual: {actual}, max: {max}")]
  TooManyEmbeds { actual: usize, max: usize },
  #[error("missing embed field")]
  MissingEmbedField,
  #[error("duplicate attachment filename: {0}")]
  DuplicateAttachmentFilename(String),
  #[error("a file referenced by the attachment url does not exist: {0}")]
  AttachmentUrlNotFound(String),
  #[error("invalid url: {0}")]
  InvalidUrl(String),
  #[error("failed to serialize message")]
  Serialize(#[from] serde_json::Error),
  #[error("message is not built")]
  NotBuilt,

  #[error("request failed: {0}")]
  Request(String),
  #[error("rate limited (retry_after: {retry_after:?})")]
  RateLimited {
    retry_after: Option<Duration>,
    body: Option<String>,
  },
  #[error("discord api error: status {status}")]
  Api {
    status: StatusCode,
    body: Option<String>,
  },
}
