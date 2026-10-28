//! # shirase_discord
//!
//! Webhook notification library for Discord.
//!
//! The default client is synchronous `Webhook`.
//! If you want to use asynchronous client, enable the `async` feature. And use `AsyncWebhook`.
//!
//! ## Quick start
//!
//! ```no_run
//! # #[cfg(feature = "blocking")]
//! # {
//! use shirase_discord::Webhook;
//! use shirase_discord::Message;
//!
//! fn example() -> Result<(), shirase_discord::DiscordError> {
//!   let webhook = Webhook::new(10, 30)?;
//!   let message = Message::new().content("Hello, world!")?.build()?;
//!   webhook.send("https://discord.com/api/webhooks/...", &message)?;
//!   Ok(())
//! }
//! # }
//! ```

// #!: crate全体で有効
// deny: lintエラーをコンパイルエラーとして扱う
// missing_docs: ドキュメントが不足している箇所をコンパイルエラーとして扱う: /// や //! がない
#![deny(missing_docs)]

// いずれかの feature を有効にしていない場合はコンパイルエラー
#[cfg(not(any(feature = "blocking", feature = "async")))]
compile_error!("enable the feature `blocking`, `async`, or both");

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
#[cfg(feature = "async")]
pub use webhook::AsyncWebhook;
#[cfg(feature = "blocking")]
pub use webhook::Webhook;
