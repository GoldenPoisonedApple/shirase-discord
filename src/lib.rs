//! # shirase
//!
//! Async notification library for Discord.
//!
//! ## Quick start
//!
//! ```no_run
//! use shirase::Webhook;
//! use shirase::Message;
//!
//! #[tokio::main]
//! async fn example() -> Result<(), shirase::error::DiscordError> {
//!   let webhook = Webhook::new(10, 30)?;
//!   let message = Message::new().content("Hello, world!")?.build()?;
//!   webhook.send("https://discord.com/api/webhooks/...", &message).await?;
//!   Ok(())
//! }
//! ```

// #!: crate全体で有効
// deny: lintエラーをコンパイルエラーとして扱う
// missing_docs: ドキュメントが不足している箇所をコンパイルエラーとして扱う: /// や //! がない
#![deny(missing_docs)]

pub mod attachment;
pub mod embed;
pub mod error;
pub mod message;
pub mod webhook;

pub use attachment::Attachment;
pub use embed::{
  Embed, author::Author, color::Color, field::Field, footer::Footer, image_url::ImageUrl,
};
pub use error::DiscordError;
pub use message::Message;
pub use webhook::Webhook;
