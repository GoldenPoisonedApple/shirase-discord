//! # shirase
//!
//! Async notification library for Discord and other platforms.
//!
//! ## Quick start
//!
//! ```no_run
//! use shirase::{DiscordWebhook, Message, Notifier, NotifierExt};
//!
//! # async fn example() -> Result<(), shirase::ShiraseError> {
//!   let notifier = DiscordWebhook::new("https://discord.com/api/webhooks/...")?;
//!   notifier.send_text("Deploy complete").await?;
//!
//!   let message = Message::builder()
//!     .content("Deploy complete")
//!     .username("CI Bot")
//!     .build()?;
//!   notifier.send(&message).await?;
//! #  Ok(())
//! # }
//! ```

// #!: crate全体で有効
// deny: lintエラーをコンパイルエラーとして扱う
// missing_docs: ドキュメントが不足している箇所をコンパイルエラーとして扱う: /// や //! がない
#![deny(missing_docs)]

// cfg: 条件付きコンパイル
#[cfg(feature = "discord")]
pub mod discord;

#[cfg(feature = "discord")]
pub use discord::message::DiscordMessage;
