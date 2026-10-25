//! # shirase
//!
//! Async notification library for Discord and other platforms.
//!
//! ## Quick start
//!
//! ```no_run
//! use shirase::discord::send;
//! use shirase::discord::Message;
//!
//! # async fn example() -> Result<(), shirase::ShiraseError> {
//!   let message = Message::new().content("Hello, world!")?;
//!   send("https://discord.com/api/webhooks/...", message).await?;
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
pub use discord::webhook::send;
