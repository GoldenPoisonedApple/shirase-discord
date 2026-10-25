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
//! async fn example() -> Result<(), shirase::error::DiscordError> {
//!   let webhook = Webhook::new(10, 30)?;
//!   let message = Message::new().content("Hello, world!")?.build()?;
//!   webhook.send("https://discord.com/api/webhooks/...", &message).await?;
//!   Ok(())
//! }
//! ```
//!
//! ## Quick start (Multiple Webhooks)
//!
//! ```no_run
//! use shirase::Webhook;
//! use shirase::Message;
//! use shirase::Embed;
//! use shirase::Color;
//! use shirase::Attachment;
//!
//! #[tokio::main]
//! async fn example() -> Result<(), shirase::error::DiscordError> {
//!   let webhook = Webhook::new(10, 30)?;
//!   let webhook_urls = vec!["https://discord.com/api/webhooks/...", "https://discord.com/api/webhooks/..."];
//!
//!   let message = Message::new().content("Hello, world!")?
//!     .add_embed(
//!       Embed::new()
//!         .title("Hello, world!")?
//!         .description("This is a description")?
//!         .color(Color::RED)
//!     )?
//!     .add_attachment(
//!       Attachment::new("image.png", "@image.png")?
//!     )?.build()?;
//!
//!   let tasks = webhook_urls.into_iter().map(|url| {
//!     let webhook = webhook.clone();
//!     let message = Arc::clone(&message);
//! 
//!     tokio::spawn(async move {
//!       webhook.send(url, &message).await
//!     })
//!   });
//!
//!   let results = join_all(tasks).await;
//!
//!   for result in results {
//!     match result {
//!       Ok(Ok(())) => {}
//!       Ok(Err(e)) => eprintln!("{e:?}"),
//!       Err(e) => eprintln!("task failed: {e}"),
//!     }
//!   }
//! 
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
