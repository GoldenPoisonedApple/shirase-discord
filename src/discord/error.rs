//! discord error

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
  #[error("invalid content type: {0}")]
  InvalidContentType(&'static str),
  #[error("too many fields: actual: {actual}, max: {max}")]
  TooManyFields { actual: usize, max: usize },
  #[error("missing embed field")]
  MissingEmbedField,
  #[error("request failed: {0}")]
  Request(String),
}
